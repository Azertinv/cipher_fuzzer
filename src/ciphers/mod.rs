pub mod caesar;
pub use caesar::Caesar;
pub mod scramble;
pub use scramble::Scramble;

pub mod cipher_stack;
pub use cipher_stack::CipherStack;

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
