use std::{fs::File, io::BufWriter};

use synth8::{prelude::*, synthesizer::combiner::Add};

fn main() {
    let c7 = Wave::new(Sine).frequency(2093.00).amplitude(1.2);
    let gb7 = Wave::new(Sine).frequency(2959.96).amplitude(1.2);
    let something = Wave::new(Sine).frequency(2673.96).amplitude(1.2);

    let together = Combiner { first: c7, second: gb7, operation: Add };
    let together = Combiner { first: together, second: something, operation: Add };

    let together = Combiner { first: together, second: WhiteNoise::new(), operation: Gain };

    let writer = BufWriter::new(File::create("creepy.wav").unwrap());

    synthesize(together, 5000, 4.0, writer).unwrap();
}
