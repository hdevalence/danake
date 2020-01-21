use chrono;
use merlin::Transcript;
use rand;

use danake;

#[test]
fn wallet_issuance_and_topup() {
    use danake::{wallet::*, EpochParameters};

    let epoch_params = EpochParameters::from(std::time::Duration::from_secs(86400));
    let epoch = epoch_params.epoch_at(chrono::Utc::now());

    let secret = Secrets::new(epoch, rand::thread_rng());
    let params = Parameters::from(&secret);

    let (client_state, request) = Wallet::request_issuance(
        1_000,
        &params,
        Transcript::new(b"wallet issuance test"),
        rand::thread_rng(),
    );

    let response = secret
        .issue(
            request,
            Transcript::new(b"wallet issuance test"),
            rand::thread_rng(),
        )
        .expect("issuance should succeed");

    let wallet = client_state
        .verify_response(response)
        .expect("response should verify");

    let (client_state, request) = wallet
        .request_topup(
            2_000,
            &params,
            Transcript::new(b"wallet topup test"),
            rand::thread_rng(),
        )
        .expect("epoch is correct");

    let response = secret
        .topup(
            request,
            Transcript::new(b"wallet topup test"),
            rand::thread_rng(),
        )
        .expect("topup should succeed");

    let _wallet2 = client_state
        .verify_response(response)
        .expect("response should verify");
}
