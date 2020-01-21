use criterion::{black_box, criterion_group, criterion_main, Criterion};

use chrono;
use merlin::Transcript;
use rand;

use danake;

// Only do one benchmark for now so we can move things around later.
pub fn wallet_issuance_response(c: &mut Criterion) {
    use danake::{wallet::*, Epoch, EpochParameters};

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

    c.bench_function("wallet issuance request", |b| {
        b.iter(|| {
            let response = secret
                .issue(
                    request.clone(),
                    Transcript::new(b"wallet issuance test"),
                    rand::thread_rng(),
                )
                .expect("issuance should succeed");
        })
    });
}

criterion_group!(wallet_issuance, wallet_issuance_response);
criterion_main!(wallet_issuance);
