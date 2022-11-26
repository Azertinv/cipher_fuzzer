use cipher_fuzzer::{
    samples::{get_texts, random_ciphertexts, messages_vec},
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

    /// Use random plaintext
    #[arg(short, default_value_t = false)]
    random_ct: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let cts = if args.stdin {
        get_texts()?
    } else if args.random_ct {
        random_ciphertexts()
    } else {
        messages_vec()
    };

    println!("Normal stream ---------------------------------------------");
    println!("{:#?}", LetterFrequency::measure(&cts));
    println!("{:#?}", IndexBounds::measure(&cts));
    println!("{:#?}", PeriodicIoC::measure(&cts));
    println!("{:#?}", IsomorphsCounts::measure(&cts));

    println!("Delta stream ----------------------------------------------");
    let cts = delta_stream(&cts);
    println!("{:#?}", LetterFrequency::measure(&cts));
    println!("{:#?}", IndexBounds::measure(&cts));
    println!("{:#?}", PeriodicIoC::measure(&cts));
    println!("{:#?}", IsomorphsCounts::measure(&cts));

    Ok(())
}
