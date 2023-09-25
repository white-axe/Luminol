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

import { getLuminol } from '/luminol-singleton.js';

function luminolInterruptCallback() {
    const luminol = getLuminol();
    luminol.asyncify_start_rewind(16);
    luminol.luminol_start();
}

export function luminolInterrupt() {
    const luminol = getLuminol();

    if (typeof luminol.asyncify_get_state !== 'function') {
        throw new Error('`luminol.asyncify_get_state()` not defined. You probably failed to properly post-process the binary with Asyncify.');
    }

    const state = luminol.asyncify_get_state();

    // Run when `luminol_interrupt()` is called from Rust
    if (state === 0) {
        setTimeout(luminolInterruptCallback, 5000);
        luminol.asyncify_start_unwind(16);
        return; // This return value isn't used, we can set it to whatever
    }

    // Run when `luminolInterruptCallback()` defined above finishes executing
    else if (state === 2) {
        luminol.asyncify_stop_rewind();
        return 1337; // This is the actual return value of `luminol_interrupt()`
    }

    else {
        throw new Error('Invalid Asyncify state: ' + state);
    }
}
