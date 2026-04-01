//! Integration tests for sankhya.

use sankhya::babylonian;
use sankhya::chinese;
use sankhya::egyptian;
use sankhya::greek;
use sankhya::mayan;
use sankhya::vedic;

// ===== Mayan =====

#[test]
fn mayan_long_count_dec_21_2012() {
    // December 21, 2012 corresponds to Mayan Long Count 13.0.0.0.0
    // = 13 baktun = 13 * 144,000 = 1,872,000 days from creation
    let days = 13u64 * 144_000;
    let lc = mayan::LongCount::from_days(days).unwrap();
    assert_eq!(lc.baktun, 13);
    assert_eq!(lc.katun, 0);
    assert_eq!(lc.tun, 0);
    assert_eq!(lc.uinal, 0);
    assert_eq!(lc.kin, 0);
    assert_eq!(lc.to_string(), "13.0.0.0.0");
}

#[test]
fn mayan_long_count_roundtrip() {
    // Roundtrip: days -> LongCount -> days
    for days in [0, 1, 20, 360, 7200, 144_000, 1_872_000, 2_000_000] {
        let lc = mayan::LongCount::from_days(days).unwrap();
        assert_eq!(lc.to_days(), days, "roundtrip failed for {days} days");
    }
}

#[test]
fn mayan_vigesimal_roundtrip_400() {
    // 400 in base 20 is 1.0.0 (1*400 + 0*20 + 0)
    let digits = mayan::to_vigesimal(400);
    assert_eq!(digits, vec![1, 0, 0]);
    assert_eq!(mayan::from_vigesimal(&digits).unwrap(), 400);
}

#[test]
fn mayan_vigesimal_roundtrip_large() {
    for n in [0, 1, 19, 20, 399, 400, 8000, 160_000, 1_000_000] {
        let digits = mayan::to_vigesimal(n);
        assert_eq!(
            mayan::from_vigesimal(&digits).unwrap(),
            n,
            "vigesimal roundtrip failed for {n}"
        );
    }
}

#[test]
fn mayan_calendar_round_cycle() {
    // The Calendar Round repeats every lcm(260, 365) = 18,980 days
    let (tz1, haab1) = mayan::calendar_round(0);
    let (tz2, haab2) = mayan::calendar_round(mayan::CALENDAR_ROUND_DAYS);
    assert_eq!(tz1, tz2, "Tzolkin should repeat after 18,980 days");
    assert_eq!(haab1, haab2, "Haab should repeat after 18,980 days");
    assert_eq!(mayan::CALENDAR_ROUND_DAYS, 18_980);
}

#[test]
fn mayan_tzolkin_260_day_cycle() {
    // Tzolkin repeats every 260 days
    let tz1 = mayan::Tzolkin::from_days(0);
    let tz2 = mayan::Tzolkin::from_days(260);
    assert_eq!(tz1, tz2);
}

#[test]
fn mayan_haab_365_day_cycle() {
    // Haab repeats every 365 days
    let h1 = mayan::Haab::from_days(0);
    let h2 = mayan::Haab::from_days(365);
    assert_eq!(h1, h2);
}

#[test]
fn mayan_venus_synodic_period() {
    // The Mayan-computed Venus synodic period is approximately 583.92 days
    assert!((mayan::VENUS_SYNODIC_PERIOD - 583.92).abs() < 0.01);
    // The Mayan rounded value (584) times 5 equals 8 Haab years
    assert_eq!(584 * 5, 365 * 8);
}

#[test]
fn mayan_venus_phases_sum_to_584() {
    // Morning Star (236) + Superior Conjunction (90) + Evening Star (250)
    // + Inferior Conjunction (8) = 584
    assert_eq!(236 + 90 + 250 + 8, 584);
}

// ===== Babylonian =====

#[test]
fn babylonian_sexagesimal_roundtrip_3600() {
    // 3600 in base 60 is 1.0.0 (1*3600 + 0*60 + 0)
    let digits = babylonian::to_sexagesimal(3600);
    assert_eq!(digits, vec![1, 0, 0]);
    assert_eq!(babylonian::from_sexagesimal(&digits).unwrap(), 3600);
}

#[test]
fn babylonian_sexagesimal_roundtrip_large() {
    for n in [0, 1, 59, 60, 3599, 3600, 216_000, 12_960_000] {
        let digits = babylonian::to_sexagesimal(n);
        assert_eq!(
            babylonian::from_sexagesimal(&digits).unwrap(),
            n,
            "sexagesimal roundtrip failed for {n}"
        );
    }
}

#[test]
fn babylonian_sqrt2_herons_method() {
    // The YBC 7289 tablet shows sqrt(2) accurate to 6 decimal places
    let result = babylonian::babylonian_sqrt(2.0, 10).unwrap();
    assert!(
        (result - std::f64::consts::SQRT_2).abs() < 1e-15,
        "Heron's method sqrt(2) = {result}, expected {}",
        std::f64::consts::SQRT_2
    );
}

#[test]
fn babylonian_sqrt_perfect_squares() {
    for n in [4.0, 9.0, 16.0, 25.0, 100.0] {
        let result = babylonian::babylonian_sqrt(n, 20).unwrap();
        let expected = n.sqrt();
        assert!(
            (result - expected).abs() < 1e-12,
            "sqrt({n}) = {result}, expected {expected}"
        );
    }
}

#[test]
fn babylonian_plimpton_322_all_valid() {
    let triples = babylonian::generate_plimpton_triples();
    assert_eq!(triples.len(), 15);
    for (a, b, c) in &triples {
        assert_eq!(a * a + b * b, c * c, "invalid triple: ({a}, {b}, {c})");
    }
}

#[test]
fn babylonian_reciprocal_table_spot_check() {
    let table = babylonian::reciprocal_table();
    // 1/2 = 0;30 in sexagesimal
    assert_eq!(table.get(&2).unwrap(), &vec![30]);
    // 1/3 = 0;20
    assert_eq!(table.get(&3).unwrap(), &vec![20]);
    // 1/4 = 0;15
    assert_eq!(table.get(&4).unwrap(), &vec![15]);
    // 1/8 = 0;7,30
    assert_eq!(table.get(&8).unwrap(), &vec![7, 30]);
}

// ===== Egyptian =====

#[test]
fn egyptian_decompose_2_3() {
    // 2/3 = 1/2 + 1/6
    let result = egyptian::decompose(2, 3).unwrap();
    assert_eq!(result, vec![2, 6]);
    // Verify sum
    let sum: f64 = result.iter().map(|&d| 1.0 / d as f64).sum();
    assert!((sum - 2.0 / 3.0).abs() < 1e-15);
}

#[test]
fn egyptian_decompose_various() {
    // 3/7 should decompose into unit fractions that sum to 3/7
    let result = egyptian::decompose(3, 7).unwrap();
    let sum: f64 = result.iter().map(|&d| 1.0 / d as f64).sum();
    assert!(
        (sum - 3.0 / 7.0).abs() < 1e-10,
        "3/7 decomposition sum = {sum}"
    );
}

#[test]
fn egyptian_multiply_12_13() {
    assert_eq!(egyptian::egyptian_multiply(12, 13), 156);
}

#[test]
fn egyptian_multiply_various() {
    assert_eq!(egyptian::egyptian_multiply(0, 100), 0);
    assert_eq!(egyptian::egyptian_multiply(1, 1), 1);
    assert_eq!(egyptian::egyptian_multiply(7, 11), 77);
    assert_eq!(egyptian::egyptian_multiply(100, 100), 10_000);
}

#[test]
fn egyptian_divide_basic() {
    let (q, rem) = egyptian::egyptian_divide(10, 3).unwrap();
    assert_eq!(q, 3);
    // Remainder is 1/3
    let rem_sum: f64 = rem.iter().map(|&d| 1.0 / d as f64).sum();
    assert!((rem_sum - 1.0 / 3.0).abs() < 1e-15);
}

#[test]
fn egyptian_decans_count() {
    assert_eq!(egyptian::DECANS.len(), 36);
    // Each decan spans 10 degrees
    for (i, decan) in egyptian::DECANS.iter().enumerate() {
        assert_eq!(decan.number, (i + 1) as u8);
        assert!((decan.ecliptic_longitude - (i as f64 * 10.0)).abs() < 1e-10);
    }
}

// ===== Vedic =====

#[test]
fn vedic_sulba_sqrt2_baudhayana() {
    // Baudhayana's approximation: 1 + 1/3 + 1/(3*4) - 1/(3*4*34)
    // = 577/408 = 1.41421568627...
    let approx = vedic::sulba_sqrt2();
    let expected = 577.0 / 408.0;
    assert!(
        (approx - expected).abs() < 1e-15,
        "sulba_sqrt2() = {approx}, expected {expected}"
    );
    // Accurate to 5 decimal places of true sqrt(2)
    assert!(
        (approx - std::f64::consts::SQRT_2).abs() < 1e-5,
        "Baudhayana's sqrt(2) should be accurate to 5 decimal places"
    );
}

#[test]
fn vedic_meru_prastara_matches_pascal() {
    let triangle = vedic::meru_prastara(5).unwrap();
    assert_eq!(triangle.len(), 5);
    assert_eq!(triangle[0], vec![1]);
    assert_eq!(triangle[1], vec![1, 1]);
    assert_eq!(triangle[2], vec![1, 2, 1]);
    assert_eq!(triangle[3], vec![1, 3, 3, 1]);
    assert_eq!(triangle[4], vec![1, 4, 6, 4, 1]);
}

#[test]
fn vedic_meru_prastara_row_sums() {
    // Row n should sum to 2^n
    let triangle = vedic::meru_prastara(10).unwrap();
    for (i, row) in triangle.iter().enumerate() {
        let sum: u64 = row.iter().sum();
        assert_eq!(sum, 1u64 << i, "row {i} sum should be 2^{i}");
    }
}

#[test]
fn vedic_sulba_diagonal() {
    // 3-4-5 triangle
    assert!((vedic::sulba_diagonal(3.0, 4.0) - 5.0).abs() < 1e-15);
    // 5-12-13 triangle
    assert!((vedic::sulba_diagonal(5.0, 12.0) - 13.0).abs() < 1e-10);
}

#[test]
fn vedic_katapayadi_roundtrip() {
    for n in [0, 1, 42, 123, 9876] {
        let encoded = vedic::katapayadi_encode(n);
        let decoded = vedic::katapayadi_decode(&encoded).unwrap();
        assert_eq!(decoded, n, "Katapayadi roundtrip failed for {n}");
    }
}

// ===== Chinese =====

#[test]
fn chinese_remainder_sun_tzu_original() {
    // The original Sun Tzu problem:
    // "Count by 3 remainder 2, by 5 remainder 3, by 7 remainder 2"
    let result = chinese::chinese_remainder(&[(2, 3), (3, 5), (2, 7)]);
    assert_eq!(result, Some(23));
}

#[test]
fn chinese_remainder_simple() {
    // x = 1 (mod 2), x = 2 (mod 3) -> x = 5
    let result = chinese::chinese_remainder(&[(1, 2), (2, 3)]);
    assert_eq!(result, Some(5));
}

#[test]
fn chinese_magic_square_3x3_lo_shu() {
    let sq = chinese::magic_square(3).unwrap();
    assert!(chinese::is_magic_square(&sq));
    // Lo Shu magic constant = n(n^2+1)/2 = 3*10/2 = 15
    let magic_sum: u64 = sq[0].iter().sum();
    assert_eq!(magic_sum, 15);
    // Must contain all numbers 1-9
    let mut all: Vec<u64> = sq.iter().flat_map(|row| row.iter().copied()).collect();
    all.sort();
    assert_eq!(all, (1..=9).collect::<Vec<u64>>());
}

#[test]
fn chinese_magic_square_5x5() {
    let sq = chinese::magic_square(5).unwrap();
    assert!(chinese::is_magic_square(&sq));
    // Magic constant for 5x5 = 5*26/2 = 65
    let magic_sum: u64 = sq[0].iter().sum();
    assert_eq!(magic_sum, 65);
}

#[test]
fn chinese_rod_arithmetic() {
    let a = chinese::RodNumeral::new(42);
    let b = chinese::RodNumeral::new(17);
    assert_eq!(chinese::rod_add(a, b).value, 59);
    assert_eq!(chinese::rod_subtract(a, b).value, 25);
    assert_eq!(chinese::rod_multiply(a, b).value, 714);
}

// ===== Greek =====

#[test]
fn greek_archimedes_pi_bounds() {
    // After 10 iterations, pi should be tightly bounded
    let (lower, upper) = greek::archimedes_pi(10);
    assert!(
        lower < std::f64::consts::PI,
        "lower bound {lower} should be < pi"
    );
    assert!(
        upper > std::f64::consts::PI,
        "upper bound {upper} should be > pi"
    );
    assert!(
        (upper - lower) < 1e-5,
        "bounds should be tight after 10 iterations"
    );
}

#[test]
fn greek_archimedes_pi_initial() {
    // After 0 iterations (hexagon): 3 < pi < 3.4641...
    let (lower, upper) = greek::archimedes_pi(0);
    assert!((lower - 3.0).abs() < 1e-10);
    assert!((upper - 2.0 * 3.0_f64.sqrt()).abs() < 1e-10);
}

#[test]
fn greek_gcd_48_18() {
    assert_eq!(greek::gcd(48, 18), 6);
}

#[test]
fn greek_gcd_coprime() {
    assert_eq!(greek::gcd(17, 13), 1);
}

#[test]
fn greek_sieve_count() {
    // There are 25 primes below 100
    let primes = greek::sieve(100);
    assert_eq!(primes.len(), 25);
    assert_eq!(primes[0], 2);
    assert_eq!(primes[24], 97);
}

#[test]
fn greek_sieve_10000() {
    // There are 1229 primes below 10000
    let primes = greek::sieve(10_000);
    assert_eq!(primes.len(), 1229);
}

#[test]
fn greek_golden_ratio() {
    let expected = (1.0_f64 + 5.0_f64.sqrt()) / 2.0;
    assert!((greek::PHI - expected).abs() < 1e-15);
}

#[test]
fn greek_fibonacci_ratio_converges() {
    let ratio = greek::fibonacci_ratio(40);
    assert!((ratio - greek::PHI).abs() < 1e-10);
}

#[test]
fn greek_antikythera_metonic_cycle() {
    let ratios = greek::antikythera_gear_ratios();
    let metonic = ratios.get("Metonic").unwrap();
    // 19 years = 235 synodic months
    assert_eq!(metonic.gear_teeth, 235);
}

// ===== Serde roundtrips =====

#[test]
fn serde_roundtrip_long_count() {
    let lc = mayan::LongCount::from_days(1_872_000).unwrap();
    let json = serde_json::to_string(&lc).unwrap();
    let lc2: mayan::LongCount = serde_json::from_str(&json).unwrap();
    assert_eq!(lc, lc2);
}

#[test]
fn serde_roundtrip_tzolkin() {
    let tz = mayan::Tzolkin::from_days(0);
    let json = serde_json::to_string(&tz).unwrap();
    let tz2: mayan::Tzolkin = serde_json::from_str(&json).unwrap();
    assert_eq!(tz, tz2);
}

#[test]
fn serde_roundtrip_babylonian_numeral() {
    let bn = babylonian::BabylonianNumeral::from_value(42).unwrap();
    let json = serde_json::to_string(&bn).unwrap();
    let bn2: babylonian::BabylonianNumeral = serde_json::from_str(&json).unwrap();
    assert_eq!(bn, bn2);
}

#[test]
fn serde_roundtrip_mayan_numeral() {
    let mn = mayan::MayanNumeral::from_value(13).unwrap();
    let json = serde_json::to_string(&mn).unwrap();
    let mn2: mayan::MayanNumeral = serde_json::from_str(&json).unwrap();
    assert_eq!(mn, mn2);
}

#[test]
fn serde_roundtrip_haab() {
    let h = mayan::Haab::from_days(100);
    let json = serde_json::to_string(&h).unwrap();
    let h2: mayan::Haab = serde_json::from_str(&json).unwrap();
    assert_eq!(h, h2);
}

#[test]
fn serde_roundtrip_rod_numeral() {
    let rn = chinese::RodNumeral::new(42);
    let json = serde_json::to_string(&rn).unwrap();
    let rn2: chinese::RodNumeral = serde_json::from_str(&json).unwrap();
    assert_eq!(rn, rn2);
}

#[test]
fn serde_roundtrip_venus_phase() {
    let phase = mayan::venus_phase(100);
    let json = serde_json::to_string(&phase).unwrap();
    let phase2: mayan::VenusPhase = serde_json::from_str(&json).unwrap();
    assert_eq!(phase, phase2);
}

#[test]
fn serde_roundtrip_error() {
    let err = sankhya::SankhyaError::InvalidDate("test".into());
    let json = serde_json::to_string(&err).unwrap();
    let err2: sankhya::SankhyaError = serde_json::from_str(&json).unwrap();
    assert_eq!(err, err2);
}
