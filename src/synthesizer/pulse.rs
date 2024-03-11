use crate::prelude::Sample;

use super::TimeSynthesizer;

pub struct Pulse {
    pub duration: f32,
}

impl TimeSynthesizer for Pulse {
    fn sample(&mut self, time: f32) -> Sample {
        Sample(2.0 * ((time + self.duration).floor() - time.floor()) - 1.0)
    }
}
