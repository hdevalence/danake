use curve25519_dalek::{
    ristretto::{CompressedRistretto, RistrettoPoint},
    scalar::Scalar,
    traits::MultiscalarMul,
};
use merlin::Transcript;
use rand_core::{CryptoRng, RngCore};

use crate::{constants, Epoch, Tag};

use super::keys::{Parameters, Secrets};
use super::Wallet;

mod proofs {
    define_proof! {
        client,
        "wallet::topup::client",
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
        Com_w = (w * P + w_blinding * B_blinding),
        Com_w_prime = (w_prime * P + w_blinding * B_blinding),
        V = (w_blinding * X_1 + minus_r_Q * B)
    }

    define_proof! {
        issuer,
        "wallet::topup::issuer",
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
    w_prime: u64,
    n_prime: Scalar,
    d: Scalar,
    w_blinding: Scalar,
    D: RistrettoPoint,
    Enc_w_prime_B: (CompressedRistretto, CompressedRistretto),
    Enc_n_prime_B: (CompressedRistretto, CompressedRistretto),
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
        let B: &RistrettoPoint = &constants::B;

        if self.epoch != parameters.epoch {
            return Err("wrong epoch");
        }

        let tag = self.tag.randomize(&mut rng);

        let w = Scalar::from(self.w);
        let w_prime = Scalar::from(self.w + c);
        let w_blinding = Scalar::random(&mut rng);

        // The commitment to the updated balance w_prime has bases P,
        // B_blinding, so we construct custom pedersen commitment generators to
        // pass to the bulletproofs library.
        let pc_gens = bulletproofs::PedersenGens {
            B: tag.P,
            B_blinding: constants::PG.B_blinding,
        };
        let Com_w = pc_gens.commit(w, w_blinding);
        let Com_w_prime = pc_gens.commit(w_prime, w_blinding);

        let r_Q = Scalar::random(&mut rng);
        let C_Q = tag.Q + r_Q * B;

        let V = w_blinding * parameters.X_1 - r_Q * B;

        let n_prime = Scalar::random(&mut rng);
        let d = Scalar::random(&mut rng);
        let D = d * B;

        let r_w = Scalar::random(&mut rng);
        let Enc_w_prime_B = (r_w * B, (w_prime + r_w * d) * B);

        let r_n = Scalar::random(&mut rng);
        let Enc_n_prime_B = (r_n * B, (n_prime + r_n * d) * B);

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
                B: B,
                B_blinding: &constants::B_BLINDING,
                X_1: &parameters.X_1,
            },
        );

        let (range_proof, _) = bulletproofs::RangeProof::prove_single(
            &constants::BP_GENS,
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
                w_prime: self.w + c,
                n_prime,
                transcript,
                w_blinding,
                D,
                Enc_n_prime_B: (points.Enc_n_prime_B_0, points.Enc_n_prime_B_1),
                Enc_w_prime_B: (points.Enc_w_prime_B_0, points.Enc_w_prime_B_1),
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
#[allow(non_snake_case)]
pub struct Response {
    P: CompressedRistretto,
    Enc_Q: (CompressedRistretto, CompressedRistretto),
    T_1: CompressedRistretto,
    T_2: CompressedRistretto,
    proof: proofs::issuer::CompactProof,
}

impl Secrets {
    #[allow(non_snake_case)]
    pub fn topup<R: RngCore + CryptoRng>(
        &self,
        request: Request,
        mut transcript: Transcript,
        mut rng: R,
    ) -> Result<Response, &'static str> {
        let B: &RistrettoPoint = &constants::B;
        let sk = &self.inner;
        let params = &self.cached_params;

        if params.epoch != request.epoch {
            return Err("wrong epoch");
        }

        // XXX check nullifier

        let Com_w = request.Com_w.decompress().ok_or("bad point")?;
        let C_Q = request.C_Q.decompress().ok_or("bad point")?;
        let P = request.P.decompress().ok_or("bad point")?;
        let D = request.D.decompress().ok_or("bad point")?;

        let V =
            RistrettoPoint::multiscalar_mul(&[sk.x_0 + sk.x_2 * request.n, sk.x_1], &[P, Com_w])
                - C_Q;

        let Com_w_prime = (Com_w + P * Scalar::from(request.c)).compress();

        proofs::client::verify_compact(
            &request.proof,
            &mut transcript,
            proofs::client::VerifyAssignments {
                B: &constants::B_COMPRESSED,
                B_blinding: &constants::B_BLINDING_COMPRESSED,
                Com_w: &request.Com_w,
                Com_w_prime: &Com_w_prime,
                D: &request.D,
                Enc_n_prime_B_0: &request.Enc_n_prime_B.0,
                Enc_n_prime_B_1: &request.Enc_n_prime_B.1,
                Enc_w_prime_B_0: &request.Enc_w_prime_B.0,
                Enc_w_prime_B_1: &request.Enc_w_prime_B.1,
                P: &request.P,
                V: &V.compress(),
                X_1: &params.X_1.compress(),
            },
        )
        .map_err(|_| "client proof failed to verify")?;

        let pc_gens = bulletproofs::PedersenGens {
            B: request.P.decompress().ok_or("bad point")?,
            B_blinding: constants::PG.B_blinding,
        };
        request
            .range_proof
            .verify_single(
                &constants::BP_GENS,
                &pc_gens,
                &mut transcript,
                &Com_w_prime,
                64,
            )
            .map_err(|_| "range proof failed to verify")?;

        let b = Scalar::random(&mut rng);
        let r = Scalar::random(&mut rng);

        let Enc_n_prime_B = (
            request.Enc_n_prime_B.0.decompress().ok_or("bad point")?,
            request.Enc_n_prime_B.1.decompress().ok_or("bad point")?,
        );

        let Enc_w_prime_B = (
            request.Enc_w_prime_B.0.decompress().ok_or("bad point")?,
            request.Enc_w_prime_B.1.decompress().ok_or("bad point")?,
        );

        let P = b * B;
        let Enc_Q = (
            RistrettoPoint::multiscalar_mul(
                &[r, b * sk.x_1, b * sk.x_2],
                &[*B, Enc_w_prime_B.0, Enc_n_prime_B.0],
            ),
            RistrettoPoint::multiscalar_mul(
                &[r, sk.x_0, b * sk.x_1, b * sk.x_2],
                &[D, P, Enc_w_prime_B.1, Enc_n_prime_B.1],
            ),
        );

        use proofs::issuer::*;
        let t_1 = b * sk.x_1;
        let T_1 = b * params.X_1;
        let t_2 = b * sk.x_2;
        let T_2 = b * params.X_2;
        let (proof, points) = prove_compact(
            &mut transcript,
            ProveAssignments {
                b: &b,
                r: &r,
                x_0: &sk.x_0,
                x_1: &sk.x_1,
                x_2: &sk.x_2,
                x_0_blinding: &sk.x_0_blinding,
                t_1: &t_1,
                t_2: &t_2,
                P: &P,
                D: &D,
                Enc_w_prime_B_0: &Enc_w_prime_B.0,
                Enc_w_prime_B_1: &Enc_w_prime_B.1,
                Enc_n_prime_B_0: &Enc_n_prime_B.0,
                Enc_n_prime_B_1: &Enc_n_prime_B.1,
                Enc_Q_0: &Enc_Q.0,
                Enc_Q_1: &Enc_Q.1,
                T_1_a: &T_1,
                T_1_b: &T_1,
                T_2_a: &T_2,
                T_2_b: &T_2,
                X_0: &params.X_0,
                X_1: &params.X_1,
                X_2: &params.X_2,
                B: B,
                B_blinding: &constants::B_BLINDING,
            },
        );

        Ok(Response {
            P: points.P,
            Enc_Q: (points.Enc_Q_0, points.Enc_Q_1),
            T_1: points.T_1_a,
            T_2: points.T_2_a,
            proof,
        })
    }
}

impl AwaitingResponse {
    #[allow(non_snake_case)]
    pub fn verify_response(mut self, response: Response) -> Result<Wallet, &'static str> {
        let P = response.P.decompress().ok_or("bad point")?;

        use proofs::issuer::*;
        verify_compact(
            &response.proof,
            &mut self.transcript,
            VerifyAssignments {
                P: &response.P,
                D: &self.D.compress(),
                Enc_w_prime_B_0: &self.Enc_w_prime_B.0,
                Enc_w_prime_B_1: &self.Enc_w_prime_B.1,
                Enc_n_prime_B_0: &self.Enc_n_prime_B.0,
                Enc_n_prime_B_1: &self.Enc_n_prime_B.1,
                Enc_Q_0: &response.Enc_Q.0,
                Enc_Q_1: &response.Enc_Q.1,
                T_1_a: &response.T_1,
                T_1_b: &response.T_1,
                T_2_a: &response.T_2,
                T_2_b: &response.T_2,
                X_0: &self.parameters.X_0.compress(),
                X_1: &self.parameters.X_1.compress(),
                X_2: &self.parameters.X_2.compress(),
                B: &constants::B_COMPRESSED,
                B_blinding: &constants::B_BLINDING_COMPRESSED,
            },
        )
        .map_err(|_| "issuer proof failed to verify")?;

        let Enc_Q = (
            response.Enc_Q.0.decompress().ok_or("bad point")?,
            response.Enc_Q.1.decompress().ok_or("bad point")?,
        );

        let Q = Enc_Q.1 - self.d * Enc_Q.0;

        Ok(Wallet {
            epoch: self.parameters.epoch,
            tag: Tag { P, Q },
            n: self.n_prime,
            w: self.w_prime,
        })
    }
}
