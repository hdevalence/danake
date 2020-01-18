use bulletproofs::PedersenGens;

use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;

use rand_core::{CryptoRng, RngCore};

use crate::Epoch;

/// Public parameters for a wallet issuer.
///
/// These are used by the client to prepare presentation proofs and to ensure
/// that the client is using the same parameters as all otheer clients,
/// preventing key partitioning attacks.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[allow(non_snake_case)]
pub struct Parameters {
    pub(super) X_0: RistrettoPoint,
    pub(super) X_1: RistrettoPoint,
    pub(super) X_2: RistrettoPoint,
    pub(super) epoch: Epoch,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(super) struct Inner {
    pub(super) x_0: Scalar,
    pub(super) x_1: Scalar,
    pub(super) x_2: Scalar,
    pub(super) x_0_blinding: Scalar,
    pub(super) epoch: Epoch,
}

/// Secret key material for a wallet issuer.
///
/// Held by the issuer and used to issue and verify credentials.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Secrets {
    pub(super) inner: Inner,
    pub(super) cached_params: Parameters,
}

impl Inner {
    fn parameters(&self) -> Parameters {
        let pg = PedersenGens::default();
        Parameters {
            X_0: pg.commit(self.x_0, self.x_0_blinding),
            X_1: &pg.B_blinding * &self.x_1,
            X_2: &pg.B_blinding * &self.x_2,
            epoch: self.epoch,
        }
    }
}

impl Secrets {
    pub fn new<R: RngCore + CryptoRng>(epoch: Epoch, mut rng: R) -> Secrets {
        // XXX expand from a seed?
        let inner = Inner {
            epoch,
            x_0: Scalar::random(&mut rng),
            x_1: Scalar::random(&mut rng),
            x_2: Scalar::random(&mut rng),
            x_0_blinding: Scalar::random(&mut rng),
        };
        Secrets {
            inner,
            cached_params: inner.parameters(),
        }
    }
}

impl<'a> From<&'a Secrets> for Parameters {
    fn from(secret: &'a Secrets) -> Parameters {
        secret.cached_params.clone()
    }
}
