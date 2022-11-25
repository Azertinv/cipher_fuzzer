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
pub struct CtAutoKeyer {
    iv: u8,
    cipher_factory: InnerCipherFactory,
}

#[typetag::serde]
impl Cipher for CtAutoKeyer {
    fn generate() -> Self where Self: Sized {
        let iv = *CT_ALPHABET.choose(&mut thread_rng()).unwrap();
        let cipher_factory = InnerCipherFactory::random_factory();
        CtAutoKeyer { iv, cipher_factory }
    }

    fn mutation_count(&self) -> u32 {
        0
    }

    fn mutate(&mut self, _: Option<u32>) {
        let mut rng = thread_rng();
        match rng.gen_range(0..1){
            0 => self.iv = *CT_ALPHABET.choose(&mut rng).unwrap(),
            _ => *self = Self::generate(),
        }
    }

    fn encrypt(&self, data: &mut [u8]) {
        if data.is_empty() {
            return;
        }
        let ciphers: Vec<Box<dyn Cipher>> = (0..CT_ALPHABET_SIZE)
            .map(|l| self.cipher_factory.build_from_hint((l).into()))
            .collect();
        ciphers[self.iv as usize].encrypt(&mut data[0..1]);
        for i in 1..data.len() {
            ciphers[data[i - 1] as usize].encrypt(&mut data[i..i+1]);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mutate() {
        let mut autokeyer = CtAutoKeyer {
            iv: 0,
            cipher_factory: InnerCipherFactory::ShiftFactory,
        };
        for _ in 0..10 {
            autokeyer.mutate(None);
        }
        assert!(autokeyer.iv != 0 || autokeyer.cipher_factory != InnerCipherFactory::ShiftFactory);
    }

    #[test]
    fn encrypt() {
        let mut data = "1337 BOI".as_bytes().to_vec();
        let autokeyer = CtAutoKeyer {
            iv: 0,
            cipher_factory: InnerCipherFactory::ShiftFactory,
        };
        autokeyer.encrypt(&mut data);
        assert_eq!(data, vec![49, 17, 68, 40, 72, 55, 51, 41]);
    }
}
