use cipher_fuzzer::{
    common::*,
    samples::{
        messages_vec,
        plaintexts_vec,
    },
    ciphers::CipherStack,
    measurements::{
        get_ioc, get_letter_count,
        measure,
    },
};
use clap::Parser;
use std::collections::VecDeque;
// use statistical as stat;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of cipher step used by the cipher
    #[arg(long, default_value_t = 2)]
    steps: u32,
}

// #[derive(Debug)]
// struct Testcase {
//     score: f64,
//     cipher_stack: CipherStack,
// }

struct HillClimber {
    /// Parameters given through cli
    args: Args,

    /// Fitness function
    fitness_fn: fn(&Cts) -> f64,

    /// Number of cipher stack evaluated
    execs: usize,
}

impl HillClimber {
    fn new(args: Args) -> HillClimber {
        HillClimber {
            args,
            fitness_fn: |x| get_ioc(get_letter_count(x)),
            execs: 0,
        }
    }

    fn climb(&mut self) -> std::io::Result<()> {
        let pts = messages_vec();
        let mut best_score = 0.0f64;
        for i in 0.. {
            if i % 10000 == 0 {
                println!("STATUS execs: {}, best_score: {}",
                    self.execs, best_score);
            }
            let cipher_stack = CipherStack::random(self.args.steps);
            let cts = cipher_stack.encrypt(pts.clone());
            let score = (self.fitness_fn)(&cts);
            self.execs += 1;
            if score > best_score {
                best_score = score;
                println!("NEW BEST SCORE: {}", best_score);
                println!("{:?}", cipher_stack);
            }
        }
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let mut hillclimber = HillClimber::new(args);
    hillclimber.climb()?;
    Ok(())
}
