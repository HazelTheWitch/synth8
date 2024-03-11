use rand::{rngs::ThreadRng, thread_rng, Rng};

use crate::prelude::{Sample, Synthesizer};

pub struct WhiteNoise {
    rng: ThreadRng,
}

impl WhiteNoise {
    pub fn new() -> Self {
        Self { rng: thread_rng() }
    }
}

impl Synthesizer for WhiteNoise {
    fn next(&mut self, _sample: u32, _samples_per_second: u32) -> Sample {
        Sample(self.rng.gen_range(-1.0..1.0))
    }
}
