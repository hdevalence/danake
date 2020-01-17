use curve25519_dalek::{ristretto::RistrettoPoint, scalar::Scalar};
use rand_core::{CryptoRng, RngCore};

/// A rerandomizable MAC tag which attests to the integrity of some message.
#[allow(non_snake_case)]
pub(crate) struct Tag {
    pub(crate) P: RistrettoPoint,
    pub(crate) Q: RistrettoPoint,
}

impl Tag {
    /// Rerandomize the tag to obtain a new, unlinkable tag for the same message.
    pub(crate) fn randomize<R: RngCore + CryptoRng>(&self, mut rng: R) -> Self {
        let r = Scalar::random(&mut rng);
        Tag {
            P: self.P * r,
            Q: self.Q * r,
        }
    }
}
