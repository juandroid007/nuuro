// Copyright 2020-2020 Juan Villacorta
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This module contains methods exported to the nuuro javascript code for WebAssembly.
//! DO NOT USE DIRECTLY!

#![allow(non_snake_case)]

use std::os::raw::{c_char, c_int, c_void};

use super::{app_runner_borrow, app_runner_borrow_mut, app_runner_is_defined};
use crate::input::KeyCode;
use crate::renderer::shaders;

pub fn nuuroWasmInit() {
    app_runner_borrow_mut().init();
}

pub fn nuuroWasmOnResize(w: c_int, h: c_int) {
    app_runner_borrow_mut().resize((w as u32, h as u32));
}

pub fn nuuroWasmUpdateAndDraw(time_millis: f64, cursor_x: c_int, cursor_y: c_int) -> c_int {
    app_runner_borrow_mut().update_cursor(cursor_x as i32, cursor_y as i32);
    let continuing = app_runner_borrow_mut().update_and_draw(time_millis / 1000.0);
    if continuing {
        1
    } else {
        0
    }
}

pub fn nuuroWasmKeyEvent(code: c_int, down: bool) -> c_int {
    assert!(code >= 0 && code <= 255);
    let code = KeyCode::from_u8(code as u8).unwrap();
    let continuing = app_runner_borrow_mut().input(code, down);
    if continuing {
        1
    } else {
        0
    }
}

pub fn nuuroWasmMouseEvent(cursor_x: c_int, cursor_y: c_int, button: c_int, down: bool) -> c_int {
    app_runner_borrow_mut().update_cursor(cursor_x as i32, cursor_y as i32);
    let code = match button {
        0 => Some(KeyCode::MouseLeft),
        1 => Some(KeyCode::MouseMiddle),
        2 => Some(KeyCode::MouseRight),
        _ => None,
    };
    let continuing = if let Some(code) = code {
        app_runner_borrow_mut().input(code, down)
    } else {
        true
    };
    if continuing {
        1
    } else {
        0
    }
}

pub fn nuuroWasmIsAppDefined() -> c_int {
    if app_runner_is_defined() {
        1
    } else {
        0
    }
}

pub fn nuuroWasmMusicCount() -> c_int {
    app_runner_borrow().music_count() as c_int
}

pub fn nuuroWasmSoundCount() -> c_int {
    app_runner_borrow().sound_count() as c_int
}

pub fn nuuroWasmSpriteVertSrc() -> *const c_char {
    shaders::VS_SPRITE_SRC
}

pub fn nuuroWasmSpriteFragSrc() -> *const c_char {
    shaders::FS_SPRITE_SRC
}

pub fn nuuroWasmOnRestart() {
    app_runner_borrow_mut().on_restart();
}

pub fn nuuroWasmCookieDataPtr(size: usize) -> *mut c_void {
    app_runner_borrow_mut().cookie_buffer(size).as_mut_ptr() as *mut c_void
}

/// Macro to be placed in the `main.rs` file for a Nuuro app.
///
/// Currently, the only use this macro has is to export WASM functions for the app
/// when compiling to the `wasm32-unknown-unknown` target.
#[macro_export]
macro_rules! nuuro_header {
    () => {
        pub mod nuuro_wasm_exports {
            use std::os::raw::{c_char, c_int, c_void};
            #[no_mangle]
            pub unsafe extern "C" fn nuuroWasmInit() {
                ::nuuro::wasm_exports::nuuroWasmInit()
            }
            #[no_mangle]
            pub unsafe extern "C" fn nuuroWasmOnResize(w: c_int, h: c_int) {
                ::nuuro::wasm_exports::nuuroWasmOnResize(w, h)
            }
            #[no_mangle]
            pub unsafe extern "C" fn nuuroWasmUpdateAndDraw(
                time_millis: f64,
                cursor_x: c_int,
                cursor_y: c_int,
            ) -> c_int {
                ::nuuro::wasm_exports::nuuroWasmUpdateAndDraw(time_millis, cursor_x, cursor_y)
            }
            #[no_mangle]
            pub unsafe extern "C" fn nuuroWasmKeyEvent(code: c_int, down: bool) -> c_int {
                ::nuuro::wasm_exports::nuuroWasmKeyEvent(code, down)
            }
            #[no_mangle]
            pub unsafe extern "C" fn nuuroWasmMouseEvent(
                cursor_x: c_int,
                cursor_y: c_int,
                button: c_int,
                down: bool,
            ) -> c_int {
                ::nuuro::wasm_exports::nuuroWasmMouseEvent(cursor_x, cursor_y, button, down)
            }
            #[no_mangle]
            pub unsafe extern "C" fn nuuroWasmIsAppDefined() -> c_int {
                ::nuuro::wasm_exports::nuuroWasmIsAppDefined()
            }
            #[no_mangle]
            pub unsafe extern "C" fn nuuroWasmMusicCount() -> c_int {
                ::nuuro::wasm_exports::nuuroWasmMusicCount()
            }
            #[no_mangle]
            pub unsafe extern "C" fn nuuroWasmSoundCount() -> c_int {
                ::nuuro::wasm_exports::nuuroWasmSoundCount()
            }
            #[no_mangle]
            pub unsafe extern "C" fn nuuroWasmSpriteVertSrc() -> *const c_char {
                ::nuuro::wasm_exports::nuuroWasmSpriteVertSrc()
            }
            #[no_mangle]
            pub unsafe extern "C" fn nuuroWasmSpriteFragSrc() -> *const c_char {
                ::nuuro::wasm_exports::nuuroWasmSpriteFragSrc()
            }
            #[no_mangle]
            pub unsafe extern "C" fn nuuroWasmOnRestart() {
                ::nuuro::wasm_exports::nuuroWasmOnRestart()
            }
            #[no_mangle]
            pub unsafe extern "C" fn nuuroWasmCookieDataPtr(size: usize) -> *mut c_void {
                ::nuuro::wasm_exports::nuuroWasmCookieDataPtr(size)
            }
        }
    };
}
