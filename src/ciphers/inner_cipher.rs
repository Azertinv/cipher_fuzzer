use serde::{Serialize, Deserialize};
use rand::{
    thread_rng,
    seq::SliceRandom,
};

use crate::{
    ciphers::{
        Cipher,
        Shift,
        Scramble,
    },
};

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum InnerCipherFactory {
    ShiftFactory,
    ScrambleFactory,
}

impl InnerCipherFactory {
    const CIPHER_FACTORIES: [Self; 2] = [
        Self::ShiftFactory,
        Self::ScrambleFactory,
    ];

    pub fn random_factory() -> InnerCipherFactory {
        *Self::CIPHER_FACTORIES.choose(&mut thread_rng()).unwrap()
    }

    pub fn build_from_hint(&self, hint: i32) -> Box<dyn Cipher> {
        match self {
            Self::ShiftFactory => Shift::from_hint(hint),
            Self::ScrambleFactory => Scramble::from_hint(hint),
        }
    }
}

#[typetag::serde(tag = "type")]
pub trait InnerCipher {
    fn from_hint(hint: i32) -> Box<dyn Cipher> where Self: Sized;
}
