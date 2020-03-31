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

//! Nuuro is a specialized game development library.
//!
//! When creating a game, it is good practice to make a layer,
//! specific to one's needs, that separates the
//! game logic from the resource management, rendering, audio, and other interfacing
//! that is needed for a game.
//!
//! Users of this crate should create a build script in their project,
//! invoking functionality from the sibling crate "nuuro_build".
//! This will generate texture atlases and enums to reference assets.
//! See the "nuuro_build" crate for more details.
//!
//! You can start with the [nuuro template](https://github.com/juandroid007/nuuro_template).

#[macro_use]
extern crate lazy_static;
extern crate byteorder;
#[cfg(not(target_arch = "wasm32"))]
extern crate glutin;
#[cfg(not(target_arch = "wasm32"))]
extern crate image;
pub extern crate paste;
#[cfg(not(target_arch = "wasm32"))]
extern crate rodio;
#[cfg(target_arch = "wasm32")]
extern crate serde;
#[cfg(target_arch = "wasm32")]
extern crate serde_json;

mod app_context;
mod app_info;
pub mod asset_id;
mod core;
mod input;
pub mod renderer;
pub(crate) mod timer;
pub(crate) mod utils;

#[cfg(target_arch = "wasm32")]
pub use crate::core::{wasm_exports, wasm_imports};

pub use crate::core::println;

pub use crate::app_context::{AppContext, Audio};
pub use crate::app_info::AppInfo;
pub use crate::input::{KeyCode, TouchPoint};
pub use crate::timer::Timer;

use crate::asset_id::AppAssetId;
use crate::renderer::Renderer;

const MAX_TIMESTEP: f64 = 1. / 15.;

/// Invoke this in a `main` method to run the `App`.
///
/// Will panic if this method is called more than once.
/// The `AppInfo` is used to specify intiailization parameters for the application.
pub fn run<AS: 'static + AppAssetId, AP: 'static + App<AS>>(info: AppInfo, app: AP) {
    core::run(info, app);
}

/// Simple macro to print a message in the console of the current target.
#[macro_export]
macro_rules! nuuro_println {
    ($($t: tt )*) => {
        ::nuuro::println(format!($($t)*));
    };
}

/// Trait that a user can implement to specify application behavior, passed into `nuuro::run(...)`.
pub trait App<A: AppAssetId> {
    /// Invoked when the application is first started, default behavior is a no-op.
    fn start(&mut self, _ctx: &mut AppContext<A>) {}

    /// Advances the app state by a given amount of `seconds` (usually a fraction of a second).
    fn advance(&mut self, seconds: f64, ctx: &mut AppContext<A>);

    /// Invoked when a key or mouse button is pressed down.
    fn key_down(&mut self, key: KeyCode, ctx: &mut AppContext<A>);

    /// Invoked when a key or mouse button is released, default behavior is a no-op.
    fn key_up(&mut self, _key: KeyCode, _ctx: &mut AppContext<A>) {}

    /// Render the app in its current state.
    fn render(&mut self, renderer: &mut Renderer<A>, ctx: &AppContext<A>);
}
