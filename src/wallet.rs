use curve25519_dalek::scalar::Scalar;

use crate::{Epoch, Tag};

/// A wallet token.
pub struct Wallet {
    epoch: Epoch,
    w: u64,
    n: Scalar,
    tag: Tag,
}

mod keys;
pub use keys::{Parameters, Secrets};

/// Issuance protocol states and messages.
pub mod issuance;

/// Topup protocol states and messages.
pub mod topup;

/// Rollover protocol states and messages.
pub mod rollover;
