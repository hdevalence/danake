use curve25519_dalek::{
    ristretto::{CompressedRistretto, RistrettoPoint},
    scalar::Scalar,
    traits::MultiscalarMul,
};
use merlin::Transcript;
use rand_core::{CryptoRng, RngCore};

use crate::{Epoch, Tag};

use super::keys::{Parameters, Secrets};
use super::Wallet;

mod proofs {
    define_proof! {
        client,
        "WalletTopup_Client",
        (
            d,
            w,
            w_prime,
            w_blinding,
            n_prime,
            minus_r_Q,
            r_w,
            r_n
        ),
        (
            D,
            Enc_w_prime_B_0,
            Enc_w_prime_B_1,
            Enc_n_prime_B_0,
            Enc_n_prime_B_1,
            P,
            V,
            Com_w,
            Com_w_prime
        ),
        (
            B,
            B_blinding,
            X_1
        )
        :
        D = (d * B),
        Enc_n_prime_B_0 = (r_n * B),
        Enc_n_prime_B_1 = (n_prime * B + r_n * D),
        Enc_w_prime_B_0 = (r_w * B),
        Enc_w_prime_B_1 = (w_prime * B + r_w * D),
        Com_w = (w * B + w_blinding * B_blinding),
        Com_w_prime = (w_prime * B + w_blinding * B_blinding),
        V = (w_blinding * X_1 + minus_r_Q * B)
    }

    define_proof! {
        issuer,
        "WalletTopup_Issuer",
        (
            b,
            r,
            x_0,
            x_1,
            x_2,
            x_0_blinding,
            t_1,
            t_2
        ),
        (
            P,
            D,
            Enc_w_prime_B_0,
            Enc_w_prime_B_1,
            Enc_n_prime_B_0,
            Enc_n_prime_B_1,
            Enc_Q_0,
            Enc_Q_1,
            T_1_a,
            T_1_b,
            T_2_a,
            T_2_b
        ),
        (X_0, X_1, X_2, B, B_blinding)
        :
        X_0 = (x_0 * B + x_0_blinding * B_blinding),
        X_1 = (x_1 * B_blinding),
        X_2 = (x_2 * B_blinding),
        P = (b * B),
        T_1_a = (b * X_1),
        T_1_b = (t_1 * B_blinding),
        T_2_a = (b * X_2),
        T_2_b = (t_2 * B_blinding),
        Enc_Q_0 = (r * B + t_1 * Enc_w_prime_B_0 + t_2 * Enc_n_prime_B_0),
        Enc_Q_1 = (x_0 * P + r * D + t_1 * Enc_w_prime_B_1 + t_2 * Enc_n_prime_B_1)
    }
}

/// A request for wallet topup.
#[derive(Clone)]
#[allow(non_snake_case)]
pub struct Request {
    epoch: Epoch,
    c: u64,
    n: Scalar,
    D: CompressedRistretto,
    Enc_n_prime_B: (CompressedRistretto, CompressedRistretto),
    Enc_w_prime_B: (CompressedRistretto, CompressedRistretto),
    Com_w: CompressedRistretto,
    P: CompressedRistretto,
    C_Q: CompressedRistretto,
    proof: proofs::client::CompactProof,
    range_proof: bulletproofs::RangeProof,
}

/// State held by the client while awaiting a topup response.
#[derive(Clone)]
#[allow(non_snake_case)]
pub struct AwaitingResponse {
    parameters: Parameters,
    transcript: Transcript,
    d: Scalar,
    w_blinding: Scalar,
}

impl Wallet {
    /// Request a topup, consuming this credential and generating a topup request
    /// message together with the client state needed to verify a response with a
    /// new wallet credential.
    #[allow(non_snake_case)]
    pub fn request_topup<R: RngCore + CryptoRng>(
        self,
        c: u64,
        parameters: &Parameters,
        mut transcript: Transcript,
        mut rng: R,
    ) -> Result<(AwaitingResponse, Request), &'static str> {
        if self.epoch != parameters.epoch {
            return Err("wrong epoch");
        }
        let pg = bulletproofs::PedersenGens::default();

        let tag = self.tag.randomize(&mut rng);

        let w = Scalar::from(self.w);
        let w_prime = Scalar::from(self.w + c);
        let w_blinding = Scalar::random(&mut rng);
        let Com_w = pg.commit(w, w_blinding);
        let Com_w_prime = pg.commit(w_prime, w_blinding);

        let r_Q = Scalar::random(&mut rng);
        let C_Q = tag.Q - r_Q * pg.B;

        let V = w_blinding * parameters.X_1 - r_Q * pg.B;

        let n_prime = Scalar::random(&mut rng);
        let d = Scalar::random(&mut rng);
        let D = d * pg.B;

        let r_w = Scalar::random(&mut rng);
        let Enc_w_prime_B = (r_w * pg.B, (w_prime + r_w * d) * pg.B);

        let r_n = Scalar::random(&mut rng);
        let Enc_n_prime_B = (r_n * pg.B, (n_prime + r_n * d) * pg.B);

        use proofs::client::*;

        let (proof, points) = prove_compact(
            &mut transcript,
            ProveAssignments {
                d: &d,
                w: &w,
                w_prime: &w_prime,
                w_blinding: &w_blinding,
                n_prime: &n_prime,
                minus_r_Q: &(-r_Q),
                r_w: &r_w,
                r_n: &r_n,
                D: &D,
                Enc_n_prime_B_0: &Enc_n_prime_B.0,
                Enc_n_prime_B_1: &Enc_n_prime_B.1,
                Enc_w_prime_B_0: &Enc_w_prime_B.0,
                Enc_w_prime_B_1: &Enc_w_prime_B.1,
                P: &tag.P,
                V: &V,
                Com_w: &Com_w,
                Com_w_prime: &Com_w_prime,
                B: &pg.B,
                B_blinding: &pg.B_blinding,
                X_1: &parameters.X_1,
            },
        );

        // XXX extract this into constants
        let bp_gens = bulletproofs::BulletproofGens::new(64, 1);
        let pc_gens = bulletproofs::PedersenGens {
            B: tag.P,
            B_blinding: pg.B,
        };
        let (range_proof, _) = bulletproofs::RangeProof::prove_single(
            &bp_gens,
            &pc_gens,
            &mut transcript,
            self.w + c,
            &w_blinding,
            64,
        )
        .map_err(|_| "range proof failed")?;

        Ok((
            AwaitingResponse {
                parameters: parameters.clone(),
                d,
                transcript,
                w_blinding,
            },
            Request {
                epoch: self.epoch,
                c,
                n: self.n,
                D: points.D,
                Enc_n_prime_B: (points.Enc_n_prime_B_0, points.Enc_n_prime_B_1),
                Enc_w_prime_B: (points.Enc_w_prime_B_0, points.Enc_w_prime_B_1),
                Com_w: points.Com_w,
                P: points.P,
                C_Q: C_Q.compress(),
                proof,
                range_proof,
            },
        ))
    }
}

/// A response to a topup request.
pub struct Response {
    // XXX
}

impl Secrets {
    pub fn topup<R: RngCore + CryptoRng>(
        &self,
        request: Request,
        mut transcript: Transcript,
        mut rng: R,
    ) -> Result<Response, &'static str> {
        unimplemented!();
    }
}

impl AwaitingResponse {
    pub fn verify_response(self, response: Response) -> Result<Wallet, &'static str> {
        unimplemented!();
    }
}
