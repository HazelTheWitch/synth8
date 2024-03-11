use std::f32::consts;

use crate::sample::Sample;

use super::TimeSynthesizer;

pub struct Sine;

impl TimeSynthesizer for Sine {
    fn sample(&mut self, time: f32) -> Sample {
        Sample(f32::sin(consts::TAU * time))
    }
}
