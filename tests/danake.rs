use chrono;
use merlin::Transcript;
use rand;

use danake;

#[test]
fn wallet_issuance_flow() {
    use danake::{wallet::*, EpochParameters};

    let epoch_params = EpochParameters::from(std::time::Duration::from_secs(86400));
    let epoch = epoch_params.epoch_at(chrono::Utc::now());

    let secret = Secrets::new(epoch, rand::thread_rng());
    let params = Parameters::from(&secret);

    let (client_state, request) = Wallet::request_issuance(
        100_000,
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

    let _wallet = client_state
        .verify_response(response)
        .expect("response should verify");
}