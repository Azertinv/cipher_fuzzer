use cipher_fuzzer::{
    samples::{
        messages_vec,
        plaintexts_vec,
    },
    ciphers::CipherStack,
    distribution::Distributions,
    measurements::measure,
};
use clap::Parser;
use std::collections::VecDeque;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of cipher step used by the cipher
    #[arg(short, long, default_value_t = 1)]
    steps: u32,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let random_dist = Distributions::random();

    let messages_p = random_dist.p_values(&measure(&messages_vec()));

    let mut queue: VecDeque<CipherStack> = VecDeque::new();
    let mut best_score = 100000000000000.0;

    for _ in 0..10 {
        queue.push_back(CipherStack::random(args.steps));
    }

    let pts = plaintexts_vec();
    for _ in 0..10000 {
        let cipher_stack = CipherStack::random(args.steps);
        let cts = cipher_stack.encrypt(pts.clone());
        let cts_p = random_dist.p_values(&measure(&cts));
        let score = messages_p.distance(&cts_p);
        if score < best_score {
            best_score = score;
            cipher_stack.save("/tmp/best_cipher.cs")?;
            queue.push_back(cipher_stack);
            println!("{score}");
        }
    }
    println!("{best_score}");

    Ok(())
}
