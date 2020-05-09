use chrono;
use curve25519_dalek::{
    ristretto::CompressedRistretto, ristretto::RistrettoPoint, scalar::Scalar,
    traits::MultiscalarMul,
};
use merlin::Transcript;
use rand_core::{CryptoRng, RngCore};

use crate::{constants, Epoch, EpochState, Tag};

use super::keys::{Parameters, Secrets};
use super::Wallet;

mod proofs {
    define_proof! {
        client,
        "WalletRollover_Client",
        (
            d,
            w,
            w_blinding,
            n_prime,
            minus_r_Q,
            r_Q,
            r_w,
            r_n
        ),
        (
            D,
            Enc_w_B_0,
            Enc_w_B_1,
            Enc_n_prime_B_0,
            Enc_n_prime_B_1,
            P,
            V,
            Com_w
        ),
        (
            B,
            B_blinding,
            X_1
        )
        :
        Enc_n_prime_B_0 = (r_n * B),
        Enc_n_prime_B_1 = (n_prime * B + r_n * D),
        Enc_w_B_0 = (r_w * B),
        Enc_w_B_1 = (w * B + r_w * D),
        Com_w = (w * P + w_blinding * B_blinding),
        V = (w_blinding * X_1 + minus_r_Q * B)
    }

    define_proof! {
        issuer,
        "WalletRollover_Issuer",
        (
            b,
            r,
            x_0,
            x_1,
            x_2,
            x_0_blinding,
            x_prime_0,
            x_prime_1,
            x_prime_2,
            x_prime_0_blinding,
            t_1,
            t_2
        ),
        (
            P,
            D,
            Enc_w_B_0,
            Enc_w_B_1,
            Enc_n_prime_B_0,
            Enc_n_prime_B_1,
            Enc_Q_0,
            Enc_Q_1,
            T_1_a,
            T_1_b,
            T_2_a,
            T_2_b
        ),
        (
            X_0,
            X_1,
            X_2,
            X_prime_0,
            X_prime_1,
            X_prime_2,
            B,
            B_blinding
        )
        :
        X_0 = (x_0 * B + x_0_blinding * B_blinding),
        X_1 = (x_1 * B_blinding),
        X_2 = (x_2 * B_blinding),
        X_prime_0 = (x_prime_0 * B + x_prime_0_blinding * B_blinding),
        X_prime_1 = (x_prime_1 * B_blinding),
        X_prime_2 = (x_prime_2 * B_blinding),
        P = (b * B),
        T_1_a = (b * X_prime_1),
        T_1_b = (t_1 * B_blinding),
        T_2_a = (b * X_prime_2),
        T_2_b = (t_2 * B_blinding),
        Enc_Q_0 = (r * B + t_1 * Enc_w_B_0 + t_2 * Enc_n_prime_B_0),
        Enc_Q_1 = (x_prime_0 * P + r * D + t_1 * Enc_w_B_1 + t_2 * Enc_n_prime_B_1)
    }
}

/// A request for wallet rollover.
#[allow(non_snake_case)]
pub struct Request {
    epoch: Epoch,
    new_epoch: Epoch,
    n: Scalar,
    D: CompressedRistretto,
    Enc_n_prime_B: (CompressedRistretto, CompressedRistretto),
    Enc_w_B: (CompressedRistretto, CompressedRistretto),
    Com_w: CompressedRistretto,
    P: CompressedRistretto,
    C_Q: CompressedRistretto,
    proof: proofs::client::CompactProof,
}

/// State held by the client while awaiting a wallet rollover response.
#[allow(non_snake_case)]
pub struct AwaitingResponse {
    old_parameters: Parameters,
    new_parameters: Parameters,
    transcript: Transcript,
    n_prime: Scalar,
    w: u64,
    d: Scalar,
    D: RistrettoPoint,
    Enc_w_B: (CompressedRistretto, CompressedRistretto),
    Enc_n_prime_B: (CompressedRistretto, CompressedRistretto),
}

impl Wallet {
    #[allow(non_snake_case)]
    pub fn request_rollover<R: RngCore + CryptoRng>(
        self,
        old_parameters: &Parameters,
        new_parameters: &Parameters,
        mut transcript: Transcript,
        mut rng: R,
    ) -> Result<(AwaitingResponse, Request), &'static str> {
        let B: &RistrettoPoint = &constants::B;

        // Step 1.1: Old and new parameters are currently passed in.

        // Step 1.2
        let tag: Tag = self.tag.randomize(&mut rng);

        // Step 1.3
        let w: Scalar = Scalar::from(self.w);
        let w_blinding = Scalar::random(&mut rng);

        let pc_gens = bulletproofs::PedersenGens {
            B: tag.P,
            B_blinding: constants::PG.B_blinding,
        };
        let Com_w = pc_gens.commit(w, w_blinding);

        // Step 1.4
        let r_Q = Scalar::random(&mut rng);
        let C_Q = tag.Q + r_Q * B;

        // Step 1.5
        let V = w_blinding * old_parameters.X_1 - r_Q * B;

        // Step 1.6
        let n_prime: Scalar = Scalar::random(&mut rng);

        // Step 1.7
        let d = Scalar::random(&mut rng);
        let D: RistrettoPoint = d * B;

        // Step 1.8
        let r_n = Scalar::random(&mut rng);
        let r_w = Scalar::random(&mut rng);
        let Enc_w_B = (r_w * B, (w + r_w * d) * B);
        let Enc_n_prime_B = (r_n * B, (n_prime + r_n * d) * B);

        // Step 1.9
        use proofs::client::*;

        let (proof, points) = prove_compact(
            &mut transcript,
            ProveAssignments {
                d: &d,
                w: &w,
                w_blinding: &w_blinding,
                n_prime: &n_prime,
                r_Q: &r_Q,
                r_w: &r_w,
                r_n: &r_n,
                D: &D,
                Enc_w_B_0: &Enc_w_B.0,
                Enc_w_B_1: &Enc_w_B.1,
                Enc_n_prime_B_0: &Enc_n_prime_B.0,
                Enc_n_prime_B_1: &Enc_n_prime_B.1,
                P: &tag.P,
                V: &V,
                Com_w: &Com_w,
                B: &B,
                B_blinding: &constants::B_BLINDING,
                X_1: &old_parameters.X_1,
                minus_r_Q: &(-r_Q),
            },
        );

        // Step 1.10
        Ok((
            AwaitingResponse {
                old_parameters: old_parameters.clone(),
                new_parameters: new_parameters.clone(),
                transcript,
                w: self.w,
                n_prime,
                d,
                D,
                Enc_w_B: (points.Enc_w_B_0, points.Enc_w_B_1),
                Enc_n_prime_B: (points.Enc_n_prime_B_0, points.Enc_n_prime_B_1),
            },
            Request {
                epoch: old_parameters.epoch,
                new_epoch: new_parameters.epoch,
                n: self.n,
                D: points.D,
                Enc_n_prime_B: (points.Enc_n_prime_B_0, points.Enc_n_prime_B_1),
                Enc_w_B: (points.Enc_w_B_0, points.Enc_w_B_1),
                Com_w: points.Com_w,
                P: points.P,
                C_Q: C_Q.compress(),
                proof,
            },
        ))
    }
}

/// A response to a wallet rollover request.
#[allow(non_snake_case)]
pub struct Response {
    P: CompressedRistretto,
    Enc_Q: (CompressedRistretto, CompressedRistretto),
    T_1: CompressedRistretto,
    T_2: CompressedRistretto,
    proof: proofs::issuer::CompactProof,
}

impl Request {
    #[allow(non_snake_case)]
    pub fn rollover<R: RngCore + CryptoRng>(
        &self,
        old_secret: Secrets,
        new_secret: Secrets,
        mut transcript: Transcript,
        mut rng: R,
        mut check_and_update_nullifier: impl FnMut([u8; 32]) -> bool,
    ) -> Result<Response, &'static str> {
        // Step 2.1
        let old_parameters = old_secret.cached_params;
        let new_parameters = new_secret.cached_params;

        let time_req_processing = chrono::Utc::now();
        let old_epoch_state = self.epoch.state_at(time_req_processing);
        match old_epoch_state {
            EpochState::Active => {}
            EpochState::Primary => {}
            EpochState::Rollover => {}
            _ => return Err("old epoch not in Active, Primary, or Rollover state"),
        }

        let new_epoch_state = self.new_epoch.state_at(time_req_processing);
        match new_epoch_state {
            EpochState::Active => {}
            EpochState::Primary => {}
            _ => return Err("new epoch not in Active or Primary state"),
        }

        // Step 2.2
        if !check_and_update_nullifier(self.n.to_bytes()) {
            return Err("nullifier is in wallet nullifier set");
        }

        // Step 2.3
        let Com_w = self.Com_w.decompress().ok_or("bad point")?;
        let C_Q = self.C_Q.decompress().ok_or("bad point")?;
        let old_sk = old_secret.inner;
        let P = self.P.decompress().ok_or("bad point")?;
        let V = RistrettoPoint::multiscalar_mul(
            &[old_sk.x_0 + old_sk.x_2 * self.n, old_sk.x_1],
            &[P, Com_w],
        ) - C_Q;

        // Step 2.4
        proofs::client::verify_compact(
            &self.proof,
            &mut transcript,
            proofs::client::VerifyAssignments {
                D: &self.D,
                Enc_w_B_0: &self.Enc_w_B.0,
                Enc_w_B_1: &self.Enc_w_B.1,
                Enc_n_prime_B_0: &self.Enc_n_prime_B.0,
                Enc_n_prime_B_1: &self.Enc_n_prime_B.1,
                P: &self.P,
                V: &V.compress(),
                Com_w: &self.Com_w,
                B: &constants::B_COMPRESSED,
                B_blinding: &constants::B_BLINDING_COMPRESSED,
                X_1: &old_parameters.X_1.compress(),
            },
        )
        .map_err(|_| "client proof failed to verify")?;

        // Step 2.5
        let b = Scalar::random(&mut rng);
        let B: &RistrettoPoint = &constants::B;
        let P = b * B;

        // Step 2.6
        let r = Scalar::random(&mut rng);

        let Enc_n_prime_B = (
            self.Enc_n_prime_B.0.decompress().ok_or("bad point")?,
            self.Enc_n_prime_B.1.decompress().ok_or("bad point")?,
        );

        let Enc_w_B = (
            self.Enc_w_B.0.decompress().ok_or("bad point")?,
            self.Enc_w_B.1.decompress().ok_or("bad point")?,
        );
        let D = self.D.decompress().ok_or("bad point")?;

        let new_sk = new_secret.inner;
        let Enc_Q = (
            RistrettoPoint::multiscalar_mul(
                &[r, b * new_sk.x_1, b * new_sk.x_2],
                &[*B, Enc_w_B.0, Enc_n_prime_B.0],
            ),
            RistrettoPoint::multiscalar_mul(
                &[r, new_sk.x_0, b * new_sk.x_1, b * new_sk.x_2],
                &[D, P, Enc_w_B.1, Enc_n_prime_B.1],
            ),
        );

        // Step 2.7
        use proofs::issuer::*;
        let t_1 = b * new_sk.x_1;
        let T_1 = b * new_parameters.X_1;
        let t_2 = b * new_sk.x_2;
        let T_2 = b * new_parameters.X_2;
        let (proof, points) = prove_compact(
            &mut transcript,
            ProveAssignments {
                b: &b,
                r: &r,
                x_0: &old_sk.x_0,
                x_1: &old_sk.x_1,
                x_2: &old_sk.x_2,
                x_0_blinding: &old_sk.x_0_blinding,
                x_prime_0: &new_sk.x_0,
                x_prime_1: &new_sk.x_1,
                x_prime_2: &new_sk.x_2,
                x_prime_0_blinding: &new_sk.x_0_blinding,
                t_1: &t_1,
                t_2: &t_2,
                P: &P,
                D: &D,
                Enc_w_B_0: &Enc_w_B.0,
                Enc_w_B_1: &Enc_w_B.1,
                Enc_n_prime_B_0: &Enc_n_prime_B.0,
                Enc_n_prime_B_1: &Enc_n_prime_B.1,
                Enc_Q_0: &Enc_Q.0,
                Enc_Q_1: &Enc_Q.1,
                X_0: &old_parameters.X_0,
                X_1: &old_parameters.X_1,
                X_2: &old_parameters.X_2,
                X_prime_0: &new_parameters.X_0,
                X_prime_1: &new_parameters.X_1,
                X_prime_2: &new_parameters.X_2,
                B: B,
                B_blinding: &constants::B_BLINDING,
                T_1_a: &T_1,
                T_1_b: &T_1,
                T_2_a: &T_2,
                T_2_b: &T_2,
            },
        );

        // Step 2.8
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
        // Step 3.1
        let P = response.P.decompress().ok_or("bad point")?;

        use proofs::issuer::*;
        verify_compact(
            &response.proof,
            &mut self.transcript,
            VerifyAssignments {
                P: &response.P,
                D: &self.D.compress(),
                Enc_w_B_0: &self.Enc_w_B.0,
                Enc_w_B_1: &self.Enc_w_B.1,
                Enc_n_prime_B_0: &self.Enc_n_prime_B.0,
                Enc_n_prime_B_1: &self.Enc_n_prime_B.1,
                Enc_Q_0: &response.Enc_Q.0,
                Enc_Q_1: &response.Enc_Q.1,
                T_1_a: &response.T_1,
                T_1_b: &response.T_1,
                T_2_a: &response.T_2,
                T_2_b: &response.T_2,
                X_0: &self.old_parameters.X_0.compress(),
                X_1: &self.old_parameters.X_1.compress(),
                X_2: &self.old_parameters.X_2.compress(),
                X_prime_0: &self.new_parameters.X_0.compress(),
                X_prime_1: &self.new_parameters.X_1.compress(),
                X_prime_2: &self.new_parameters.X_2.compress(),
                B: &constants::B_COMPRESSED,
                B_blinding: &constants::B_BLINDING_COMPRESSED,
            },
        )
        .map_err(|_| "issuer proof failed to verify")?;

        // Step 3.2
        let Enc_Q = (
            response.Enc_Q.0.decompress().ok_or("bad point")?,
            response.Enc_Q.1.decompress().ok_or("bad point")?,
        );

        let Q = Enc_Q.1 - self.d * Enc_Q.0;

        Ok(Wallet {
            epoch: self.new_parameters.epoch,
            tag: Tag { P, Q },
            n: self.n_prime,
            w: self.w,
        })
    }
}
