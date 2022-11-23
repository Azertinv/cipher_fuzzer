use cipher_fuzzer::{
    samples::get_texts,
    measurements::*,
};
use std::io;

fn main() -> io::Result<()> {
    let cts = get_texts()?;
    println!("{:?}", letter_frequency::measure(&cts));
    println!("{:?}", letter_repeats::measure(&cts));
    println!("{:?}", index_bounds::measure(&cts));
    Ok(())
}
