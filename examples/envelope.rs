use std::{fs::File, io::BufWriter};

use synth8::prelude::*;

fn main() {
    let synthesizer = Wave::new(Sine).frequency(440.0);

    let envelope = Envelope {
        attack: 1.0,
        attack_time: 0.2,
        decay_time: 0.3,
        sustain: 0.6,
        sustain_time: 0.4,
        release_time: 0.3,
        synthesizer,
    };

    let writer = BufWriter::new(File::create("envelope_sin440.wav").unwrap());

    let duration = envelope.duration();

    synthesize(envelope, 44100, duration, writer).unwrap();
}
