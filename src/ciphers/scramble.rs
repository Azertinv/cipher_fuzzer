use crate::ciphers::{InnerCipher, Cipher};
use crate::common::*;

use rand::{
    seq::SliceRandom,
    rngs::StdRng,
    thread_rng,
    RngCore,
    SeedableRng,
};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Scramble {
    seed: u64,
}

#[typetag::serde]
impl Cipher for Scramble {
    fn generate() -> Self where Self: Sized {
        Scramble { seed: thread_rng().next_u64() }
    }

    fn mutation_count(&self) -> u32 {
        0
    }

    fn mutate(&mut self, _: Option<u32>) {
        *self = Self::generate();
    }

    fn encrypt(&self, ct: &mut [u8]) {
        let mut alphabet: Vec<u8> = CT_ALPHABET.to_vec();
        alphabet.shuffle(&mut StdRng::seed_from_u64(self.seed));
        substitute(ct, &alphabet);
    }
}

#[typetag::serde]
impl InnerCipher for Scramble {
    fn from_hint(hint: i32) -> Box<dyn Cipher> where Self: Sized {
        let seed = (hint as u64) * 1337;
        Box::new(Scramble { seed }) as Box<dyn Cipher>
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mutate() {
        let mut data = vec![1, 2, 3, 4];
        let mut data_copy = data.clone();
        assert_eq!(data, data_copy);
        let mut cipher = Scramble{seed: 123456789};
        cipher.encrypt(&mut data);
        cipher.mutate(None);
        cipher.encrypt(&mut data_copy);
        assert_ne!(data, data_copy);
        println!("{data:?}");
        println!("{data_copy:?}");
    }

    #[test]
    fn encrypt() {
        let mut data = vec![1, 2, 3, 4];
        let cipher = Scramble{seed: 123456789};
        cipher.encrypt(&mut data);
        println!("{data:?}");
    }
}
