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

fn roman_roundtrip_3999(c: &mut Criterion) {
    c.bench_function("roman/roundtrip_3999", |b| {
        b.iter(|| {
            for n in 1..=3999u32 {
                let s = sankhya::roman::to_roman_str(black_box(n)).unwrap();
                black_box(sankhya::roman::from_roman(&s).unwrap());
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

fn gregorian_jdn_roundtrip(c: &mut Criterion) {
    c.bench_function("gregorian/jdn_roundtrip_1000", |b| {
        b.iter(|| {
            for d in 0..1000 {
                let jdn = 2_451_544.5 + f64::from(d);
                let date = sankhya::gregorian::jdn_to_gregorian(black_box(jdn));
                black_box(sankhya::gregorian::gregorian_to_jdn(&date).unwrap());
            }
        });
    });
}

fn hebrew_jdn_roundtrip(c: &mut Criterion) {
    c.bench_function("hebrew/jdn_roundtrip_100", |b| {
        b.iter(|| {
            for d in 0..100 {
                let jdn = 2_460_000.5 + f64::from(d);
                let date = sankhya::hebrew::jdn_to_hebrew(black_box(jdn));
                black_box(sankhya::hebrew::hebrew_to_jdn(&date).unwrap());
            }
        });
    });
}

fn persian_jdn_roundtrip(c: &mut Criterion) {
    c.bench_function("persian/jdn_roundtrip_1000", |b| {
        b.iter(|| {
            for d in 0..1000 {
                let jdn = 2_459_294.5 + f64::from(d);
                let date = sankhya::persian::jdn_to_persian(black_box(jdn));
                black_box(sankhya::persian::persian_to_jdn(&date).unwrap());
            }
        });
    });
}

fn astro_precession(c: &mut Criterion) {
    c.bench_function("astro/precession_1000", |b| {
        let coord = sankhya::astro::star_j2000(sankhya::astro::StarName::Sirius);
        b.iter(|| {
            for i in 0..1000 {
                let target_jdn = 2_451_545.0 - f64::from(i) * 365.25;
                black_box(sankhya::astro::precess_coordinates(
                    &coord,
                    2_451_545.0,
                    black_box(target_jdn),
                ));
            }
        });
    });
}

fn epoch_convert_all(c: &mut Criterion) {
    c.bench_function("epoch/convert_all_calendars", |b| {
        b.iter(|| {
            let greg = sankhya::gregorian::GregorianDate {
                year: 2025,
                month: sankhya::gregorian::GregorianMonth::January,
                day: 1,
            };
            black_box(
                sankhya::epoch::convert(&sankhya::epoch::CalendarDate::Gregorian(black_box(greg)))
                    .unwrap(),
            );
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
    roman_roundtrip_3999,
    archimedes_pi_iterations,
    gregorian_jdn_roundtrip,
    hebrew_jdn_roundtrip,
    persian_jdn_roundtrip,
    astro_precession,
    epoch_convert_all,
);
criterion_main!(benches);
