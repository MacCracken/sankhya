//! Adversarial tests for sankhya.
//!
//! Every public function that accepts user input is fuzzed with hostile values:
//! NaN, ±Inf, ±0, f64::MIN/MAX, f64::EPSILON, overflow-prone integers, and
//! empty/degenerate inputs. The contract: either return `Err`, or return
//! `Ok(finite_value)` — never panic, never propagate NaN/Inf silently.

// ===== Mayan =====

#[test]
fn fuzz_from_vigesimal_empty() {
    assert_eq!(sankhya::mayan::from_vigesimal(&[]).unwrap(), 0);
}

#[test]
fn fuzz_from_vigesimal_invalid_digit() {
    assert!(sankhya::mayan::from_vigesimal(&[20]).is_err());
    assert!(sankhya::mayan::from_vigesimal(&[255]).is_err());
}

#[test]
fn fuzz_from_vigesimal_overflow() {
    let digits = vec![19u8; 20];
    assert!(sankhya::mayan::from_vigesimal(&digits).is_err());
}

#[test]
fn fuzz_long_count_from_days_max() {
    let result = sankhya::mayan::LongCount::from_days(u64::MAX);
    if let Ok(lc) = result {
        assert!(lc.baktun > 0);
    }
}

#[test]
fn fuzz_long_count_new_invalid() {
    assert!(sankhya::mayan::LongCount::new(0, 20, 0, 0, 0).is_err());
    assert!(sankhya::mayan::LongCount::new(0, 0, 20, 0, 0).is_err());
    assert!(sankhya::mayan::LongCount::new(0, 0, 0, 18, 0).is_err());
    assert!(sankhya::mayan::LongCount::new(0, 0, 0, 0, 20).is_err());
}

#[test]
fn fuzz_mayan_numeral_out_of_range() {
    assert!(sankhya::mayan::MayanNumeral::from_value(20).is_err());
    assert!(sankhya::mayan::MayanNumeral::from_value(255).is_err());
}

#[test]
fn fuzz_find_calendar_round_invalid() {
    use sankhya::mayan::{DaySign, HaabMonth};
    assert!(sankhya::mayan::find_calendar_round(0, DaySign::Ahau, 0, HaabMonth::Pop, 0).is_err());
    assert!(sankhya::mayan::find_calendar_round(14, DaySign::Ahau, 0, HaabMonth::Pop, 0).is_err());
    assert!(sankhya::mayan::find_calendar_round(1, DaySign::Ahau, 20, HaabMonth::Pop, 0).is_err());
    assert!(sankhya::mayan::find_calendar_round(1, DaySign::Ahau, 5, HaabMonth::Wayeb, 0).is_err());
}

#[test]
fn fuzz_find_tzolkin_invalid() {
    use sankhya::mayan::DaySign;
    assert!(sankhya::mayan::find_tzolkin(0, DaySign::Ahau, 0).is_err());
    assert!(sankhya::mayan::find_tzolkin(14, DaySign::Ahau, 0).is_err());
}

#[test]
fn fuzz_from_julian_day_before_epoch() {
    assert!(sankhya::mayan::LongCount::from_julian_day(0).is_err());
    assert!(sankhya::mayan::LongCount::from_julian_day(100).is_err());
}

// ===== Babylonian =====

#[test]
fn fuzz_from_sexagesimal_invalid_digit() {
    assert!(sankhya::babylonian::from_sexagesimal(&[60]).is_err());
    assert!(sankhya::babylonian::from_sexagesimal(&[255]).is_err());
}

#[test]
fn fuzz_from_sexagesimal_overflow() {
    let digits = vec![59u8; 15];
    assert!(sankhya::babylonian::from_sexagesimal(&digits).is_err());
}

#[test]
fn fuzz_babylonian_numeral_out_of_range() {
    assert!(sankhya::babylonian::BabylonianNumeral::from_value(60).is_err());
    assert!(sankhya::babylonian::BabylonianNumeral::from_value(255).is_err());
}

#[test]
fn fuzz_babylonian_sqrt_negative() {
    assert!(sankhya::babylonian::babylonian_sqrt(-1.0, 10).is_err());
    assert!(sankhya::babylonian::babylonian_sqrt(f64::NEG_INFINITY, 10).is_err());
}

#[test]
fn fuzz_babylonian_sqrt_zero_iterations() {
    assert!(sankhya::babylonian::babylonian_sqrt(2.0, 0).is_err());
}

#[test]
fn fuzz_babylonian_sqrt_nan() {
    let result = sankhya::babylonian::babylonian_sqrt(f64::NAN, 10);
    if let Ok(v) = result {
        assert!(v.is_finite(), "NaN input must not produce NaN output");
    }
}

#[test]
fn fuzz_babylonian_sqrt_infinity() {
    let result = sankhya::babylonian::babylonian_sqrt(f64::INFINITY, 10);
    if let Ok(v) = result {
        assert!(v.is_finite(), "Inf input must not produce Inf output");
    }
}

#[test]
fn fuzz_babylonian_sqrt_huge() {
    let result = sankhya::babylonian::babylonian_sqrt(f64::MAX, 10);
    if let Ok(v) = result {
        assert!(v.is_finite());
    }
}

#[test]
fn fuzz_babylonian_sqrt_tiny() {
    let result = sankhya::babylonian::babylonian_sqrt(f64::EPSILON, 10);
    assert!(result.is_ok());
    let v = result.unwrap();
    assert!(v.is_finite());
    assert!(v >= 0.0);
}

#[test]
fn fuzz_babylonian_to_jdn_invalid_day() {
    use sankhya::babylonian::{BabylonianDate, BabylonianMonth};
    let date = BabylonianDate {
        year: 1,
        month: BabylonianMonth::Nisannu,
        day: 31,
    };
    assert!(sankhya::babylonian::babylonian_to_jdn(&date).is_err());

    let date = BabylonianDate {
        year: 1,
        month: BabylonianMonth::Ayaru,
        day: 0,
    };
    assert!(sankhya::babylonian::babylonian_to_jdn(&date).is_err());
}

// ===== Egyptian =====

#[test]
fn fuzz_decompose_zero_denominator() {
    assert!(sankhya::egyptian::decompose(1, 0).is_err());
}

#[test]
fn fuzz_decompose_zero_numerator() {
    let result = sankhya::egyptian::decompose(0, 7).unwrap();
    assert!(result.is_empty());
}

#[test]
fn fuzz_decompose_large_fraction() {
    let result = sankhya::egyptian::decompose(5, 121);
    if let Ok(fracs) = result {
        let sum: f64 = fracs.iter().map(|&d| 1.0 / d as f64).sum();
        assert!((sum - 5.0 / 121.0).abs() < 1e-8);
    }
}

#[test]
fn fuzz_egyptian_divide_by_zero() {
    assert!(sankhya::egyptian::egyptian_divide(10, 0).is_err());
}

#[test]
fn fuzz_next_sopdet_rising_extreme_latitude() {
    assert!(sankhya::egyptian::next_sopdet_rising(2_451_545.0, 90.0).is_err());
    assert!(sankhya::egyptian::next_sopdet_rising(2_451_545.0, -90.0).is_err());
    assert!(sankhya::egyptian::next_sopdet_rising(2_451_545.0, 61.0).is_err());
    assert!(sankhya::egyptian::next_sopdet_rising(2_451_545.0, -61.0).is_err());
}

#[test]
fn fuzz_decan_from_longitude_extremes() {
    let _ = sankhya::egyptian::decan_from_longitude(f64::MAX);
    let _ = sankhya::egyptian::decan_from_longitude(f64::MIN);
    let _ = sankhya::egyptian::decan_from_longitude(-720.0);
    let _ = sankhya::egyptian::decan_from_longitude(720.0);
    let _ = sankhya::egyptian::decan_from_longitude(0.0);
    let _ = sankhya::egyptian::decan_from_longitude(360.0);
}

// ===== Vedic =====

#[test]
fn fuzz_katapayadi_decode_invalid() {
    assert!(sankhya::vedic::katapayadi_decode("xyz").is_err());
    assert!(sankhya::vedic::katapayadi_decode("hello-world").is_err());
}

#[test]
fn fuzz_meru_prastara_zero() {
    let result = sankhya::vedic::meru_prastara(0).unwrap();
    assert!(result.is_empty());
}

#[test]
fn fuzz_meru_prastara_large() {
    let result = sankhya::vedic::meru_prastara(70);
    assert!(result.is_err());
}

#[test]
fn fuzz_vedic_multiply_nikhilam_exceeds_base() {
    let result = sankhya::vedic::vedic_multiply_nikhilam(100, 1);
    if let Ok((_, _, _, _, product)) = result {
        assert_eq!(product, 100);
    }
}

// ===== Chinese =====

#[test]
fn fuzz_chinese_remainder_empty() {
    assert!(sankhya::chinese::chinese_remainder(&[]).is_none());
}

#[test]
fn fuzz_chinese_remainder_zero_modulus() {
    assert!(sankhya::chinese::chinese_remainder(&[(1, 0)]).is_none());
    assert!(sankhya::chinese::chinese_remainder(&[(1, 3), (2, 0)]).is_none());
}

#[test]
fn fuzz_chinese_remainder_non_coprime() {
    assert!(sankhya::chinese::chinese_remainder(&[(1, 4), (2, 6)]).is_none());
}

#[test]
fn fuzz_magic_square_invalid() {
    assert!(sankhya::chinese::magic_square(0).is_none());
    assert!(sankhya::chinese::magic_square(1).is_none());
    assert!(sankhya::chinese::magic_square(2).is_none());
    assert!(sankhya::chinese::magic_square(4).is_none());
    assert!(sankhya::chinese::magic_square(6).is_none());
}

#[test]
fn fuzz_is_magic_square_empty() {
    assert!(!sankhya::chinese::is_magic_square(&[]));
}

#[test]
fn fuzz_is_magic_square_ragged() {
    assert!(!sankhya::chinese::is_magic_square(&[vec![1, 2], vec![3]]));
}

// ===== Greek =====

#[test]
fn fuzz_sieve_zero() {
    assert!(sankhya::greek::sieve(0).is_empty());
    assert!(sankhya::greek::sieve(1).is_empty());
}

#[test]
fn fuzz_gcd_zeros() {
    assert_eq!(sankhya::greek::gcd(0, 0), 0);
    assert_eq!(sankhya::greek::gcd(0, 5), 5);
    assert_eq!(sankhya::greek::gcd(5, 0), 5);
}

#[test]
fn fuzz_lcm_zeros() {
    assert_eq!(sankhya::greek::lcm(0, 5), 0);
    assert_eq!(sankhya::greek::lcm(5, 0), 0);
    assert_eq!(sankhya::greek::lcm(0, 0), 0);
}

#[test]
fn fuzz_fibonacci_ratio_zero() {
    let r = sankhya::greek::fibonacci_ratio(0);
    assert!(r.is_finite());
}

// ===== Islamic =====

#[test]
fn fuzz_solve_al_jabr_all_zero() {
    assert!(sankhya::islamic::solve_al_jabr(0.0, 0.0, 0.0).is_err());
}

#[test]
fn fuzz_solve_al_jabr_nan() {
    let result = sankhya::islamic::solve_al_jabr(f64::NAN, 1.0, 1.0);
    if let Ok(sol) = result {
        for r in &sol.roots {
            assert!(r.is_finite(), "NaN input produced non-finite root");
        }
    }
}

#[test]
fn fuzz_solve_al_jabr_infinity() {
    let result = sankhya::islamic::solve_al_jabr(f64::INFINITY, 1.0, 1.0);
    if let Ok(sol) = result {
        for r in &sol.roots {
            assert!(r.is_finite(), "Inf input produced non-finite root");
        }
    }
}

#[test]
fn fuzz_classify_khayyam_zero_leading() {
    assert!(sankhya::islamic::classify_khayyam_cubic(0.0, 1.0, 2.0, 3.0).is_err());
}

#[test]
fn fuzz_classify_khayyam_nan() {
    let result = sankhya::islamic::classify_khayyam_cubic(f64::NAN, 1.0, 1.0, 1.0);
    if let Ok((_, roots)) = result {
        for r in &roots {
            assert!(r.is_finite(), "NaN input produced non-finite root");
        }
    }
}

#[test]
fn fuzz_complete_the_square_invalid() {
    assert!(sankhya::islamic::complete_the_square(0.0, 5.0).is_err());
    assert!(sankhya::islamic::complete_the_square(-1.0, 5.0).is_err());
    assert!(sankhya::islamic::complete_the_square(5.0, 0.0).is_err());
    assert!(sankhya::islamic::complete_the_square(5.0, -1.0).is_err());
}

#[test]
fn fuzz_hijri_to_jdn_invalid_day() {
    use sankhya::islamic::{HijriDate, HijriMonth};
    let date = HijriDate {
        year: 1,
        month: HijriMonth::Muharram,
        day: 0,
    };
    assert!(sankhya::islamic::hijri_to_jdn(&date).is_err());

    let date = HijriDate {
        year: 1,
        month: HijriMonth::Muharram,
        day: 31,
    };
    assert!(sankhya::islamic::hijri_to_jdn(&date).is_err());
}

// ===== Epoch =====

#[test]
fn fuzz_find_cycle_alignments_invalid_range() {
    use sankhya::epoch::CycleName;
    assert!(
        sankhya::epoch::find_cycle_alignments(
            &[CycleName::Saros, CycleName::Metonic],
            1000.0,
            500.0,
            1.0
        )
        .is_err()
    );
}

#[test]
fn fuzz_find_cycle_alignments_too_few_cycles() {
    use sankhya::epoch::CycleName;
    assert!(sankhya::epoch::find_cycle_alignments(&[CycleName::Saros], 0.0, 1000.0, 1.0).is_err());
}

#[test]
fn fuzz_correlate_extreme_jdn() {
    let result = sankhya::epoch::correlate(f64::MAX / 2.0);
    if let Ok(date) = result {
        assert!(date.jdn.is_finite());
    }
}

#[test]
fn fuzz_correlate_zero_jdn() {
    let result = sankhya::epoch::correlate(0.0);
    if let Ok(date) = result {
        assert!(date.mayan_long_count.is_none());
        assert!(date.julian_year.is_finite());
    }
}

#[test]
fn fuzz_correlate_negative_jdn() {
    let result = sankhya::epoch::correlate(-1_000_000.0);
    if let Ok(date) = result {
        assert!(date.mayan_long_count.is_none());
    }
}

#[test]
fn fuzz_bp_roundtrip_extremes() {
    for bp in [0.0, 1.0, 12_800.0, 100_000.0, 1e10] {
        let jdn = sankhya::epoch::bp_to_jdn(bp);
        let back = sankhya::epoch::jdn_to_bp(jdn);
        assert!(
            (back - bp).abs() < 1e-6,
            "BP roundtrip failed for {bp}: got {back}"
        );
    }
}

#[test]
fn fuzz_precessional_age_extreme_jdn() {
    let pos = sankhya::epoch::precessional_age(0.0);
    assert!(pos.fraction >= 0.0 && pos.fraction <= 1.0);

    let pos = sankhya::epoch::precessional_age(f64::MAX / 2.0);
    assert!(pos.vernal_point_longitude >= 0.0);
    assert!(pos.vernal_point_longitude < 360.0);
}

#[test]
fn fuzz_vernal_point_longitude_range() {
    for jdn in [0.0, 1_000_000.0, 2_451_545.0, 5_000_000.0, -1_000_000.0] {
        let lon = sankhya::epoch::vernal_point_longitude(jdn);
        assert!(lon >= 0.0, "longitude {lon} < 0 for jdn {jdn}");
        assert!(lon < 360.0, "longitude {lon} >= 360 for jdn {jdn}");
    }
}
