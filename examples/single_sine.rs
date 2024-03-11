use std::{fs::File, io::BufWriter};

use synth8::prelude::*;

fn main() {
    let synthesizer = Wave::new(Sine).frequency(440.0);

    let writer = BufWriter::new(File::create("sin440.wav").unwrap());

    synthesize(synthesizer, 44100, 1.0, writer).unwrap();
}
