use crate::prelude::Sample;

use super::TimeSynthesizer;

pub struct Sawtooth;

impl TimeSynthesizer for Sawtooth {
    fn sample(&mut self, time: f32) -> Sample {
        Sample(2.0 * (time - time.floor()) - 1.0)
    }
}
