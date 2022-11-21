use crate::{
    common::*,
    ciphers::{
        Cipher,
        Caesar,
        Scramble,
    },
};
use rand::{
    thread_rng,
    Rng,
    seq::SliceRandom,
};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;

fn random_cipher_step() -> Box<dyn Cipher> {
    let ciphers = [
        Caesar::gen,
        Scramble::gen,
    ];
    ciphers.choose(&mut thread_rng()).unwrap()()
}

#[derive(Serialize, Deserialize)]
pub struct CipherStack {
    ciphers: Vec<Box<dyn Cipher>>,
}

impl CipherStack {
    pub fn random(steps: u32) -> Self {
        let ciphers = (0..steps).map(|_| random_cipher_step()).collect();
        CipherStack { ciphers }
    }

    pub fn mutation_count(&self) -> u32 {
        self.ciphers.iter().map(|c| c.mutation_count()).sum()
    }

    pub fn mutate(&mut self, iteration: Option<u32>) {
        if let Some(mut iteration) = iteration {
            let mut i = 0;
            while iteration >= self.ciphers[i].mutation_count() {
                iteration -= self.ciphers[i].mutation_count();
                i += 1;
            }
            self.ciphers[i].mutate(Some(iteration));
        } else {
            let random_index = thread_rng().gen_range(0..self.ciphers.len());
            match thread_rng().gen_range(0..4) {
                // remove
                0 if self.ciphers.len() > 1 => {
                    self.ciphers.remove(random_index);
                },
                // insert
                1 if self.ciphers.len() < 10 => {
                    self.ciphers.insert(random_index, random_cipher_step());
                },
                // replace
                2 => {
                    self.ciphers[random_index] = random_cipher_step();
                },
                // mutate
                _ => {
                    self.ciphers[random_index].mutate(None);
                },
            }
        }
    }

    pub fn encrypt(&self, cts: Cts) -> Cts {
        let mut cts = cts;
        for ct in cts.iter_mut() {
            for cipher in self.ciphers.iter() {
                cipher.encrypt(ct);
            }
        }
        cts
    }

    pub fn load(name: &str) -> std::io::Result<CipherStack> {
        let mut file = File::open(name)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(serde_json::from_str(&content)?)
    }

    pub fn save(&self, name: &str) -> std::io::Result<()> {
        let mut file = File::create(name)?;
        let content = serde_json::to_string(self)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}
