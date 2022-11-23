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

#[derive(Serialize, Deserialize, Debug)]
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
        let ciphers: Vec<Box<dyn Cipher>> = (0..data.len())
            .map(|i| {
                let hint = self.key[i % key_len].into();
                self.cipher_factory.build_from_hint(hint)
            }).collect();
        for i in 0..data.len() {
            ciphers[i].encrypt(&mut data[i..i+1]);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mutate() {
        let repeater = Repeater {
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
        let mut data = "1337 BOI".as_bytes().to_vec();
        let mut repeater = Repeater {
            key: "AAABBB".as_bytes().to_vec(),
            cipher_factory: InnerCipherFactory::ShiftFactory,
        };
        repeater.encrypt(&mut data);
        println!("{data:?}");
        assert_eq!(data, vec![31, 33, 33, 38, 15, 49, 61, 55]);
    }
}
