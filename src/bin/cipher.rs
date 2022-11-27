use cipher_fuzzer::{
    common::*,
    samples::{
        get_texts,
        plaintexts_vec,
        messages_vec,
    },
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
    cipher_filename: String,

    /// Use the messages instead of a plaintext
    #[arg(short, default_value_t = false)]
    reverse: bool,

    /// Use plaintext from stdin
    #[arg(short, default_value_t = false)]
    stdin: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let cipher_stack = CipherStack::load(&args.cipher_filename)?;
    if args.stdin {
        print_texts(&cipher_stack.encrypt(get_texts()?));
    } else if args.reverse {
        print_texts(&cipher_stack.encrypt(messages_vec()));
    } else {
        print_texts(&cipher_stack.encrypt(plaintexts_vec()));
    }
    Ok(())
}
