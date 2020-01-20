use bulletproofs::{BulletproofGens, PedersenGens};
use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref B: RistrettoPoint = PedersenGens::default().B;
    pub static ref B_COMPRESSED: CompressedRistretto = PedersenGens::default().B.compress();
    pub static ref B_BLINDING: RistrettoPoint = PedersenGens::default().B_blinding;
    pub static ref B_BLINDING_COMPRESSED: CompressedRistretto =
        PedersenGens::default().B_blinding.compress();
    pub static ref BP_GENS: BulletproofGens = BulletproofGens::new(64, 1);
    pub static ref PG: PedersenGens = PedersenGens::default();
}
