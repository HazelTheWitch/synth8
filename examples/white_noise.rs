use std::{fs::File, io::BufWriter};

use synth8::prelude::*;

fn main() {
    let synthesizer = WhiteNoise::new();

    let writer = BufWriter::new(File::create("white.wav").unwrap());

    synthesize(synthesizer, 44100, 1.0, writer).unwrap();
}