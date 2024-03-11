use crate::prelude::Sample;

use super::Synthesizer;

pub struct Amplitude<S> {
    pub amplitude: f32,
    pub synthesizer: S,
}

impl<S> Synthesizer for Amplitude<S>
where
    S: Synthesizer
{
    fn next(&mut self, sample: u32, samples_per_second: u32) -> Sample {
        self.synthesizer.next(sample, samples_per_second) * self.amplitude
    }
}
