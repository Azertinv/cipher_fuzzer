use crate::ciphers::{
    InnerCipher,
    Cipher,
};
use crate::common::*;

use rand::{thread_rng, Rng};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Shift {
    pub key: u8,
}

#[typetag::serde]
impl Cipher for Shift {
    fn generate() -> Self where Self: Sized {
        Shift { key: thread_rng().gen_range(0..CT_ALPHABET_SIZE) }
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
        self.key = self.key.rem_euclid(CT_ALPHABET_SIZE);
    }

    fn encrypt(&self, ct: &mut [u8]) {
        let alphabet: Vec<u8> = (0..CT_ALPHABET_SIZE).map(|letter| {
            (letter + self.key).rem_euclid(CT_ALPHABET_SIZE)
        }).collect();
        substitute(ct, &alphabet);
    }
}

#[typetag::serde]
impl InnerCipher for Shift {
    fn from_hint(hint: i32) -> Box<dyn Cipher> where Self: Sized {
        let key = hint.rem_euclid(CT_ALPHABET_SIZE.into()) as u8;
        Box::new(Shift { key }) as Box<dyn Cipher>
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mutate() {
        let mut shift = Shift { key: 42 };
        shift.mutate(Some(0));
        assert_eq!(shift.key, 43);
        shift.mutate(Some(1));
        assert_eq!(shift.key, 42);
    }

    #[test]
    fn encrypt() {
        let shift = Shift { key: 42 };
        let mut data = vec![1, 2, 3, 80];
        shift.encrypt(&mut data);
        assert_eq!(data, vec![43, 44, 45, 39]);
    }
}