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

#[derive(Debug)]
struct Testcase {
    score: f64,
    cipher_stack: CipherStack,
}

struct HillClimber {
    /// Parameters given through cli
    args: Args,

    /// Fitness function
    fitness_fn: fn(&Cts) -> f64,

    /// Best scorer
    queue: VecDeque<Testcase>,

    /// Number of cipher stack evaluated
    execs: usize,
}

impl HillClimber {
    fn new(args: Args) -> HillClimber {
        HillClimber {
            args,
            fitness_fn: |x| get_ioc(get_letter_count(x)),
            queue: VecDeque::new(),
            execs: 0,
        }
    }

    fn climb(&mut self) -> std::io::Result<()> {
        let pts = messages_vec();
        let mut best_score = 0.0f64;
        for i in 0.. {
            if i % 100000 == 0 {
                println!("STATUS execs: {}, best_score: {}",
                    self.execs, best_score);
            }
            if let Some(Testcase{score, cipher_stack}) = self.queue.pop_front() {
                let mutation_count = cipher_stack.mutation_count();
                for i in 0..mutation_count * 2 {
                    let mut cipher_stack = cipher_stack.clone();
                    if i >= mutation_count {
                        cipher_stack.mutate(Some(i - mutation_count));
                    } else {
                        cipher_stack.mutate(None);
                    }
                    let score = (self.fitness_fn)(&cipher_stack.encrypt(pts.clone()));
                    self.execs += 1;
                    if score > best_score {
                        best_score = score;
                        println!("NEW BEST SCORE: {}", best_score);
                        println!("{:#?}", cipher_stack);
                        self.queue.push_back(Testcase { score, cipher_stack } )
                    }
                }
            } else {
                let cipher_stack = CipherStack::random(self.args.steps);
                let score = (self.fitness_fn)(&cipher_stack.encrypt(pts.clone()));
                self.execs += 1;
                if score > best_score {
                    best_score = score;
                    println!("NEW BEST SCORE: {}", best_score);
                    println!("{:#?}", cipher_stack);
                    self.queue.push_back(Testcase { score, cipher_stack } )
                } else if score > 1.20 {
                    self.queue.push_back(Testcase { score, cipher_stack } )
                }
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
