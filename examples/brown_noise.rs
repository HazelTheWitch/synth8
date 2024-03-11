use std::{fs::File, io::BufWriter};

use synth8::prelude::*;

fn main() {
    let synthesizer = BrownNoise::new(0.025);

    let writer = BufWriter::new(File::create("brown.wav").unwrap());

    synthesize(synthesizer, 44100, 5.0, writer).unwrap();
}