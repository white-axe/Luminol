// Copyright (C) 2023 Lily Lyons
//
// This file is part of Luminol.
//
// Luminol is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Luminol is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Luminol.  If not, see <http://www.gnu.org/licenses/>.
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(module = "/assets/asyncify-callback.js")]
extern "C" {
    // If you decide to change the name (Rust name or js_name) of this function,
    // don't forget to update the symbol name in asyncify.sh and asyncify.sh.cmd.
    // The symbol name can be found by running wasm-dis from Binaryen on the
    // compiled WASM file and searching for the js_name of the function in the
    // output.
    #[wasm_bindgen(js_name = luminolInterrupt)]
    pub fn luminol_interrupt() -> JsValue;
}
