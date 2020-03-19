use std::io;

use rodio::buffer::SamplesBuffer;
use rodio::Decoder;
use rodio::Source as RSource;

use super::sound_data::SoundData;

pub struct SoundSource {
    // data: io::Cursor<SoundData>,
    sink: rodio::Sink,
    channels: u16,
    samples_rate: u32,
    samples: Vec<f32>,
}

impl SoundSource {
    // Create a new `SoundSource` from the given file.
    pub fn new(device: &rodio::Device, path: &str) -> io::Result<Self> {
        let data = SoundData::new(path)?;
        SoundSource::from_data(device, data)
    }

    // Creates a new `SoundSource` using the given `SoundData` object.
    pub fn from_data(device: &rodio::Device, data: SoundData) -> io::Result<Self> {
        if !data.can_play() {
            panic!("Could not decode the given audio data");
        }
        let sink = rodio::Sink::new(device);
        let cursor = io::Cursor::new(data);
        let src = Decoder::new(cursor).unwrap();
        Ok(SoundSource {
            // data,
            sink,
            channels: src.channels(),
            samples_rate: src.sample_rate(),
            samples: src.convert_samples().collect::<Vec<f32>>(),
        })
    }

    fn to_buffer(&self) -> SamplesBuffer<f32> {
        SamplesBuffer::new(self.channels, self.samples_rate, self.samples.clone())
    }

    pub fn play(&self, repeat: bool) {
        if repeat {
            let sound = self.to_buffer().repeat_infinite();
            self.sink.append(sound);
        } else {
            let sound = self.to_buffer();
            self.sink.append(sound);
        }
    }

    pub fn stop(&mut self) {
        let device = rodio::default_output_device().unwrap();
        self.sink = rodio::Sink::new(&device);
    }
}
