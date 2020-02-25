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

//! This module contains methods imported from the nuuro javascript code for WebAssembly.
//! DO NOT USE DIRECTLY!

use std::os::raw::{c_int, c_void};

extern "C" {
    pub fn nuuroWasmSetScissor(x: c_int, y: c_int, w: c_int, h: c_int);

    pub fn nuuroWasmClear(r: f32, g: f32, b: f32);
    pub fn nuuroWasmDrawSprites(size: usize, data: *const c_void);

    pub fn nuuroWasmPlaySound(id: c_int);
    pub fn nuuroWasmPlayMusic(id: c_int);
    pub fn nuuroWasmLoopMusic(id: c_int);
    pub fn nuuroWasmStopMusic();

    pub fn nuuroWasmSpriteAtlasBinSize() -> usize;
    pub fn nuuroWasmSpriteAtlasBinFill(buffer: *mut c_void);
    pub fn nuuroWasmTiledAtlasBinSize() -> usize;
    pub fn nuuroWasmTiledAtlasBinFill(buffer: *mut c_void);

    pub fn nuuroWasmRequestFullscreen();
    pub fn nuuroWasmCancelFullscreen();
    pub fn nuuroWasmIsFullscreen() -> c_int;

    pub fn nuuroWasmWriteCookie(size: usize, data: *const c_void);
}
