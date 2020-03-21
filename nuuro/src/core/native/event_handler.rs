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

use std::collections::HashSet;

use glutin::dpi::LogicalPosition;
use glutin::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};

use crate::asset_id::AppAssetId;
use crate::input::KeyCode;
use crate::renderer::Renderer;
use crate::{App, AppContext};

pub struct EventHandler {
    held_keys: HashSet<KeyCode>,
}

impl EventHandler {
    pub fn new() -> EventHandler {
        EventHandler {
            held_keys: HashSet::new(),
        }
    }

    pub fn process_events<AS: AppAssetId, AP: App<AS>>(
        &mut self,
        event: Event,
        app: &mut AP,
        ctx: &mut AppContext<AS>,
        renderer: &Renderer<AS>,
    ) -> bool {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state,
                            virtual_keycode,
                            ..
                        },
                    ..
                } => match state {
                    ElementState::Pressed => {
                        if let Some(keycode) = virtual_keycode {
                            if let Some(keycode) = glutin_to_nuuro_key(keycode) {
                                if self.held_keys.insert(keycode) {
                                    app.key_down(keycode, ctx);
                                }
                            }
                        }
                    }
                    ElementState::Released => {
                        if let Some(keycode) = virtual_keycode {
                            if let Some(keycode) = glutin_to_nuuro_key(keycode) {
                                if self.held_keys.remove(&keycode) {
                                    app.key_up(keycode, ctx);
                                }
                            }
                        }
                    }
                },
                WindowEvent::CursorMoved {
                    position: LogicalPosition { x, y },
                    ..
                } => ctx.set_cursor(renderer.to_app_pos(x as i32, y as i32)),
                WindowEvent::MouseInput { state, button, .. } => match state {
                    ElementState::Pressed => {
                        if let Some(button) = mouse_button_to_nuuro_key(button) {
                            if self.held_keys.insert(button) {
                                app.key_down(button, ctx);
                            }
                        }
                    }
                    ElementState::Released => {
                        if let Some(button) = mouse_button_to_nuuro_key(button) {
                            if self.held_keys.remove(&button) {
                                app.key_up(button, ctx);
                            }
                        }
                    }
                },
                WindowEvent::CloseRequested => {
                    ctx.close();
                }
                _ => (),
            },
            _ => (),
        }

        if ctx.take_close_request() {
            return false;
        }
        true
    }
}

fn glutin_to_nuuro_key(key: VirtualKeyCode) -> Option<KeyCode> {
    match key {
        VirtualKeyCode::A => Some(KeyCode::A),
        VirtualKeyCode::B => Some(KeyCode::B),
        VirtualKeyCode::C => Some(KeyCode::C),
        VirtualKeyCode::D => Some(KeyCode::D),
        VirtualKeyCode::E => Some(KeyCode::E),
        VirtualKeyCode::F => Some(KeyCode::F),
        VirtualKeyCode::G => Some(KeyCode::G),
        VirtualKeyCode::H => Some(KeyCode::H),
        VirtualKeyCode::I => Some(KeyCode::I),
        VirtualKeyCode::J => Some(KeyCode::J),
        VirtualKeyCode::K => Some(KeyCode::K),
        VirtualKeyCode::L => Some(KeyCode::L),
        VirtualKeyCode::M => Some(KeyCode::M),
        VirtualKeyCode::N => Some(KeyCode::N),
        VirtualKeyCode::O => Some(KeyCode::O),
        VirtualKeyCode::P => Some(KeyCode::P),
        VirtualKeyCode::Q => Some(KeyCode::Q),
        VirtualKeyCode::R => Some(KeyCode::R),
        VirtualKeyCode::S => Some(KeyCode::S),
        VirtualKeyCode::T => Some(KeyCode::T),
        VirtualKeyCode::U => Some(KeyCode::U),
        VirtualKeyCode::V => Some(KeyCode::V),
        VirtualKeyCode::W => Some(KeyCode::W),
        VirtualKeyCode::X => Some(KeyCode::X),
        VirtualKeyCode::Y => Some(KeyCode::Y),
        VirtualKeyCode::Z => Some(KeyCode::Z),
        VirtualKeyCode::Key0 => Some(KeyCode::Num0),
        VirtualKeyCode::Key1 => Some(KeyCode::Num1),
        VirtualKeyCode::Key2 => Some(KeyCode::Num2),
        VirtualKeyCode::Key3 => Some(KeyCode::Num3),
        VirtualKeyCode::Key4 => Some(KeyCode::Num4),
        VirtualKeyCode::Key5 => Some(KeyCode::Num5),
        VirtualKeyCode::Key6 => Some(KeyCode::Num6),
        VirtualKeyCode::Key7 => Some(KeyCode::Num7),
        VirtualKeyCode::Key8 => Some(KeyCode::Num8),
        VirtualKeyCode::Key9 => Some(KeyCode::Num9),
        VirtualKeyCode::Right => Some(KeyCode::Right),
        VirtualKeyCode::Left => Some(KeyCode::Left),
        VirtualKeyCode::Down => Some(KeyCode::Down),
        VirtualKeyCode::Up => Some(KeyCode::Up),
        VirtualKeyCode::Return => Some(KeyCode::Return),
        VirtualKeyCode::Space => Some(KeyCode::Space),
        VirtualKeyCode::Back => Some(KeyCode::Backspace),
        VirtualKeyCode::Delete => Some(KeyCode::Delete),
        _ => None,
    }
}

fn mouse_button_to_nuuro_key(button: MouseButton) -> Option<KeyCode> {
    match button {
        MouseButton::Left => Some(KeyCode::MouseLeft),
        MouseButton::Right => Some(KeyCode::MouseRight),
        MouseButton::Middle => Some(KeyCode::MouseMiddle),
        _ => None,
    }
}
