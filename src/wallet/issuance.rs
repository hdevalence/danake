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
        "wallet::issuance::client",
        (d, n, r),
        (D, Enc_nB_0, Enc_nB_1),
        (B)
        :
        D = (d * B),
        Enc_nB_0 = (r * B),
        Enc_nB_1 = (n * B + r * D)
    }

    define_proof! {
        issuer,
        "wallet::issuance::issuer",
        (b, r, x_0, x_1, x_2, x_0_blinding, t_2),
        (P, wP, D, Enc_nB_0, Enc_nB_1, Enc_Q_0, Enc_Q_1, T_2_a, T_2_b),
        (X_0, X_1, X_2, B, B_blinding)
        :
        X_0 = (x_0 * B + x_0_blinding * B_blinding),
        X_1 = (x_1 * B_blinding),
        X_2 = (x_2 * B_blinding),
        P = (b * B),
        T_2_a = (b * X_2),
        T_2_b = (t_2 * B_blinding),
        Enc_Q_0 = (r * B + t_2 * Enc_nB_0),
        Enc_Q_1 = (x_0 * P + x_1 * wP + r * D + t_2 * Enc_nB_1)
    }
}

/// A request for issuance of a wallet credential.
#[derive(Clone)]
#[allow(non_snake_case)]
pub struct Request {
    w: u64,
    epoch: Epoch,
    D: CompressedRistretto,
    Enc_nB: (CompressedRistretto, CompressedRistretto),
    proof: proofs::client::CompactProof,
}

/// State held by the client while awaiting an issuance response.
#[allow(non_snake_case)]
pub struct AwaitingResponse {
    parameters: Parameters,
    transcript: Transcript,
    w: u64,
    n: Scalar,
    d: Scalar,
    D: RistrettoPoint,
    Enc_nB: (CompressedRistretto, CompressedRistretto),
}

impl Wallet {
    /// Request issuance of a wallet credential, generating an issuance request
    /// message together with the client state needed to verify a response from
    /// the issuer.
    #[allow(non_snake_case)]
    pub fn request_issuance<R: RngCore + CryptoRng>(
        w: u64,
        parameters: &Parameters,
        mut transcript: Transcript,
        mut rng: R,
    ) -> (AwaitingResponse, Request) {
        let B: &RistrettoPoint = &constants::B;

        let n = Scalar::random(&mut rng);
        let d = Scalar::random(&mut rng);
        let r = Scalar::random(&mut rng);

        let D = &d * B;
        let Enc_nB = (B * r, B * (n + r * d));

        use proofs::client::*;

        // XXX zkp API should take an RNG
        let (proof, points) = prove_compact(
            &mut transcript,
            ProveAssignments {
                d: &d,
                n: &n,
                r: &r,
                D: &D,
                Enc_nB_0: &Enc_nB.0,
                Enc_nB_1: &Enc_nB.1,
                B: B,
            },
        );

        (
            AwaitingResponse {
                // XXX avoid this clone
                parameters: parameters.clone(),
                transcript,
                w,
                n,
                d,
                D,
                Enc_nB: (points.Enc_nB_0, points.Enc_nB_1),
            },
            Request {
                w,
                epoch: parameters.epoch,
                D: points.D,
                Enc_nB: (points.Enc_nB_0, points.Enc_nB_1),
                proof,
            },
        )
    }
}

/// A response to a wallet issuance request.
#[derive(Clone)]
#[allow(non_snake_case)]
pub struct Response {
    P: CompressedRistretto,
    Enc_Q: (CompressedRistretto, CompressedRistretto),
    T_2: CompressedRistretto,
    proof: proofs::issuer::CompactProof,
}

impl Secrets {
    /// Issues a wallet credential in response to an issuance request.
    ///
    /// This function is solely responsible for the issuance itself and not for
    /// application policy (e.g., checking that the requested amount is valid).
    ///
    /// The response should be returned to the client, who can process it.
    #[allow(non_snake_case)]
    pub fn issue<R: RngCore + CryptoRng>(
        &self,
        request: Request,
        mut transcript: Transcript,
        mut rng: R,
    ) -> Result<Response, &'static str> {
        let B: &RistrettoPoint = &constants::B;

        let sk = &self.inner;
        let params = &self.cached_params;

        if request.epoch != params.epoch {
            return Err("IssuanceRequest has wrong epoch for this IssuanceSecret");
        }

        proofs::client::verify_compact(
            &request.proof,
            &mut transcript,
            proofs::client::VerifyAssignments {
                D: &request.D,
                Enc_nB_0: &request.Enc_nB.0,
                Enc_nB_1: &request.Enc_nB.1,
                B: &constants::B_COMPRESSED,
            },
        )
        .map_err(|_| "client proof failed to verify")?;

        let b = Scalar::random(&mut rng);
        let r = Scalar::random(&mut rng);

        let Enc_nB = (
            request.Enc_nB.0.decompress().ok_or("bad point")?,
            request.Enc_nB.1.decompress().ok_or("bad point")?,
        );
        let D = request.D.decompress().ok_or("failed to decompress")?;
        let w = Scalar::from(request.w);
        let P = B * &b;
        let wP = B * &(b * w);

        let Enc_Q = (
            RistrettoPoint::multiscalar_mul(&[r, b * sk.x_2], &[*B, Enc_nB.0]),
            RistrettoPoint::multiscalar_mul(
                &[sk.x_0 + sk.x_1 * w, b * sk.x_2, r],
                &[P, Enc_nB.1, D],
            ),
        );

        use proofs::issuer::*;

        let t_2 = b * sk.x_2;
        let T_2 = b * params.X_2;
        // XXX zkp API should take an RNG
        let (proof, points) = prove_compact(
            &mut transcript,
            ProveAssignments {
                b: &b,
                r: &r,
                x_0: &sk.x_0,
                x_1: &sk.x_1,
                x_2: &sk.x_2,
                x_0_blinding: &sk.x_0_blinding,
                t_2: &t_2,
                P: &P,
                wP: &wP,
                D: &D,
                Enc_nB_0: &Enc_nB.0,
                Enc_nB_1: &Enc_nB.1,
                Enc_Q_0: &Enc_Q.0,
                Enc_Q_1: &Enc_Q.1,
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
            T_2: points.T_2_a,
            Enc_Q: (points.Enc_Q_0, points.Enc_Q_1),
            proof,
        })
    }
}

impl AwaitingResponse {
    /// Verify an issuance response and obtain a wallet credential.
    #[allow(non_snake_case)]
    pub fn verify_response(mut self, response: Response) -> Result<Wallet, &'static str> {
        // XXX-zkp: need to be able to pass either compressed or decompressed points or both
        let P = response.P.decompress().ok_or("failed to decompress")?;
        let wP = P * Scalar::from(self.w);

        use proofs::issuer::*;
        verify_compact(
            &response.proof,
            &mut self.transcript,
            VerifyAssignments {
                P: &response.P,
                wP: &wP.compress(),
                D: &self.D.compress(),
                Enc_nB_0: &self.Enc_nB.0,
                Enc_nB_1: &self.Enc_nB.1,
                Enc_Q_0: &response.Enc_Q.0,
                Enc_Q_1: &response.Enc_Q.1,
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
            n: self.n,
            w: self.w,
        })
    }
}
