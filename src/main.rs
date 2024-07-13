use std::{
    env::args,
    fs::File,
    io::{copy, BufReader},
    time::Instant,
};

use flate2::{write::GzEncoder, Compression};

extern crate flate2;

fn main() {
    if args().len() != 3 {
        eprintln!(
            "Usage: 
        compress `source` `target`"
        );
        return;
    }

    let mut inp = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let out = File::create(args().nth(2).unwrap()).unwrap();

    let mut encoder = GzEncoder::new(out, Compression::default());

    let start = Instant::now();

    copy(&mut inp, &mut encoder).unwrap();

    let output = encoder.finish().unwrap();

    println!("Source len: {:?}", inp.get_ref().metadata().unwrap().len());
    println!("Target len: {:?}", output.metadata().unwrap().len());
    println!("Elapsed: {:?}", start.elapsed());
}
