use crate::common::*;
use crate::ciphers::{
    InnerCipherFactory,
    Cipher,
};

use rand::{
    thread_rng,
    Rng,
    seq::SliceRandom,
};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Repeater {
    key: Vec<u8>,
    cipher_factory: InnerCipherFactory,
}

#[typetag::serde]
impl Cipher for Repeater {
    fn generate() -> Self where Self: Sized {
        let mut rng = thread_rng();
        let key = (0..rng.gen_range(1..=50))
            .map(|_| *CT_ALPHABET.choose(&mut rng).unwrap())
            .collect();
        let cipher_factory = InnerCipherFactory::random_factory();
        Repeater { key, cipher_factory }
    }

    fn mutation_count(&self) -> u32 {
        3
    }

    fn mutate(&mut self, iteration: Option<u32>) {
        match iteration {
            Some(choice) => {
                let key_len = self.key.len();
                let mut rng = thread_rng();
                let rand_index = rng.gen_range(0..key_len);
                let rand_letter = *CT_ALPHABET.choose(&mut rng).unwrap();
                match choice {
                    0 if key_len > 1 => { self.key.remove(rand_index); },
                    1 if key_len < 50 => { self.key.insert(rand_index, rand_letter); },
                    _ => { self.key[rand_index] = rand_letter; },
                };
            },
            _ => *self = Self::generate(),
        }
    }

    fn encrypt(&self, data: &mut [u8]) {
        let key_len = self.key.len();
        let ciphers: Vec<Box<dyn Cipher>> = self.key.iter()
            .map(|k| self.cipher_factory.build_from_hint((*k).into()))
            .collect();
        for i in 0..data.len() {
            ciphers[i % key_len].encrypt(&mut data[i..i+1]);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mutate() {
        let mut repeater = Repeater {
            key: "ABCDE".as_bytes().to_vec(),
            cipher_factory: InnerCipherFactory::ShiftFactory,
        };
        repeater.mutate(Some(0));
        assert_eq!(repeater.key.len(), 4);
        repeater.mutate(Some(1));
        assert_eq!(repeater.key.len(), 5);
        repeater.mutate(Some(2));
        assert_eq!(repeater.key.len(), 5);
    }

    #[test]
    fn encrypt() {
        let mut data = vec![5, 18, 13, 8, CT_ALPHABET_SIZE - 1, 8, 8, 8];
        let repeater = Repeater {
            key: vec![14, 14, 14, 7, 7, CT_ALPHABET_SIZE - 1],
            cipher_factory: InnerCipherFactory::ShiftFactory,
        };
        repeater.encrypt(&mut data);
        println!("{data:?}");
        assert_eq!(data, vec![
            (5 + 14u8).rem_euclid(CT_ALPHABET_SIZE),
            (18 + 14u8).rem_euclid(CT_ALPHABET_SIZE),
            (13 + 14u8).rem_euclid(CT_ALPHABET_SIZE),
            (8 + 7u8).rem_euclid(CT_ALPHABET_SIZE),
            (CT_ALPHABET_SIZE - 1 + 7u8).rem_euclid(CT_ALPHABET_SIZE),
            (8 + CT_ALPHABET_SIZE - 1u8).rem_euclid(CT_ALPHABET_SIZE),
            (8 + 14u8).rem_euclid(CT_ALPHABET_SIZE),
            (8 + 14u8).rem_euclid(CT_ALPHABET_SIZE),
        ]);
    }
}
