use crate::prelude::{Sample, Synthesizer};

use super::white::WhiteNoise;

pub struct BrownNoise {
    white: WhiteNoise,
    current: f32,
    beta: f32,
}

impl BrownNoise {
    pub fn new(beta: f32) -> Self {
        Self { white: WhiteNoise::new(), current: 0.0, beta }
    }
}


impl Synthesizer for BrownNoise {
    fn next(&mut self, sample: u32, samples_per_second: u32) -> Sample {
        self.current -= self.beta * (self.current - self.white.next(sample, samples_per_second).into_inner());
        Sample(self.current)
    }
}
