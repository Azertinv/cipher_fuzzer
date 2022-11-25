use crate::common::*;
pub mod messages;
pub use messages::{messages_vec, MESSAGES};

pub mod plaintexts;
pub use plaintexts::{plaintexts_vec, PLAINTEXTS};

use rand::Rng;
use rand::seq::SliceRandom;
use std::io::{Error, ErrorKind};
use std::io;

pub fn get_texts() -> io::Result<Cts> {
    let mut lines = vec![];
    for l in io::stdin().lines() {
        let mut line = l?.as_bytes().to_vec();
        for c in line.iter_mut() {
            *c -= READABLE_OFFSET;
            if *c >= CT_ALPHABET_SIZE {
                return Err(Error::new(ErrorKind::Other, "Input is out of bound of the alphabet"))
            }
        }
        lines.push(line);
    }
    Ok(lines)
}

fn random_ciphertext() -> Ct {
    let mut rng = rand::thread_rng();
    let size = rng.gen_range(MIN_CT_USIZE..=MAX_CT_USIZE);
    (0..size).map(|_| *CT_ALPHABET.choose(&mut rng).unwrap()).collect()
}

pub fn random_ciphertexts() -> Cts {
    (0..CT_PER_CTS).map(|_| random_ciphertext()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_plaintext_in_bound() {
        let pts = plaintexts_vec();
        for pt in pts.iter() {
            for l in pt {
                assert!(CT_ALPHABET.contains(l));
            }
        }
    }

    #[test]
    fn sample_messages_in_bound() {
        let cts = messages_vec();
        for ct in cts.iter() {
            for l in ct {
                assert!(CT_ALPHABET.contains(l));
            }
        }
    }
}
