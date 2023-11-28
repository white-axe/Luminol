use egui::TexturesDelta;
use wasm_bindgen::JsValue;

use crate::{epi, App};

use super::{now_sec, web_painter::WebPainter, NeedRepaint};

pub struct AppRunner {
    web_options: crate::WebOptions,
    pub(crate) frame: epi::Frame,
    egui_ctx: egui::Context,
    pub(crate) painter: super::ActiveWebPainter,
    pub(crate) input: super::WebInput,
    app: Box<dyn epi::App>,
    pub(crate) needs_repaint: std::sync::Arc<NeedRepaint>,
    last_save_time: f64,
    pub(crate) text_cursor_pos: Option<egui::Pos2>,
    pub(crate) mutable_text_under_cursor: bool,
    textures_delta: TexturesDelta,

    pub(super) canvas: web_sys::OffscreenCanvas,
    pub(super) worker_options: super::WorkerOptions,
}

impl Drop for AppRunner {
    fn drop(&mut self) {
        log::debug!("AppRunner has fully dropped");
    }
}

impl AppRunner {
    /// # Errors
    /// Failure to initialize WebGL renderer.
    pub async fn new(
        canvas: web_sys::OffscreenCanvas,
        web_options: crate::WebOptions,
        app_creator: epi::AppCreator,
        worker_options: super::WorkerOptions,
    ) -> Result<Self, String> {
        let Some(worker) = luminol_web::bindings::worker() else {
            panic!("cannot create a web runner outside of a web worker");
        };
        let location = worker.location();
        let user_agent = worker.navigator().user_agent().unwrap_or_default();

        let painter = super::ActiveWebPainter::new(canvas.clone(), &web_options).await?;

        let system_theme = if web_options.follow_system_theme {
            worker_options.prefers_color_scheme_dark.map(|x| {
                if x {
                    crate::Theme::Dark
                } else {
                    crate::Theme::Light
                }
            })
        } else {
            None
        };

        let info = epi::IntegrationInfo {
            web_info: epi::WebInfo {
                user_agent: user_agent.clone(),
                location: crate::Location {
                    url: location
                        .href()
                        .strip_suffix("/worker.js")
                        .unwrap_or(location.href().as_str())
                        .to_string(),
                    protocol: location.protocol(),
                    host: location.host(),
                    hostname: location.hostname(),
                    port: location.port(),
                    hash: Default::default(),
                    query: Default::default(),
                    query_map: Default::default(),
                    origin: location.origin(),
                },
            },
            system_theme,
            cpu_usage: None,
            native_pixels_per_point: Some(1.),
        };
        let storage = LocalStorage {
            channels: worker_options.channels.clone(),
        };

        let egui_ctx = egui::Context::default();
        egui_ctx.set_os(egui::os::OperatingSystem::from_user_agent(&user_agent));
        super::storage::load_memory(&egui_ctx, &worker_options.channels).await;

        let theme = system_theme.unwrap_or(web_options.default_theme);
        egui_ctx.set_visuals(theme.egui_visuals());

        let app = app_creator(&epi::CreationContext {
            egui_ctx: egui_ctx.clone(),
            integration_info: info.clone(),
            storage: Some(&storage),

            #[cfg(feature = "glow")]
            gl: Some(painter.gl().clone()),

            #[cfg(all(feature = "wgpu", not(feature = "glow")))]
            wgpu_render_state: painter.render_state(),
            #[cfg(all(feature = "wgpu", feature = "glow"))]
            wgpu_render_state: None,
        });

        let frame = epi::Frame {
            info,
            output: Default::default(),
            storage: Some(Box::new(storage)),

            #[cfg(feature = "glow")]
            gl: Some(painter.gl().clone()),

            #[cfg(all(feature = "wgpu", not(feature = "glow")))]
            wgpu_render_state: painter.render_state(),
            #[cfg(all(feature = "wgpu", feature = "glow"))]
            wgpu_render_state: None,
        };

        let needs_repaint: std::sync::Arc<NeedRepaint> = Default::default();
        {
            let needs_repaint = needs_repaint.clone();
            egui_ctx.set_request_repaint_callback(move |info| {
                needs_repaint.repaint_after(info.after.as_secs_f64());
            });
        }

        let mut runner = Self {
            web_options,
            frame,
            egui_ctx,
            painter,
            input: Default::default(),
            app,
            needs_repaint,
            last_save_time: now_sec(),
            text_cursor_pos: None,
            mutable_text_under_cursor: false,
            textures_delta: Default::default(),

            worker_options,
            canvas,
        };

        runner.input.raw.max_texture_side = Some(runner.painter.max_texture_side());

        Ok(runner)
    }

    pub fn egui_ctx(&self) -> &egui::Context {
        &self.egui_ctx
    }

    /// Get mutable access to the concrete [`App`] we enclose.
    ///
    /// This will panic if your app does not implement [`App::as_any_mut`].
    pub fn app_mut<ConcreteApp: 'static + App>(&mut self) -> &mut ConcreteApp {
        self.app
            .as_any_mut()
            .expect("Your app must implement `as_any_mut`, but it doesn't")
            .downcast_mut::<ConcreteApp>()
            .expect("app_mut got the wrong type of App")
    }

    pub fn auto_save_if_needed(&mut self) {
        let time_since_last_save = now_sec() - self.last_save_time;
        if time_since_last_save > self.app.auto_save_interval().as_secs_f64() {
            self.save();
        }
    }

    pub fn save(&mut self) {
        if self.app.persist_egui_memory() {
            super::storage::save_memory(&self.egui_ctx, &self.worker_options.channels);
        }
        if let Some(storage) = self.frame.storage_mut() {
            self.app.save(storage);
        }
        self.last_save_time = now_sec();
    }

    pub fn warm_up(&mut self) {
        if self.app.warm_up_enabled() {
            let saved_memory: egui::Memory = self.egui_ctx.memory(|m| m.clone());
            self.egui_ctx
                .memory_mut(|m| m.set_everything_is_visible(true));
            self.logic();
            self.egui_ctx.memory_mut(|m| *m = saved_memory); // We don't want to remember that windows were huge.
            self.egui_ctx.clear_animations();
        }
    }

    pub fn destroy(mut self) {
        log::debug!("Destroying AppRunner");
        self.painter.destroy();
    }

    /// Returns how long to wait until the next repaint.
    ///
    /// Call [`Self::paint`] later to paint
    pub fn logic(&mut self) -> (std::time::Duration, Vec<egui::ClippedPrimitive>) {
        let frame_start = now_sec();

        let raw_input = self.input.new_frame(
            egui::vec2(self.painter.width as f32, self.painter.height as f32),
            self.painter.pixel_ratio,
        );

        let full_output = self.egui_ctx.run(raw_input, |egui_ctx| {
            self.app.update(egui_ctx, &mut self.frame);
        });
        let egui::FullOutput {
            platform_output,
            repaint_after,
            textures_delta,
            shapes,
        } = full_output;

        self.mutable_text_under_cursor = platform_output.mutable_text_under_cursor;
        self.worker_options
            .channels
            .send(super::WebRunnerOutputInner::PlatformOutput(
                platform_output,
                self.egui_ctx.options(|o| o.screen_reader),
                self.egui_ctx.wants_keyboard_input(),
            ));
        self.textures_delta.append(textures_delta);
        let clipped_primitives = self.egui_ctx.tessellate(shapes);

        {
            let app_output = self.frame.take_app_output();
            let epi::backend::AppOutput {} = app_output;
        }

        self.frame.info.cpu_usage = Some((now_sec() - frame_start) as f32);

        (repaint_after, clipped_primitives)
    }

    /// Paint the results of the last call to [`Self::logic`].
    pub fn paint(&mut self, clipped_primitives: &[egui::ClippedPrimitive]) -> Result<(), JsValue> {
        let textures_delta = std::mem::take(&mut self.textures_delta);

        self.painter.paint_and_update_textures(
            self.app.clear_color(&self.egui_ctx.style().visuals),
            clipped_primitives,
            self.egui_ctx.pixels_per_point(),
            &textures_delta,
        )?;

        Ok(())
    }

    pub(super) fn handle_platform_output(
        state: &super::MainState,
        platform_output: egui::PlatformOutput,
        screen_reader_enabled: bool,
        wants_keyboard_input: bool,
    ) {
        if screen_reader_enabled {
            if let Ok(mut inner) = state.inner.try_borrow_mut() {
                if let Some(screen_reader) = &mut inner.screen_reader {
                    screen_reader.speak(&platform_output.events_description());
                }
            }
        }

        let egui::PlatformOutput {
            cursor_icon,
            open_url,
            copied_text,
            events: _, // already handled
            mutable_text_under_cursor,
            text_cursor_pos,
            #[cfg(feature = "accesskit")]
                accesskit_update: _, // not currently implemented
        } = platform_output;

        super::set_cursor_icon(cursor_icon);
        if let Some(open) = open_url {
            super::open_url(&open.url, open.new_tab);
        }

        #[cfg(web_sys_unstable_apis)]
        if !copied_text.is_empty() {
            super::set_clipboard_text(&copied_text);
        }

        #[cfg(not(web_sys_unstable_apis))]
        let _ = copied_text;

        if let Ok(mut inner) = state.inner.try_borrow_mut() {
            inner.mutable_text_under_cursor = mutable_text_under_cursor;
            inner.wants_keyboard_input = wants_keyboard_input;

            if inner.text_cursor_pos != text_cursor_pos {
                super::text_agent::move_text_cursor(text_cursor_pos, &state.canvas);
                inner.text_cursor_pos = text_cursor_pos;
            }
        }
    }
}

// ----------------------------------------------------------------------------

struct LocalStorage {
    channels: super::WorkerChannels,
}

impl epi::Storage for LocalStorage {
    fn get_string(&self, key: &str) -> Option<String> {
        let (oneshot_tx, oneshot_rx) = oneshot::channel();
        self.channels.send(super::WebRunnerOutputInner::StorageGet(
            key.to_string(),
            oneshot_tx,
        ));
        oneshot_rx.recv().ok().flatten()
    }

    fn set_string(&mut self, key: &str, value: String) {
        let (oneshot_tx, oneshot_rx) = oneshot::channel();
        self.channels.send(super::WebRunnerOutputInner::StorageSet(
            key.to_string(),
            value,
            oneshot_tx,
        ));
        let _ = oneshot_rx.recv();
    }

    fn flush(&mut self) {}
}
