use cipher_fuzzer::{
    common::*,
    samples::{get_texts, plaintexts_vec},
    ciphers::{
        CipherStack,
    },
};
use clap::Parser;
use std::io;

/// CLI for ciphers
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of cipher step used by the cipher
    #[arg(short, long)]
    cipher_filename: Option<String>,

    /// Use plaintext from stdin
    #[arg(short, default_value_t = false)]
    stdin: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let cipher_stack = CipherStack::load(
        &args.cipher_filename
        .unwrap_or("/tmp/best_cipher.cs".to_string()))?;
    if args.stdin {
        print_texts(&cipher_stack.encrypt(get_texts()?));
    } else {
        print_texts(&cipher_stack.encrypt(plaintexts_vec()));
    }
    Ok(())
}
