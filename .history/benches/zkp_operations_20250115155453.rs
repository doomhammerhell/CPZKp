use criterion::{black_box, criterion_group, criterion_main, Criterion};
use num_bigint::BigUint;
use zkp_auth::{exponentiates_points, Point, Secp256k1Point};

fn scalar_operations_benchmark(c: &mut Criterion) {
    let p = BigUint::from(10009u32);
    let g = Point::Scalar(BigUint::from(3u32));
    let h = Point::Scalar(BigUint::from(2892u32));
    let exp = BigUint::from(123u32);

    c.bench_function("scalar exponentiation", |b| {
        b.iter(|| {
            exponentiates_points(black_box(&exp), black_box(&g), black_box(&h), black_box(&p))
        })
    });
}

fn ec_operations_benchmark(c: &mut Criterion) {
    let p = Secp256k1Point::prime();
    let g = Secp256k1Point::generator();
    let h = g.clone().scale(BigUint::from(13u32));
    let g = Point::from_secp256k1(&g).unwrap();
    let h = Point::from_secp256k1(&h).unwrap();
    let exp = BigUint::from(300u32);

    c.bench_function("elliptic curve exponentiation", |b| {
        b.iter(|| {
            exponentiates_points(black_box(&exp), black_box(&g), black_box(&h), black_box(&p))
        })
    });
}

fn serialization_benchmark(c: &mut Criterion) {
    let point = Point::ECPoint(
        BigUint::from_bytes_be(&[255; 32]),
        BigUint::from_bytes_be(&[255; 32]),
    );

    c.bench_function("point serialization", |b| {
        b.iter(|| black_box(&point).serialize())
    });
}

criterion_group!(
    benches,
    scalar_operations_benchmark,
    ec_operations_benchmark,
    serialization_benchmark
);
criterion_main!(benches);
