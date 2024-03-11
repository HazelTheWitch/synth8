use crate::prelude::Sample;

use super::TimeSynthesizer;

pub struct Wave<S> {
    pub frequency: f32,
    pub amplitude: f32,
    pub offset: f32,
    pub constant: f32,
    pub synthesizer: S,
}

impl<S> Wave<S> {
    pub fn new(synthesizer: S) -> Self {
        Self { frequency: 1.0, amplitude: 1.0, offset: 0.0, constant: 0.0, synthesizer }
    }

    pub fn frequency(mut self, frequency: f32) -> Self {
        self.frequency = frequency;

        self
    }

    pub fn amplitude(mut self, amplitude: f32) -> Self {
        self.amplitude = amplitude;

        self
    }

    pub fn offset(mut self, offset: f32) -> Self {
        self.offset = offset;

        self
    }

    pub fn constant(mut self, constant: f32) -> Self {
        self.constant = constant;

        self
    }
}

impl<S> TimeSynthesizer for Wave<S>
where
    S: TimeSynthesizer,
{
    fn sample(&mut self, time: f32) -> Sample {
        let sample = self.synthesizer.sample((time - self.offset) * self.frequency).into_inner();
        Sample(sample * self.amplitude + self.constant)
    }
}
