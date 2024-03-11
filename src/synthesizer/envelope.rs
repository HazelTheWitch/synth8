use crate::prelude::Sample;

use super::Synthesizer;

pub struct Envelope<S> {
    pub attack: f32,
    pub attack_time: f32,
    pub decay_time: f32,
    pub sustain: f32,
    pub sustain_time: f32,
    pub release_time: f32,
    pub synthesizer: S
}

impl<S> Envelope<S> {
    pub(crate) fn amplitude(&self, mut time: f32) -> f32 {
        if time <= 0.0 {
            return 0.0;
        }

        if time <= self.attack_time {
            return self.attack / self.attack_time * time;
        }

        time -= self.attack_time;

        if time <= self.decay_time {
            return self.attack - (self.attack - self.sustain) / self.decay_time * time;
        }

        time -= self.decay_time;

        if time <= self.sustain_time {
            return self.sustain;
        }

        time -= self.sustain_time;

        if time <= self.release_time {
            return self.sustain - self.sustain / self.release_time * time;
        }

        return 0.0;
    }

    pub fn duration(&self) -> f32 {
        self.attack_time + self.decay_time + self.sustain_time + self.release_time
    }
}


impl<S> Synthesizer for Envelope<S>
where
    S: Synthesizer,
{
    fn next(&mut self, sample: u32, samples_per_second: u32) -> Sample {
        let time = sample as f32 / samples_per_second as f32;

        self.synthesizer.next(sample, samples_per_second) * self.amplitude(time)

    }
}
