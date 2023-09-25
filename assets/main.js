import wasm_bindgen_init, { luminol_start } from '/luminol.js';
import { setLuminol } from '/luminol-singleton.js';

// Load Luminol's WASM code and initialize wasm-bindgen.
const luminol = await wasm_bindgen_init('/luminol_bg.wasm');

// Save the handle to the WASM code so we can access it from elsewhere.
setLuminol(luminol);

// Initialize `__asyncify_data`, located at stack offsets 16 through 23,
// to the start (inclusive) and end (exclusive) offsets of the part of the stack
// we allocate for Asyncify.
new Int32Array(luminol.memory.buffer, 16).set([24, 1024]);

// Start the program. Non-blocking.
luminol_start();
