use cipher_fuzzer::{
    samples::get_texts,
    measurements::*,
};
use std::io;

fn main() -> io::Result<()> {
    let cts = get_texts()?;
    println!("{:?}", LetterFrequency::measure(&cts));
    println!("{:?}", LetterRepeats::measure(&cts));
    println!("{:?}", IndexBounds::measure(&cts));
    println!("{:?}", IoC::measure(&cts));
    println!("{:?}", IsomorphsCounts::measure(&cts));
    Ok(())
}
