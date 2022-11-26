use cipher_fuzzer::{
    samples::{
        messages_vec,
        plaintexts_vec,
    },
    ciphers::CipherStack,
    distribution::{Sigmas, Distributions},
    measurements::measure,
};
use clap::Parser;
use std::collections::VecDeque;
use statistical as stat;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of iterations to fuzz for
    #[arg(short, long, default_value_t = 10000)]
    iterations: usize,

    /// Number of cipher step used by the cipher
    #[arg(long, default_value_t = 3)]
    steps: u32,

    /// Score threshold
    #[arg(long, default_value_t = 500.0)]
    score_threshold: f64,
}

#[derive(Debug)]
struct Testcase {
    score: f64,
    cipher_stack: CipherStack,
}

struct Fuzzer {
    /// Parameters given through cli
    args: Args,

    /// CipherStacks that were scheduly mutated
    corpus: Vec<Testcase>,

    /// CipherStacks found below the threshold, to be scheduly mutated
    queue: VecDeque<Testcase>,

    /// Distributions used for this campaign
    distributions: Distributions,

    /// Function that generate plaintext
    pts_fn: fn() -> Vec<Vec<u8>>,

    /// Goal sigma values
    messages_sigmas: Sigmas,

    /// Number of cipher stack evaluated
    execs: usize,
}

impl Fuzzer {
    fn new(args: Args) -> Fuzzer {
        let distributions = Distributions::random();
        Fuzzer {
            args,
            corpus: Vec::new(),
            queue: VecDeque::new(),
            messages_sigmas: distributions.sigmas(&measure(&messages_vec())),
            distributions,
            pts_fn: plaintexts_vec,
            execs: 0,
        }
    }

    fn get_cipher_stack_score(&mut self, cipher_stack: &CipherStack) -> f64 {
        self.execs += 1;
        let cts = cipher_stack.encrypt((self.pts_fn)());
        let cts_p = self.distributions.sigmas(&measure(&cts));
        self.messages_sigmas.distance(&cts_p)
    }

    fn testcase_below_threshold(&mut self) -> Testcase {
        loop {
            let cipher_stack = CipherStack::random(self.args.steps);
            let score = self.get_cipher_stack_score(&cipher_stack);
            if score < self.args.score_threshold {
                return Testcase { score, cipher_stack };
            }
        }
    }

    fn fuzz(&mut self) -> std::io::Result<()> {
        let mut best_score = self.args.score_threshold;
        for i in 0..self.args.iterations {
            if i % 1000 == 0 {
                println!("STATUS execs: {}, best_score: {}, queue: {}",
                    self.execs, best_score, self.queue.len());
            }
            if let Some(testcase) = self.queue.pop_back() {
                let mut improved = false;
                let mut scores = Vec::new();
                let mutation_count = testcase.cipher_stack.mutation_count();
                for i in 0..mutation_count * 2 {
                    let mut cipher_stack = testcase.cipher_stack.clone();
                    if i >= mutation_count {
                        cipher_stack.mutate(Some(i - mutation_count));
                    } else {
                        cipher_stack.mutate(None);
                    }
                    let score = self.get_cipher_stack_score(&cipher_stack);
                    if i >= mutation_count {
                        scores.push(score);
                    }
                    if score < testcase.score * 0.95 {
                        improved = true;
                        self.queue.push_front(Testcase { score, cipher_stack });
                    }
                }
                // If we are in a local minimum
                if !improved && scores.len() != 0 {
                    let score = stat::median(&scores);
                    if score < best_score {
                        best_score = score;
                        println!("NEW BEST SCORE: {}", best_score);
                        println!("{:?}", testcase.cipher_stack);
                    }
                }
            } else {
                let tc = self.testcase_below_threshold();
                self.queue.push_front(tc);
            }
        }
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let mut fuzzer = Fuzzer::new(args);
    fuzzer.fuzz()?;
    Ok(())
}
