use std::io;

use super::sound_data::SoundData;

pub struct SoundSource {
    data: io::Cursor<SoundData>,
    sink: rodio::Sink,
}

impl SoundSource {
    // Create a new `SoundSource` from the given file.
    pub fn new(device: &rodio::Device, path: &str) -> io::Result<Self> {
        let path = path.as_ref();
        let data = SoundData::new(path)?;
        SoundSource::from_data(device, data)
    }

    // Creates a new `SoundSource` using the given `SoundData` object.
    pub fn from_data(device: &rodio::Device, data: SoundData) -> io::Result<Self> {
        if !data.can_play() {
            panic!("Could not decode the given audio data");
        }
        let sink = rodio::Sink::new(device);
        let data = io::Cursor::new(data);
        Ok(SoundSource {
            data,
            sink,
        })
    }

    pub fn play(&self, repeat: bool) {
        use rodio::Source;
        let cursor = self.data.clone();
        if repeat {
            let sound = rodio::Decoder::new(cursor)
                .unwrap()
                .repeat_infinite();
            self.sink.append(sound);
        } else {
            let sound = rodio::Decoder::new(cursor)
                .unwrap();
            self.sink.append(sound);
        }
    }

    pub fn stop(&mut self) {
        let device = rodio::default_output_device().unwrap();
        self.sink = rodio::Sink::new(&device);
    }
}
