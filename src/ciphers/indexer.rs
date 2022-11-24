use crate::ciphers::{
    InnerCipherFactory,
    Cipher,
};

use rand::{thread_rng, Rng};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Indexer {
    offset: i32,
    increment: i32,
    cipher_factory: InnerCipherFactory,
}

#[typetag::serde]
impl Cipher for Indexer {
    fn generate() -> Self where Self: Sized {
        Indexer {
            offset: thread_rng().gen_range(-10..=10),
            increment: thread_rng().gen_range(-2..=2),
            cipher_factory: InnerCipherFactory::random_factory()
        }
    }

    fn mutation_count(&self) -> u32 {
        6
    }

    fn mutate(&mut self, iteration: Option<u32>) {
        match iteration {
            Some(0) => self.offset += 1,
            Some(1) => self.offset -= 1,
            Some(2) => self.offset = -self.offset,
            Some(3) => self.increment += 1,
            Some(4) => self.increment -= 1,
            Some(5) => self.increment = -self.increment,
            _ => *self = Self::generate(),
        }
    }

    fn encrypt(&self, data: &mut [u8]) {
        let ciphers: Vec<Box<dyn Cipher>> = (0..data.len())
            .map(|i| {
                let i = i as i32 * self.increment + self.offset;
                self.cipher_factory.build_from_hint(i)
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
        let mut indexer = Indexer {
            offset: 0,
            increment: 1,
            cipher_factory: InnerCipherFactory::ShiftFactory,
        };
        let mut data = vec![0, 1, 2, 5, 6, 7];
        indexer.mutate(Some(0));
        indexer.encrypt(&mut data);
        assert_eq!(data, vec![1, 3, 5, 9, 11, 13]);
        indexer.mutate(Some(1));
        indexer.encrypt(&mut data);
        assert_eq!(data, vec![1, 4, 7, 12, 15, 18]);
        indexer.mutate(Some(1));
        indexer.mutate(Some(2));
        println!("{data:?}");
        println!("{indexer:?}");
        indexer.encrypt(&mut data);
        println!("{data:?}");
        assert_eq!(data, vec![2, 6, 10, 16, 20, 24]);
    }

    #[test]
    fn encrypt() {
        let indexer = Indexer {
            offset: 0,
            increment: 1,
            cipher_factory: InnerCipherFactory::ShiftFactory,
        };
        let mut data = vec![0, 1, 2, 5, 6, 7];
        indexer.encrypt(&mut data);
        assert_eq!(data, vec![0, 2, 4, 8, 10, 12]);
    }
}
