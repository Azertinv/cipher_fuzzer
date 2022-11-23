pub mod cipher_stack;
pub use cipher_stack::CipherStack;
pub mod inner_cipher;
pub use inner_cipher::{
    InnerCipherFactory,
    InnerCipher,
};

pub mod shift;
pub use shift::Shift;
pub mod scramble;
pub use scramble::Scramble;

pub mod indexer;
pub use indexer::Indexer;
pub mod repeater;
pub use repeater::Repeater;

use rand::{
    thread_rng,
    seq::SliceRandom,
};

pub fn random_cipher_step() -> Box<dyn Cipher> {
    let ciphers = [
        Shift::gen,
        Scramble::gen,
        Indexer::gen,
        Repeater::gen,
    ];
    ciphers.choose(&mut thread_rng()).unwrap()()
}


/// Trait describing a cipher step
#[typetag::serde(tag = "type")]
pub trait Cipher {
    /// Generate the cipher step as a trait object
    fn gen() -> Box<dyn Cipher>
    where
        Self: 'static + Sized
    {
        Box::new(Self::generate()) as Box<dyn Cipher>
    }

    /// Return a randomly generated `Self`
    fn generate() -> Self where Self: Sized;

    /// Returns the number of scheduled mutation possible on this cipher step
    fn mutation_count(&self) -> u32;

    /// Mutate own parameter, implement scheduled mutation based on iteration
    /// If iteration is none, only do unscheduled mutations
    fn mutate(&mut self, iteration: Option<u32>);

    /// Encrypts data in place
    fn encrypt(&self, data: &mut [u8]);
}
