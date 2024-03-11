pub mod sample;
pub mod synthesizer;

use std::io::{Seek, Write};

use hound::{WavSpec, WavWriter};
use synthesizer::Synthesizer;

pub mod prelude {
    pub use crate::{
        sample::Sample,
        synthesize,
        synthesizer::{
            envelope::Envelope, pulse::Pulse, sawtooth::Sawtooth, sine::Sine, wave::Wave, amplitude::Amplitude,
            combiner::{Combiner, Max, Min, Limit, Gain},
            noise::{white::WhiteNoise, brown::BrownNoise},
            TimeSynthesizer, Synthesizer,
        },
    };
}

pub fn synthesize(
    mut synthesizer: impl Synthesizer,
    samples_per_second: u32,
    length: f32,
    writer: impl Write + Seek,
) -> Result<(), hound::Error> {
    let spec = WavSpec {
        channels: 1,
        sample_rate: samples_per_second,
        bits_per_sample: 8,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = WavWriter::new(writer, spec)?;

    let sample_rate = samples_per_second as f32;
    let samples = (sample_rate * length) as u32;

    for sample in 0..samples {
        let sample = synthesizer.next(sample, samples_per_second);

        writer.write_sample(sample.to_i8())?;
    }

    Ok(())
}
