use crate::ciphers::{
    InnerCipherFactory,
    Cipher,
};

use rand::{thread_rng, Rng};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Progressor {
    size: usize,
    increment: i32,
    offset: i32,
    cipher_factory: InnerCipherFactory,
}

#[typetag::serde]
impl Cipher for Progressor {
    fn generate() -> Self where Self: Sized {
        let mut rng = thread_rng();
        Progressor {
            size: rng.gen_range(1..=20),
            increment: rng.gen_range(-2..=2),
            offset: rng.gen_range(-3..=3),
            cipher_factory: InnerCipherFactory::random_factory(),
        }
    }

    fn mutation_count(&self) -> u32 {
        10
    }

    fn mutate(&mut self, iteration: Option<u32>) {
        let mut rng = thread_rng();
        if let Some(choice) = iteration { match choice {
            0 => self.size += 1,
            1 if self.size > 1 => self.size -= 1,
            2 => self.increment += 1,
            3 => self.increment -= 1,
            4 => self.increment = -self.increment,
            5 => self.offset += 1,
            6 => self.offset -= 1,
            7 => self.offset = -self.offset,
            _ => *self = Self::generate(),
        }} else { match rng.gen_range(0..=5) {
            0 => self.size = rng.gen_range(1..=20),
            1 => self.increment = rng.gen_range(-2..=2),
            2 => self.offset = rng.gen_range(-3..=3),
            3 => self.cipher_factory = InnerCipherFactory::random_factory(),
            _ => *self = Self::generate(),
        }}
    }

    fn encrypt(&self, data: &mut [u8]) {
        let ciphers: Vec<Box<dyn Cipher>> = (0..data.len())
            .map(|i| {
                let i = (i / self.size) as i32 * self.increment + self.offset;
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
        let mut cipher = Progressor {
            size: 3,
            offset: 0,
            increment: 1,
            cipher_factory: InnerCipherFactory::ShiftFactory,
        };
        let mut data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        cipher.mutate(Some(0));
        cipher.encrypt(&mut data);
        assert_eq!(data, vec![0, 1, 2, 3, 5, 6, 7, 8, 10, 11]);
        cipher.mutate(Some(1));
        cipher.encrypt(&mut data);
        assert_eq!(data, vec![0, 1, 2, 4, 6, 7, 9, 10, 12, 14]);
        cipher.mutate(Some(1));
        cipher.mutate(Some(2));
        println!("{data:?}");
        println!("{cipher:?}");
        cipher.encrypt(&mut data);
        assert_eq!(data, vec![0, 1, 4, 6, 10, 11, 15, 16, 20, 22]);
    }

    #[test]
    fn encrypt() {
        let cipher = Progressor {
            size: 3,
            offset: 0,
            increment: 1,
            cipher_factory: InnerCipherFactory::ShiftFactory,
        };
        let mut data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        cipher.encrypt(&mut data);
        assert_eq!(data, vec![0, 1, 2, 4, 5, 6, 8, 9, 10, 12]);
    }
}
