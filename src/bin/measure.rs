use cipher_fuzzer::{
    samples::{get_texts, messages_vec},
    measurements::*,
};
use clap::Parser;
use std::io;

/// CLI for ciphers
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Use plaintext from stdin
    #[arg(short, default_value_t = false)]
    stdin: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let cts = if args.stdin {
        get_texts()?
    } else {
        messages_vec()
    };
    println!("{:?}", LetterFrequency::measure(&cts));
    println!("{:?}", IndexBounds::measure(&cts));
    println!("{:?}", PeriodicIoC::measure(&cts));
    println!("{:?}", IsomorphsCounts::measure(&cts));
    Ok(())
}
