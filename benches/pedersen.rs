use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pathfinder_lib::{core::StarkHash, pedersen::pedersen_hash};

pub fn criterion_benchmark(c: &mut Criterion) {
    // These are the test vectors also used in tests, taken from
    // https://github.com/starkware-libs/crypto-cpp/blob/master/src/starkware/crypto/pedersen_hash_test.cc
    let e0 = StarkHash::from_hex_str(
        "0x3d937c035c878245caf64531a5756109c53068da139362728feb561405371cb",
    )
    .unwrap();
    let e1 = StarkHash::from_hex_str(
        "0x208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a",
    )
    .unwrap();

    c.bench_function("pedersen_hash", |b| {
        b.iter(|| {
            black_box(pedersen_hash(e0.clone(), e1.clone()));
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
