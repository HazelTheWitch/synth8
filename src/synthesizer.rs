pub mod envelope;
pub mod pulse;
pub mod sawtooth;
pub mod sine;
pub mod wave;
pub mod generic;
pub mod combiner;
pub mod noise;
pub mod amplitude;

use crate::sample::Sample;

pub trait TimeSynthesizer {
    fn sample(&mut self, time: f32) -> Sample;
}

impl<T> Synthesizer for T
where
    T: TimeSynthesizer
{
    fn next(&mut self, sample: u32, samples_per_second: u32) -> Sample {
        let time = sample as f32 / samples_per_second as f32;

        self.sample(time)
    }
}

pub trait Synthesizer {
    fn next(&mut self, sample: u32, samples_per_second: u32) -> Sample;
}
