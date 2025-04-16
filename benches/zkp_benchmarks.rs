use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cpzkp::{Group, Point, get_constants, solve_zk_challenge_s, verify};
use num_bigint::BigUint;
use rand::random;

fn generate_random_point() -> Point {
    let x = BigUint::from_bytes_be(&random::<[u8; 32]>());
    let y = BigUint::from_bytes_be(&random::<[u8; 32]>());
    Point::ECPoint(x, y)
}

fn generate_random_biguint() -> BigUint {
    BigUint::from_bytes_be(&random::<[u8; 32]>())
}

fn benchmark_scalar_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar_operations");
    
    group.bench_function("get_constants_scalar", |b| {
        b.iter(|| get_constants(black_box(&Group::Scalar)))
    });

    let (_, q, _, _) = get_constants(&Group::Scalar).unwrap();
    let x = generate_random_biguint();
    let k = generate_random_biguint();
    let c = generate_random_biguint();

    group.bench_function("solve_zk_challenge_s_scalar", |b| {
        b.iter(|| solve_zk_challenge_s(black_box(&x), black_box(&k), black_box(&c), black_box(&q)))
    });

    group.finish();
}

fn benchmark_ecc_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("ecc_operations");
    
    group.bench_function("get_constants_ecc", |b| {
        b.iter(|| get_constants(black_box(&Group::EllipticCurve)))
    });

    let (_, q, g, h) = get_constants(&Group::EllipticCurve).unwrap();
    let x = generate_random_biguint();
    let k = generate_random_biguint();
    let c = generate_random_biguint();

    group.bench_function("solve_zk_challenge_s_ecc", |b| {
        b.iter(|| solve_zk_challenge_s(black_box(&x), black_box(&k), black_box(&c), black_box(&q)))
    });

    group.bench_function("point_scale", |b| {
        b.iter(|| g.scale(black_box(x.clone())))
    });

    group.bench_function("point_add", |b| {
        let p1 = generate_random_point();
        let p2 = generate_random_point();
        b.iter(|| p1.add(black_box(&p2)))
    });

    group.finish();
}

fn benchmark_verification(c: &mut Criterion) {
    let mut group = c.benchmark_group("verification");
    
    let (p, q, g, h) = get_constants(&Group::EllipticCurve).unwrap();
    let x = generate_random_biguint();
    let k = generate_random_biguint();
    let c_val = generate_random_biguint();
    let s = solve_zk_challenge_s(&x, &k, &c_val, &q);
    let y1 = g.scale(x.clone());
    let y2 = h.scale(x);
    let r1 = g.scale(k.clone());
    let r2 = h.scale(k);

    let params = cpzkp::VerificationParams {
        r1,
        r2,
        y1,
        y2,
        g,
        h,
        c: c_val,
        s,
        p,
    };

    group.bench_function("verify_proof", |b| {
        b.iter(|| verify(black_box(&params)))
    });

    group.finish();
}

fn benchmark_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialization");
    
    let point = generate_random_point();
    group.bench_function("point_serialize", |b| {
        b.iter(|| point.serialize())
    });

    let bytes = point.serialize();
    group.bench_function("point_deserialize", |b| {
        b.iter(|| Point::deserialize(black_box(bytes.clone()), black_box(&Group::EllipticCurve)))
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_scalar_operations,
    benchmark_ecc_operations,
    benchmark_verification,
    benchmark_serialization
);
criterion_main!(benches); 