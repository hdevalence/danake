use curve25519_dalek::scalar::Scalar;

use crate::{Epoch, Tag};

/// A wallet token.
pub struct Wallet {
    epoch: Epoch,
    w: u64,
    n: Scalar,
    tag: Tag,
}

mod issuance;
mod keys;
mod proofs;

pub use issuance::{AwaitingIssuance, IssuanceRequest, IssuanceResponse};
pub use keys::{IssuanceParameters, IssuanceSecret};
