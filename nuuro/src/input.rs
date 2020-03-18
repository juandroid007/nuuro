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

//! Structs related to user input.

#[cfg(target_arch = "wasm32")]
use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use std::mem;

/// Enum for keyboard keys and mouse buttons.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum KeyCode {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Right,
    Left,
    Down,
    Up,
    Return,
    Space,
    Backspace,
    Delete,
    MouseLeft,
    MouseRight,
    MouseMiddle,
    Touch,
}

/// Struct that store touch position
#[cfg(not(target_arch = "wasm32"))]
#[derive(Copy, Clone)]
pub struct TouchPoint {
    pub(crate) id: u32,
    pub(crate) x: f64,
    pub(crate) y: f64,
}

/// Struct that store touch position
#[cfg(target_arch = "wasm32")]
#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct TouchPoint {
    pub(crate) id: u32,
    pub(crate) x: f64,
    pub(crate) y: f64,
}

impl TouchPoint {
    /// Return touch identifier
    pub fn id(&self) -> u32 {
        self.id
    }
    /// Return touch position (x, y)
    pub fn pos(&self) -> (f64, f64) {
        (self.x, self.y)
    }
}

impl std::fmt::Debug for TouchPoint {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        if fmt.alternate() {
            write!(
                fmt,
                "TouchPoint {{\n\tid: {},\n\tpos: {:?}\n}}",
                self.id(),
                self.pos()
            )
        } else {
            write!(
                fmt,
                "TouchPoint {{ id: {}, pos: {:?} }}",
                self.id(),
                self.pos()
            )
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl KeyCode {
    fn count() -> u8 {
        KeyCode::Touch as u8 + 1
    }
    pub(crate) fn from_u8(id: u8) -> Option<KeyCode> {
        if id < Self::count() {
            Some(unsafe { mem::transmute(id) })
        } else {
            None
        }
    }
}
