use crate::ciphers::Cipher;
use crate::common::*;

use rand::{thread_rng, Rng};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Caesar {
    pub key: u8,
}

#[typetag::serde]
impl Cipher for Caesar {
    fn generate() -> Self where Self: Sized {
        Caesar { key: thread_rng().gen_range(0..CT_ALPHABET_SIZE) }
    }

    fn mutation_count(&self) -> u32 {
        2
    }

    fn mutate(&mut self, iteration: Option<u32>) {
        match iteration {
            Some(0) => self.key += 1,
            Some(1) => self.key -= 1,
            _ => *self = Self::generate(),
        }
        self.key %= CT_ALPHABET_SIZE;
    }

    fn encrypt(&self, data: &mut [u8]) {
        let alphabet: Vec<u8> = (0..CT_ALPHABET_SIZE).map(|letter| {
            (letter + self.key) % CT_ALPHABET_SIZE
        }).collect();
        substitute(data, &alphabet);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mutate() {
        let mut caesar = Caesar { key: 42 };
        caesar.mutate(Some(0));
        assert_eq!(caesar.key, 43);
        caesar.mutate(Some(1));
        assert_eq!(caesar.key, 42);
    }

    #[test]
    fn encrypt() {
        let caesar = Caesar { key: 42 };
        let mut data = vec![1, 2, 3, 80];
        caesar.encrypt(&mut data);
        assert_eq!(data, vec![43, 44, 45, 39]);
    }
}