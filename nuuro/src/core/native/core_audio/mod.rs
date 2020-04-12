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

mod sound_data;
mod sound_source;

use sound_source::SoundSource;

pub struct CoreAudio {
    playing_music: Option<u16>,
    sounds: Vec<SoundSource>,
    musics: Vec<SoundSource>,
}

impl CoreAudio {
    pub(crate) fn new(sound_count: u16, musics_count: u16) -> CoreAudio {
        let device = rodio::default_output_device().unwrap();
        let sounds: Vec<_> = (0..sound_count)
            .map(|id| SoundSource::new(&device, &format!("assets/sound{}.ogg", id)).unwrap())
            .collect();
        let musics: Vec<_> = (0..musics_count)
            .map(|id| SoundSource::new(&device, &format!("assets/music{}.ogg", id)).unwrap())
            .collect();
        CoreAudio {
            sounds,
            playing_music: None,
            musics,
        }
    }

    pub fn play_sound(&mut self, sound: u16, volume: f32) {
        self.sounds[sound as usize].play(volume, false);
    }

    pub fn play_music(&mut self, music: u16, volume: f32, repeat: bool) {
        self.playing_music = Some(music);
        self.musics[music as usize].play(volume, repeat);
    }

    pub fn stop_music(&mut self) {
        if let Some(music) = self.playing_music {
            self.musics[music as usize].stop();
            self.playing_music = None;
        }
    }
}
