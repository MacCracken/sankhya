use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

fn long_count_conversion_1000(c: &mut Criterion) {
    c.bench_function("mayan/long_count_conversion_1000", |b| {
        b.iter(|| {
            for days in 0..1000u64 {
                let lc = sankhya::mayan::LongCount::from_days(black_box(days)).unwrap();
                black_box(lc.to_days());
            }
        });
    });
}

fn vigesimal_roundtrip_1000(c: &mut Criterion) {
    c.bench_function("mayan/vigesimal_roundtrip_1000", |b| {
        b.iter(|| {
            for n in 0..1000u64 {
                let digits = sankhya::mayan::to_vigesimal(black_box(n));
                black_box(sankhya::mayan::from_vigesimal(&digits).unwrap());
            }
        });
    });
}

fn sieve_10000(c: &mut Criterion) {
    c.bench_function("greek/sieve_10000", |b| {
        b.iter(|| {
            black_box(sankhya::greek::sieve(black_box(10_000)));
        });
    });
}

fn chinese_remainder_100(c: &mut Criterion) {
    c.bench_function("chinese/chinese_remainder_100", |b| {
        b.iter(|| {
            for _ in 0..100 {
                black_box(sankhya::chinese::chinese_remainder(black_box(&[
                    (2, 3),
                    (3, 5),
                    (2, 7),
                ])));
            }
        });
    });
}

fn egyptian_decompose_1000(c: &mut Criterion) {
    c.bench_function("egyptian/decompose_1000", |b| {
        b.iter(|| {
            for n in 1..=20u64 {
                for d in (n + 1)..=(n + 50) {
                    black_box(sankhya::egyptian::decompose(black_box(n), black_box(d)).ok());
                }
            }
        });
    });
}

criterion_group!(
    benches,
    long_count_conversion_1000,
    vigesimal_roundtrip_1000,
    sieve_10000,
    chinese_remainder_100,
    egyptian_decompose_1000,
);
criterion_main!(benches);
