use crate::prelude::Sample;

use super::Synthesizer;

pub struct Combiner<O, S1, S2> {
    pub first: S1,
    pub second: S2,
    pub operation: O,
}

pub trait BinaryOperation {
    fn operate(&self, first: f32, second: f32) -> f32;
}

impl<O, S1, S2> Synthesizer for Combiner<O, S1, S2>
where
    O: BinaryOperation,
    S1: Synthesizer,
    S2: Synthesizer,
{
    fn next(&mut self, sample: u32, samples_per_second: u32) -> Sample {
        let first = self.first.next(sample, samples_per_second).into_inner();
        let second = self.second.next(sample, samples_per_second).into_inner();

        Sample(self.operation.operate(first, second))
    }
}

pub struct Min;

impl BinaryOperation for Min {
    fn operate(&self, first: f32, second: f32) -> f32 {
        first.min(second)
    }
}

pub struct Max;

impl BinaryOperation for Max {
    fn operate(&self, first: f32, second: f32) -> f32 {
        first.max(second)
    }
}

pub struct Limit;

impl BinaryOperation for Limit {
    fn operate(&self, first: f32, second: f32) -> f32 {
        let other = second.abs();

        if first < 0.0 {
            first.min(-other)
        } else {
            first.max(other)
        }
    }
}

pub struct Gain;

impl BinaryOperation for Gain {
    fn operate(&self, first: f32, second: f32) -> f32 {
        first * second.abs()
    }
}

pub struct Add;

impl BinaryOperation for Add {
    fn operate(&self, first: f32, second: f32) -> f32 {
        first + second
    }
}

