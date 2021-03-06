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

mod app_clock;
mod core_audio;
mod event_handler;

pub use self::core_audio::CoreAudio;

use std::ffi::CStr;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;

use glutin::dpi::LogicalSize;
use glutin::EventsLoop;
use glutin::WindowBuilder;
use glutin::{ContextBuilder, ContextTrait, WindowedContext};

use gl::types::*;

use self::app_clock::AppClock;
use self::event_handler::EventHandler;
use super::mark_app_created_flag;
use crate::app_info::AppInfo;
use crate::asset_id::{AppAssetId, IdU16};
use crate::renderer::atlas::Atlas;
use crate::renderer::core_renderer::CoreRenderer;
use crate::renderer::core_renderer::Texture;
use crate::renderer::render_buffer::RenderBuffer;
use crate::renderer::Renderer;
use crate::{timer, App, AppContext};

/// Macro to be placed in the `main.rs` file for a Nuuro app.
///
/// Currently, the only use this macro has is to export WASM functions for the app
/// when compiling to the `wasm32-unknown-unknown` target.
#[macro_export]
macro_rules! nuuro_header {
    () => {};
}

pub fn run<AS: 'static + AppAssetId, AP: 'static + App<AS>>(info: AppInfo, mut app: AP) {
    mark_app_created_flag();

    let core_audio = CoreAudio::new(AS::Sound::count(), AS::Music::count());

    let mut events_loop = EventsLoop::new();

    let mut event_handler = EventHandler::new();
    let window = WindowBuilder::new()
        .with_title(info.title)
        .with_dimensions(LogicalSize::new(
            info.window_pixels.0 as f64,
            info.window_pixels.1 as f64,
        ))
        .with_resizable(info.resizable);
    let gl_context = ContextBuilder::new()
        .with_gl_debug_flag(true)
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 0)))
        .build_windowed(window, &events_loop)
        .unwrap();
    unsafe { gl_context.make_current().unwrap() };

    let timer = Instant::now();

    init_gl(&gl_context);

    let mut renderer = build_renderer(&info);

    gl_error_check();

    let mut ctx = AppContext::new(core_audio, renderer.app_dims(), renderer.native_px());

    if info.print_gl_info {
        print_gl_info();
    }

    app.start(&mut ctx);

    let mut clock = AppClock::new(timer, &info);

    loop {
        events_loop.poll_events(|event| {
            if !event_handler.process_events(event, &mut app, &mut ctx, &renderer) {
                ctx.close();
            }
        });

        unsafe {
            gl::ClearColor(0., 0., 0., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        let screen_dims = (
            gl_context.window().get_inner_size().unwrap().width as u32,
            gl_context.window().get_inner_size().unwrap().height as u32,
        );

        if screen_dims.0 > 0 && screen_dims.1 > 0 {
            renderer.set_screen_dims(screen_dims);
            ctx.set_dims(renderer.app_dims(), renderer.native_px());
            app.render(&mut renderer, &ctx);
            renderer.flush();
        }

        gl_error_check();
        gl_context.swap_buffers().unwrap();

        let elapsed = clock.step();

        match (ctx.is_fullscreen(), ctx.desires_fullscreen()) {
            (false, true) => {
                gl_context
                    .window()
                    .set_fullscreen(Some(events_loop.get_primary_monitor()));
                ctx.set_is_fullscreen(true);
            }
            (true, false) => {
                gl_context.window().set_fullscreen(None);
                ctx.set_is_fullscreen(false);
            }
            (false, false) | (true, true) => {}
        }

        let normalized_elapsed = elapsed.min(crate::MAX_TIMESTEP);
        timer::update_all(normalized_elapsed);
        app.advance(normalized_elapsed, &mut ctx);
        if ctx.take_close_request() {
            break;
        }
    }
}

#[allow(dead_code)]
pub fn println(string: String) {
    println!("{}", string);
}

fn build_renderer<AS: AppAssetId>(info: &AppInfo) -> Renderer<AS> {
    let sprites_atlas =
        Atlas::new(BufReader::new(File::open("assets/sprites.atlas").unwrap())).unwrap();
    let render_buffer = RenderBuffer::new(&info, info.window_pixels, sprites_atlas);

    let sprites_tex = Texture::new("assets/sprites.png");

    // TODO need to ensure Nearest-neighbor sampling is used?
    let core_renderer = CoreRenderer::new(sprites_tex);

    Renderer::<AS>::new(render_buffer, core_renderer)
}

fn init_gl(gl_context: &WindowedContext) {
    gl::load_with(|name| gl_context.get_proc_address(name) as *const _);

    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::ONE, gl::ONE_MINUS_SRC_ALPHA);
    }
}

fn print_gl_info() {
    println!("OpenGL version: {:?}", gl_get_string(gl::VERSION));
    println!(
        "GLSL version: {:?}",
        gl_get_string(gl::SHADING_LANGUAGE_VERSION)
    );
    println!("Vendor: {:?}", gl_get_string(gl::VENDOR));
    println!("Renderer: {:?}", gl_get_string(gl::RENDERER));
}

fn gl_get_string<'a>(name: GLenum) -> &'a CStr {
    unsafe { CStr::from_ptr(gl::GetString(name) as *const i8) }
}

fn gl_error_check() {
    let error = unsafe { gl::GetError() };
    assert!(
        error == gl::NO_ERROR,
        "unexpected OpenGL error, code {}",
        error
    );
}
