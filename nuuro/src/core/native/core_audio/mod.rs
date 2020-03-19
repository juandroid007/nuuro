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

// use std::path::PathBuf;

//  use sdl2::mixer::{self, Music};
use rodio::Device;

mod sound_data;
mod sound_source;

use sound_source::SoundSource;

pub struct CoreAudio {
    device: Device,
    music: Option<SoundSource>,
    sounds: Vec<SoundSource>,
    // music: Option<Music<'static>>,
    // sounds: Vec<mixer::Chunk>,
}

impl CoreAudio {
    pub(crate) fn new(sound_count: u16) -> CoreAudio {
        // let sounds: Vec<_> = (0..sound_count)
        //     .map(|id| PathBuf::from(format!("assets/sound{}.ogg", id)))
        //     .map(|p| mixer::Chunk::from_file(p).unwrap())
        //     .collect();
        let device = rodio::default_output_device().unwrap();
        let sounds: Vec<_> = (0..sound_count)
            .map(|id| SoundSource::new(&device, &format!("assets/sound{}.ogg", id)).unwrap())
            .collect();
        CoreAudio {
            device,
            sounds,
            music: None,
        }
    }

    pub fn play_sound(&mut self, sound: u16) {
        self.sounds[sound as usize].play(false);
    }

    pub fn play_music(&mut self, music: u16, repeat: bool) {
        let path = &format!("assets/music{}.ogg", music);
        self.stop_music();
        let music = SoundSource::new(&self.device, path).unwrap();
        self.music = Some(music);
        self.music.as_ref().unwrap().play(repeat);
        // let music = mixer::Music::from_file(music).unwrap();
        // music.play(if loops { 1_000_000 } else { 1 }).unwrap();
        // self.music = Some(music);
    }

    pub fn stop_music(&mut self) {
        if self.music.is_some() {
            self.music.as_mut().unwrap().stop();
        }
    }
}
