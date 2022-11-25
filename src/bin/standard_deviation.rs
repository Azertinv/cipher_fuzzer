use cipher_fuzzer::{
    common::*,
    measurements::*,
    distribution::Distributions,
    samples::{get_texts, plaintexts_vec, messages_vec},
    ciphers::{
        CipherStack,
    },
};
use clap::Parser;
use std::io;
use std::iter::zip;

/// CLI for ciphers
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of cipher step used by the cipher
    #[arg(short, long, default_value_t = String::from("/tmp/best_cipher.cs"))]
    cipher_filename: String,

    /// Use plaintext from stdin
    #[arg(short, default_value_t = false)]
    stdin: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let distributions = Distributions::random();

    let pts = if args.stdin {
        get_texts()?
    } else {
        plaintexts_vec()
    };
    let cipher_stack = CipherStack::load(&args.cipher_filename)?;
    let cts = cipher_stack.encrypt(pts);

    let cts_p = distributions.p_values(&measure(&cts));
    let msg_p = distributions.p_values(&measure(&messages_vec()));

    // for (cts_p_value, msg_p_value) in zip(cts_p.values, msg_p.values) {
    //     println!("{cts_p_value:4.1} {msg_p_value:4.1}");
    // }
    println!("{}", msg_p.distance(&cts_p));
    Ok(())
}
