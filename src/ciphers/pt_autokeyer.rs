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
pub struct PtAutoKeyer {
    iv: u8,
    cipher_factory: InnerCipherFactory,
    reverse: bool,
}

#[typetag::serde]
impl Cipher for PtAutoKeyer {
    fn generate() -> Self where Self: Sized {
        let mut rng = thread_rng();
        let iv = *CT_ALPHABET.choose(&mut rng).unwrap();
        let cipher_factory = InnerCipherFactory::random_factory();
        let reverse = rng.gen_bool(0.5);
        PtAutoKeyer { iv, cipher_factory, reverse}
    }

    fn mutation_count(&self) -> u32 {
        1
    }

    fn mutate(&mut self, iteration: Option<u32>) {
        if iteration.is_some() {
            self.reverse = !self.reverse;
        } else {
            let mut rng = thread_rng();
            match rng.gen_range(0..2){
                0 => self.iv = *CT_ALPHABET.choose(&mut rng).unwrap(),
                1 => self.cipher_factory = InnerCipherFactory::random_factory(),
                _ => *self = Self::generate(),
            }
        }
    }

    fn encrypt(&self, data: &mut [u8]) {
        if data.is_empty() {
            return;
        }
        let ciphers: Vec<Box<dyn Cipher>> = (0..CT_ALPHABET_SIZE)
            .map(|l| self.cipher_factory.build_from_hint((l).into()))
            .collect();
        let data_len = data.len();
        if !self.reverse {
            for i in (1..data_len).rev() {
                ciphers[data[i - 1] as usize].encrypt(&mut data[i..i+1]);
            }
            ciphers[self.iv as usize].encrypt(&mut data[0..1]);
        } else {
            for i in 0..data_len-1 {
                ciphers[data[i + 1] as usize].encrypt(&mut data[i..i+1]);
            }
            ciphers[self.iv as usize].encrypt(&mut data[data_len-1..data_len]);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mutate() {
        let mut autokeyer = PtAutoKeyer {
            iv: 0,
            cipher_factory: InnerCipherFactory::ShiftFactory,
            reverse: false,
        };
        for _ in 0..10 {
            autokeyer.mutate(None);
        }
        assert!(autokeyer.iv != 0 || autokeyer.cipher_factory != InnerCipherFactory::ShiftFactory);
    }

    #[test]
    fn encrypt() {
        let mut data = vec![5, 18, 13, 8, 1, 8, 8, 8];
        let autokeyer = PtAutoKeyer {
            iv: 5,
            cipher_factory: InnerCipherFactory::ShiftFactory,
            reverse: false,
        };
        autokeyer.encrypt(&mut data);
        let n0: u8 = (5u8+5).rem_euclid(CT_ALPHABET_SIZE);
        let n1 = (5u8+18).rem_euclid(CT_ALPHABET_SIZE);
        let n2 = (18u8+13).rem_euclid(CT_ALPHABET_SIZE);
        let n3 = (13u8+8).rem_euclid(CT_ALPHABET_SIZE);
        let n4 = (8u8+1).rem_euclid(CT_ALPHABET_SIZE);
        let n5 = (1u8+8).rem_euclid(CT_ALPHABET_SIZE);
        let n6 = (8u8+8).rem_euclid(CT_ALPHABET_SIZE);
        let n7 = (8u8+8).rem_euclid(CT_ALPHABET_SIZE);
        assert_eq!(data, vec![n0, n1, n2, n3, n4, n5, n6, n7]);
    }

    #[test]
    fn reverse() {
        let mut data = vec![5, 18, 13, 8, 1, 8, 8, 8];
        let autokeyer = PtAutoKeyer {
            iv: 13,
            cipher_factory: InnerCipherFactory::ShiftFactory,
            reverse: true,
        };
        autokeyer.encrypt(&mut data);
        let n7: u8 = (13u8+8).rem_euclid(CT_ALPHABET_SIZE);
        let n6 = (8u8+8).rem_euclid(CT_ALPHABET_SIZE);
        let n5 = (8u8+8).rem_euclid(CT_ALPHABET_SIZE);
        let n4 = (8u8+1).rem_euclid(CT_ALPHABET_SIZE);
        let n3 = (1u8+8).rem_euclid(CT_ALPHABET_SIZE);
        let n2 = (8u8+13).rem_euclid(CT_ALPHABET_SIZE);
        let n1 = (13u8+18).rem_euclid(CT_ALPHABET_SIZE);
        let n0 = (18u8+5).rem_euclid(CT_ALPHABET_SIZE);
        assert_eq!(data, vec![n0, n1, n2, n3, n4, n5, n6, n7]);
    }
}
