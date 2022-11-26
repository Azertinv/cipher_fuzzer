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
use statistical as stat;

/// CLI for ciphers
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of cipher step used by the cipher
    #[arg(short, long)]
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
    let mut scores = Vec::new();
    for i in 0..cipher_stack.mutation_count() {
        let mut cipher_stack = cipher_stack.clone();
        cipher_stack.mutate(Some(i));
        let cts = cipher_stack.encrypt(pts.clone());
        let cts_p = distributions.sigmas(&measure(&cts));
        let msg_p = distributions.sigmas(&measure(&messages_vec()));
        scores.push(msg_p.distance(&cts_p));
    }
    let cts = cipher_stack.encrypt(pts.clone());
    let cts_p = distributions.sigmas(&measure(&cts));
    let msg_p = distributions.sigmas(&measure(&messages_vec()));
    let score = msg_p.distance(&cts_p);

    println!("score {}", score);
    println!("mutated score {:#?}", Summary::generate(&scores));
    Ok(())
}
