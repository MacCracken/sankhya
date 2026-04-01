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

fn babylonian_sqrt_convergence(c: &mut Criterion) {
    c.bench_function("babylonian/sqrt_convergence", |b| {
        b.iter(|| {
            for &n in &[2.0, 3.0, 5.0, 7.0, 10.0, 100.0, 1000.0] {
                black_box(sankhya::babylonian::babylonian_sqrt(black_box(n), 20).unwrap());
            }
        });
    });
}

fn vedic_nikhilam_multiply(c: &mut Criterion) {
    c.bench_function("vedic/nikhilam_multiply_100", |b| {
        b.iter(|| {
            for a in 90..=99u64 {
                for b_val in 90..=99u64 {
                    let _ = black_box(sankhya::vedic::vedic_multiply_nikhilam(
                        black_box(a),
                        black_box(b_val),
                    ));
                }
            }
        });
    });
}

fn islamic_cubic_newton(c: &mut Criterion) {
    c.bench_function("islamic/cubic_newton", |b| {
        b.iter(|| {
            // x³ - 8 = 0 → x = 2
            black_box(
                sankhya::islamic::classify_khayyam_cubic(
                    black_box(1.0),
                    black_box(0.0),
                    black_box(0.0),
                    black_box(-8.0),
                )
                .unwrap(),
            );
            // x³ + 6x - 20 = 0 → x = 2
            black_box(
                sankhya::islamic::classify_khayyam_cubic(
                    black_box(1.0),
                    black_box(0.0),
                    black_box(6.0),
                    black_box(-20.0),
                )
                .unwrap(),
            );
        });
    });
}

fn epoch_correlate(c: &mut Criterion) {
    c.bench_function("epoch/correlate", |b| {
        b.iter(|| {
            // Correlate at a few significant dates
            for &jdn in &[
                584_283.0,   // Mayan epoch
                1_772_028.5, // Censorinus epoch
                2_451_545.0, // J2000.0
                sankhya::epoch::YOUNGER_DRYAS_JDN,
            ] {
                black_box(sankhya::epoch::correlate(black_box(jdn)).unwrap());
            }
        });
    });
}

fn archimedes_pi_iterations(c: &mut Criterion) {
    c.bench_function("greek/archimedes_pi_20", |b| {
        b.iter(|| {
            black_box(sankhya::greek::archimedes_pi(black_box(20)));
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
    babylonian_sqrt_convergence,
    vedic_nikhilam_multiply,
    islamic_cubic_newton,
    epoch_correlate,
    archimedes_pi_iterations,
);
criterion_main!(benches);
