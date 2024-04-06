use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use wasm_bindgen::prelude::*;

use crate::{epi, App};

use super::{events, AppRunner, PanicHandler};

/// This is how `eframe` runs your wepp application
///
/// This is cheap to clone.
///
/// See [the crate level docs](crate) for an example.
#[derive(Clone)]
pub struct WebRunner {
    /// Have we ever panicked?
    panic_handler: PanicHandler,

    /// If we ever panic during running, this RefCell is poisoned.
    /// So before we use it, we need to check [`Self::panic_handler`].
    runner: Rc<RefCell<Option<AppRunner>>>,

    /// In case of a panic, unsubscribe these.
    /// They have to be in a separate `Rc` so that we don't need to pass them to
    /// the panic handler, since they aren't `Send`.
    events_to_unsubscribe: Rc<RefCell<Vec<EventToUnsubscribe>>>,

    /// Used in `destroy` to cancel a pending frame.
    request_animation_frame_id: Cell<Option<i32>>,
}

impl WebRunner {
    /// Will install a panic handler that will catch and log any panics
    #[allow(clippy::new_without_default)]
    pub fn new(panic_tx: std::sync::Arc<parking_lot::Mutex<Option<oneshot::Sender<()>>>>) -> Self {
        #[cfg(not(web_sys_unstable_apis))]
        log::warn!(
            "eframe compiled without RUSTFLAGS='--cfg=web_sys_unstable_apis'. Copying text won't work."
        );

        let panic_handler = PanicHandler::install(panic_tx);

        Self {
            panic_handler,
            runner: Rc::new(RefCell::new(None)),
            events_to_unsubscribe: Rc::new(RefCell::new(Default::default())),
            request_animation_frame_id: Cell::new(None),
        }
    }

    /// Set up the event listeners on the main thread in order to do things like respond to
    /// mouse events and resize the canvas to fill the screen.
    pub fn setup_main_thread_hooks(
        state: super::MainState,
    ) -> Result<std::sync::Arc<parking_lot::Mutex<Option<oneshot::Sender<()>>>>, JsValue> {
        let (panic_tx, panic_rx) = oneshot::channel();

        wasm_bindgen_futures::spawn_local(async move {
            let _ = panic_rx.await;
            super::EVENTS_TO_UNSUBSCRIBE.with_borrow_mut(|events| {
                for event in events.drain(..) {
                    if let Err(e) = event.unsubscribe() {
                        log::warn!(
                            "Failed to unsubscribe from event: {}",
                            super::string_from_js_value(&e),
                        );
                    }
                }
            });
        });

        {
            events::install_canvas_events(&state)?;
            events::install_document_events(&state)?;
            events::install_window_events(&state)?;
            super::text_agent::install_text_agent(&state)?;
        }

        wasm_bindgen_futures::spawn_local(async move {
            loop {
                let Ok(command) = state.channels.output_rx.recv_async().await else {
                    log::warn!(
                        "Web runner main thread loop is stopping! This is not supposed to happen."
                    );
                    return;
                };

                match command {
                    super::WebRunnerOutput::PlatformOutput(
                        output,
                        screen_reader_enabled,
                        wants_keyboard_input,
                    ) => {
                        AppRunner::handle_platform_output(
                            &state,
                            output,
                            screen_reader_enabled,
                            wants_keyboard_input,
                        );
                    }

                    super::WebRunnerOutput::StorageGet(key, oneshot_tx) => {
                        let _ = oneshot_tx.send(super::storage::storage_get(&key).await.ok());
                    }

                    super::WebRunnerOutput::StorageSet(key, value, oneshot_tx) => {
                        let _ =
                            oneshot_tx.send(super::storage::storage_set(&key, value).await.is_ok());
                    }
                }
            }
        });

        Ok(std::sync::Arc::new(parking_lot::Mutex::new(Some(panic_tx))))
    }

    /// Create the application, install callbacks, and start running the app.
    ///
    /// # Errors
    /// Failing to initialize graphics.
    pub async fn start(
        &self,
        canvas: web_sys::OffscreenCanvas,
        web_options: crate::WebOptions,
        app_creator: epi::AppCreator,
        worker_options: super::WorkerOptions,
    ) -> Result<(), JsValue> {
        self.destroy();

        let runner = AppRunner::new(canvas, web_options, app_creator, worker_options).await?;
        self.runner.replace(Some(runner));

        {
            self.request_animation_frame()?;
        }

        Ok(())
    }

    /// Has there been a panic?
    pub fn has_panicked(&self) -> bool {
        self.panic_handler.has_panicked()
    }

    /// What was the panic message and callstack?
    pub fn panic_summary(&self) -> Option<super::PanicSummary> {
        self.panic_handler.panic_summary()
    }

    fn unsubscribe_from_all_events(&self) {
        let events_to_unsubscribe: Vec<_> =
            std::mem::take(&mut *self.events_to_unsubscribe.borrow_mut());

        if !events_to_unsubscribe.is_empty() {
            log::debug!("Unsubscribing from {} events", events_to_unsubscribe.len());
            for x in events_to_unsubscribe {
                if let Err(err) = x.unsubscribe() {
                    log::warn!(
                        "Failed to unsubscribe from event: {}",
                        super::string_from_js_value(&err)
                    );
                }
            }
        }
    }

    /// Shut down eframe and clean up resources.
    pub fn destroy(&self) {
        self.unsubscribe_from_all_events();

        if let Some(id) = self.request_animation_frame_id.get() {
            let window = web_sys::window().unwrap();
            window.cancel_animation_frame(id).ok();
        }

        if let Some(runner) = self.runner.replace(None) {
            runner.destroy();
        }
    }

    /// Returns `None` if there has been a panic, or if we have been destroyed.
    /// In that case, just return to JS.
    pub(crate) fn try_lock(&self) -> Option<std::cell::RefMut<'_, AppRunner>> {
        if self.panic_handler.has_panicked() {
            // Unsubscribe from all events so that we don't get any more callbacks
            // that will try to access the poisoned runner.
            self.unsubscribe_from_all_events();
            None
        } else {
            let lock = self.runner.try_borrow_mut().ok()?;
            std::cell::RefMut::filter_map(lock, |lock| -> Option<&mut AppRunner> { lock.as_mut() })
                .ok()
        }
    }

    /// Get mutable access to the concrete [`App`] we enclose.
    ///
    /// This will panic if your app does not implement [`App::as_any_mut`],
    /// and return `None` if this  runner has panicked.
    pub fn app_mut<ConcreteApp: 'static + App>(
        &self,
    ) -> Option<std::cell::RefMut<'_, ConcreteApp>> {
        self.try_lock()
            .map(|lock| std::cell::RefMut::map(lock, |runner| runner.app_mut::<ConcreteApp>()))
    }

    /// Convenience function to reduce boilerplate and ensure that all event handlers
    /// are dealt with in the same way.
    ///
    /// All events added with this method will automatically be unsubscribed on panic,
    /// or when [`Self::destroy`] is called.
    pub fn add_event_listener<E: wasm_bindgen::JsCast>(
        &self,
        target: &web_sys::EventTarget,
        event_name: &'static str,
        mut closure: impl FnMut(E, &mut AppRunner) + 'static,
    ) -> Result<(), wasm_bindgen::JsValue> {
        let runner_ref = self.clone();

        // Create a JS closure based on the FnMut provided
        let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            // Only call the wrapped closure if the egui code has not panicked
            if let Some(mut runner_lock) = runner_ref.try_lock() {
                // Cast the event to the expected event type
                let event = event.unchecked_into::<E>();
                closure(event, &mut runner_lock);
            }
        }) as Box<dyn FnMut(web_sys::Event)>);

        // Add the event listener to the target
        target.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())?;

        let handle = TargetEvent {
            target: target.clone(),
            event_name: event_name.to_owned(),
            closure,
        };

        // Remember it so we unsubscribe on panic.
        // Otherwise we get calls into `self.runner` after it has been poisoned by a panic.
        self.events_to_unsubscribe
            .borrow_mut()
            .push(EventToUnsubscribe::TargetEvent(handle));

        Ok(())
    }

    pub(crate) fn request_animation_frame(&self) -> Result<(), wasm_bindgen::JsValue> {
        let worker = luminol_web::bindings::worker().unwrap();
        let closure = Closure::once({
            let runner_ref = self.clone();
            move || events::paint_and_schedule(&runner_ref)
        });
        let id = worker.request_animation_frame(closure.as_ref().unchecked_ref())?;
        self.request_animation_frame_id.set(Some(id));
        closure.forget(); // We must forget it, or else the callback is canceled on drop
        Ok(())
    }
}

// ----------------------------------------------------------------------------

pub(super) struct TargetEvent {
    pub(super) target: web_sys::EventTarget,
    pub(super) event_name: String,
    pub(super) closure: Closure<dyn FnMut(web_sys::Event)>,
}

#[allow(unused)]
pub(super) struct IntervalHandle {
    pub(super) handle: i32,
    pub(super) closure: Closure<dyn FnMut()>,
}

pub(super) enum EventToUnsubscribe {
    TargetEvent(TargetEvent),

    #[allow(unused)]
    IntervalHandle(IntervalHandle),
}

impl EventToUnsubscribe {
    pub fn unsubscribe(self) -> Result<(), JsValue> {
        match self {
            Self::TargetEvent(handle) => {
                handle.target.remove_event_listener_with_callback(
                    handle.event_name.as_str(),
                    handle.closure.as_ref().unchecked_ref(),
                )?;
                Ok(())
            }
            Self::IntervalHandle(handle) => {
                let window = web_sys::window().unwrap();
                window.clear_interval_with_handle(handle.handle);
                Ok(())
            }
        }
    }
}
