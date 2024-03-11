use crate::prelude::Sample;

use super::TimeSynthesizer;

pub struct Generic<F> {
    callback: F,
}

impl<F> TimeSynthesizer for Generic<F>
where
    F: Fn(f32) -> f32
{
    fn sample(&mut self, time: f32) -> Sample {
        Sample((self.callback)(time))
    }
}
