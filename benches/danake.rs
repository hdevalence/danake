use criterion::{criterion_group, criterion_main, Criterion};

use chrono;
use merlin::Transcript;
use rand;

use danake;

// Only do one benchmark for now so we can move things around later.
pub fn wallet_topup_response(c: &mut Criterion) {
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

    let (_client_state, request) = wallet
        .request_topup(
            2_000,
            &params,
            Transcript::new(b"wallet topup test"),
            rand::thread_rng(),
        )
        .expect("epoch is correct");

    c.bench_function("wallet topup request", |b| {
        b.iter(|| {
            let _response = secret
                .topup(
                    request.clone(),
                    Transcript::new(b"wallet topup test"),
                    rand::thread_rng(),
                )
                .expect("topup should succeed");
        })
    });
}

criterion_group!(wallet_issuance, wallet_topup_response);
criterion_main!(wallet_issuance);
