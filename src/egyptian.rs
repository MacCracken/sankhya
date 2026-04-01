//! Egyptian mathematics.
//!
//! Implements Egyptian fraction decomposition (unit fractions), the
//! doubling method of multiplication from the Rhind Papyrus, Egyptian
//! division, and the stellar decan system.
//!
//! # Historical Context
//!
//! The Rhind Mathematical Papyrus (c. 1650 BCE, copied from an older
//! document c. 1850 BCE) is one of the most important sources of
//! ancient Egyptian mathematics. The Egyptians expressed all fractions
//! as sums of distinct unit fractions (1/n), with the sole exception
//! of 2/3. Their multiplication method based on repeated doubling is
//! equivalent to binary multiplication.

use serde::{Deserialize, Serialize};

use crate::error::{Result, SankhyaError};

// ---------------------------------------------------------------------------
// Egyptian fractions (unit fraction decomposition)
// ---------------------------------------------------------------------------

/// Decompose a fraction into a sum of distinct unit fractions using the
/// greedy algorithm (also known as the Fibonacci-Sylvester algorithm).
///
/// Returns a vector of denominators. For example, `decompose(2, 3)` returns
/// `[2, 6]` meaning 2/3 = 1/2 + 1/6.
///
/// # Errors
///
/// Returns [`SankhyaError::InvalidFraction`] if the denominator is zero.
/// Returns [`SankhyaError::ComputationError`] if decomposition exceeds
/// the iteration limit (prevents infinite loops on pathological inputs).
pub fn decompose(mut numerator: u64, mut denominator: u64) -> Result<Vec<u64>> {
    if denominator == 0 {
        return Err(SankhyaError::InvalidFraction(
            "denominator cannot be zero".into(),
        ));
    }
    if numerator == 0 {
        return Ok(Vec::new());
    }

    // Simplify the fraction first
    let g = gcd(numerator, denominator);
    numerator /= g;
    denominator /= g;

    // If already a unit fraction
    if numerator == 1 {
        return Ok(vec![denominator]);
    }

    let mut result = Vec::new();
    let max_iterations = 100;

    for _ in 0..max_iterations {
        if numerator == 0 {
            break;
        }
        if numerator == 1 {
            result.push(denominator);
            break;
        }

        // Find the smallest unit fraction 1/d <= numerator/denominator
        // i.e., d = ceil(denominator / numerator)
        let d = denominator.div_ceil(numerator);
        result.push(d);

        // Subtract 1/d from numerator/denominator:
        // numerator/denominator - 1/d = (numerator*d - denominator) / (denominator*d)
        let new_num = numerator
            .checked_mul(d)
            .and_then(|nd| nd.checked_sub(denominator))
            .ok_or_else(|| {
                SankhyaError::OverflowError("Egyptian fraction decomposition overflow".into())
            })?;
        let new_den = denominator.checked_mul(d).ok_or_else(|| {
            SankhyaError::OverflowError("Egyptian fraction decomposition overflow".into())
        })?;

        if new_num == 0 {
            break;
        }

        let g = gcd(new_num, new_den);
        numerator = new_num / g;
        denominator = new_den / g;
    }

    if numerator != 0 && (result.is_empty() || numerator != 1) {
        return Err(SankhyaError::ComputationError(
            "Egyptian fraction decomposition did not terminate".into(),
        ));
    }

    Ok(result)
}

/// Simple GCD for internal use.
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

// ---------------------------------------------------------------------------
// Rhind Papyrus multiplication (doubling method)
// ---------------------------------------------------------------------------

/// Multiply two numbers using the ancient Egyptian doubling method.
///
/// This method (described in the Rhind Papyrus) works by repeatedly
/// doubling one factor and halving the other, summing the doubled values
/// where the halved value is odd. This is equivalent to binary multiplication.
///
/// For example, 12 * 13:
///   1  * 13 = 13   (13 is odd, include)
///   2  * 6  = skip (6 is even)
///   4  * 3  = 12   (3 is odd, include -> 4*13=52)
///   8  * 1  = 104  (1 is odd, include -> 8*13=104)
///   Result: 13 + 52 + 104 = ... wait, let's use proper algorithm:
///   a=12, b=13: double a, halve b:
///   12*1 (b=13 odd, add 12), b=6, a=24
///   24 (b=6 even, skip), b=3, a=48
///   48 (b=3 odd, add 48), b=1, a=96
///   96 (b=1 odd, add 96)
///   Total: 12 + 48 + 96 = 156
#[must_use]
#[inline]
pub fn egyptian_multiply(a: u64, b: u64) -> u64 {
    let mut result: u64 = 0;
    let mut multiplicand = a;
    let mut multiplier = b;

    while multiplier > 0 {
        if multiplier & 1 == 1 {
            result = result.wrapping_add(multiplicand);
        }
        multiplicand = multiplicand.wrapping_shl(1);
        multiplier >>= 1;
    }
    result
}

// ---------------------------------------------------------------------------
// Egyptian division
// ---------------------------------------------------------------------------

/// Divide using the Egyptian method (inverse of doubling multiplication).
///
/// Returns `(quotient, remainder_fractions)` where `remainder_fractions`
/// is a vector of unit fraction denominators representing the remainder.
///
/// For example, `egyptian_divide(7, 3)` returns `(2, [3])` meaning
/// 7/3 = 2 + 1/3.
///
/// # Errors
///
/// Returns [`SankhyaError::InvalidFraction`] if divisor is zero.
pub fn egyptian_divide(dividend: u64, divisor: u64) -> Result<(u64, Vec<u64>)> {
    if divisor == 0 {
        return Err(SankhyaError::InvalidFraction(
            "divisor cannot be zero".into(),
        ));
    }

    let quotient = dividend / divisor;
    let remainder = dividend % divisor;

    if remainder == 0 {
        return Ok((quotient, Vec::new()));
    }

    // Express remainder/divisor as Egyptian fractions
    let fractions = decompose(remainder, divisor)?;
    Ok((quotient, fractions))
}

// ---------------------------------------------------------------------------
// Stellar decans
// ---------------------------------------------------------------------------

/// An Egyptian stellar decan — one of 36 star groups dividing the ecliptic.
///
/// The decans were used as a star clock: each decan rises heliacally
/// for about 10 days, dividing the year into 36 ten-day weeks.
/// They were painted on coffin lids and temple ceilings.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Decan {
    /// Decan number (1-36).
    pub number: u8,
    /// Traditional name of the decan.
    pub name: &'static str,
    /// Starting ecliptic longitude in degrees (0-360).
    pub ecliptic_longitude: f64,
}

/// The 36 Egyptian stellar decans, each spanning 10 degrees of the ecliptic.
///
/// Names are reconstructed from the Seti I cenotaph at Abydos and the
/// tomb of Senenmut. The ecliptic longitudes are approximate, as the
/// Egyptian decan system was sidereal, not tropical.
pub static DECANS: [Decan; 36] = [
    Decan {
        number: 1,
        name: "Kenmet",
        ecliptic_longitude: 0.0,
    },
    Decan {
        number: 2,
        name: "Khentet Hrt",
        ecliptic_longitude: 10.0,
    },
    Decan {
        number: 3,
        name: "Khentet Khrt",
        ecliptic_longitude: 20.0,
    },
    Decan {
        number: 4,
        name: "Hat Djat",
        ecliptic_longitude: 30.0,
    },
    Decan {
        number: 5,
        name: "Pehui Djat",
        ecliptic_longitude: 40.0,
    },
    Decan {
        number: 6,
        name: "Temat Hrt",
        ecliptic_longitude: 50.0,
    },
    Decan {
        number: 7,
        name: "Temat Khrt",
        ecliptic_longitude: 60.0,
    },
    Decan {
        number: 8,
        name: "Ushat",
        ecliptic_longitude: 70.0,
    },
    Decan {
        number: 9,
        name: "Bekati",
        ecliptic_longitude: 80.0,
    },
    Decan {
        number: 10,
        name: "Tepai",
        ecliptic_longitude: 90.0,
    },
    Decan {
        number: 11,
        name: "Khentu Hrt",
        ecliptic_longitude: 100.0,
    },
    Decan {
        number: 12,
        name: "Khentu Khrt",
        ecliptic_longitude: 110.0,
    },
    Decan {
        number: 13,
        name: "Sapt Khenmet",
        ecliptic_longitude: 120.0,
    },
    Decan {
        number: 14,
        name: "Khenmet",
        ecliptic_longitude: 130.0,
    },
    Decan {
        number: 15,
        name: "Seshmu",
        ecliptic_longitude: 140.0,
    },
    Decan {
        number: 16,
        name: "Kenmu",
        ecliptic_longitude: 150.0,
    },
    Decan {
        number: 17,
        name: "Semed",
        ecliptic_longitude: 160.0,
    },
    Decan {
        number: 18,
        name: "Seret",
        ecliptic_longitude: 170.0,
    },
    Decan {
        number: 19,
        name: "Sah",
        ecliptic_longitude: 180.0,
    },
    Decan {
        number: 20,
        name: "Sopdet",
        ecliptic_longitude: 190.0,
    },
    Decan {
        number: 21,
        name: "Knmt",
        ecliptic_longitude: 200.0,
    },
    Decan {
        number: 22,
        name: "Sah Sapt",
        ecliptic_longitude: 210.0,
    },
    Decan {
        number: 23,
        name: "Tepy Khentet",
        ecliptic_longitude: 220.0,
    },
    Decan {
        number: 24,
        name: "Khentet Hrt S",
        ecliptic_longitude: 230.0,
    },
    Decan {
        number: 25,
        name: "Khentet Khrt S",
        ecliptic_longitude: 240.0,
    },
    Decan {
        number: 26,
        name: "Apt Hnt",
        ecliptic_longitude: 250.0,
    },
    Decan {
        number: 27,
        name: "Ipds",
        ecliptic_longitude: 260.0,
    },
    Decan {
        number: 28,
        name: "Sba N Hry Ib",
        ecliptic_longitude: 270.0,
    },
    Decan {
        number: 29,
        name: "Kher Khept Khentet",
        ecliptic_longitude: 280.0,
    },
    Decan {
        number: 30,
        name: "Tepy Ahui",
        ecliptic_longitude: 290.0,
    },
    Decan {
        number: 31,
        name: "Ahui",
        ecliptic_longitude: 300.0,
    },
    Decan {
        number: 32,
        name: "Pehui Ahui",
        ecliptic_longitude: 310.0,
    },
    Decan {
        number: 33,
        name: "Tepy Baka",
        ecliptic_longitude: 320.0,
    },
    Decan {
        number: 34,
        name: "Baka",
        ecliptic_longitude: 330.0,
    },
    Decan {
        number: 35,
        name: "Tepy Akhui",
        ecliptic_longitude: 340.0,
    },
    Decan {
        number: 36,
        name: "Akhui",
        ecliptic_longitude: 350.0,
    },
];

/// Find the decan for a given ecliptic longitude.
///
/// Each decan spans 10 degrees. The input is normalized to 0..360.
#[must_use]
pub fn decan_from_longitude(degrees: f64) -> &'static Decan {
    // Normalize to 0..360
    let normalized = ((degrees % 360.0) + 360.0) % 360.0;
    let index = (normalized / 10.0) as usize;
    // Clamp to valid range (handles edge case of exactly 360.0)
    let index = if index >= 36 { 35 } else { index };
    &DECANS[index]
}

// ---------------------------------------------------------------------------
// Sothic cycle (Sopdet / Sirius)
// ---------------------------------------------------------------------------

/// The Sothic cycle length in Julian years: 1,461 civil years = 1,460 Julian years.
///
/// The Egyptian civil calendar had exactly 365 days (no leap year). The true
/// solar year is ~365.25 days. This 0.25-day annual drift accumulates until
/// the heliacal rising of Sopdet (Sirius) returns to the same civil calendar
/// date after 1,461 civil years (365 * 1461 = 365.25 * 1460 = 533,265 days).
///
/// The Sothic cycle was the backbone of Egyptian chronology. The Roman writer
/// Censorinus recorded a Sothic rising on 1 Thoth in 139 CE, allowing
/// historians to anchor the entire Egyptian calendar.
pub const SOTHIC_CYCLE_CIVIL_YEARS: u32 = 1461;

/// Days in one Sothic cycle: 365 * 1461 = 533,265 days.
pub const SOTHIC_CYCLE_DAYS: u64 = 533_265;

/// Annual drift of the civil calendar against the heliacal rising of
/// Sopdet, in days per civil year. Approximately 0.25 days/year.
pub const SOTHIC_DRIFT_PER_YEAR: f64 = 0.25;

/// The Julian Day Number of the Censorinus epoch: 1 Thoth coincided with
/// the heliacal rising of Sopdet on July 20, 139 CE (Julian).
///
/// JDN 1,772,028.5 = July 20, 139 CE (Julian calendar).
/// This is the most securely dated Sothic rising in the historical record.
pub const CENSORINUS_EPOCH_JDN: f64 = 1_772_028.5;

/// Return a reference to the Sopdet decan — Sirius, the brightest star,
/// whose heliacal rising governed the Egyptian calendar and heralded
/// the annual Nile flood (Akhet season).
///
/// Sopdet is decan #20 (ecliptic longitude ~190 degrees).
#[must_use]
#[inline]
pub fn sopdet() -> &'static Decan {
    &DECANS[19] // 0-indexed: decan #20
}

/// Compute the drift in days between the civil calendar and the heliacal
/// rising of Sopdet for a given number of years since a Sothic epoch.
///
/// At a Sothic epoch (e.g., 139 CE Censorinus), the drift is zero.
/// Each year, the civil calendar falls behind by ~0.25 days.
/// After 1,461 civil years the cycle resets.
///
/// Returns the drift in days (0.0 to ~365.25).
#[must_use]
pub fn sothic_drift(years_since_epoch: u32) -> f64 {
    let years_in_cycle = years_since_epoch % SOTHIC_CYCLE_CIVIL_YEARS;
    f64::from(years_in_cycle) * SOTHIC_DRIFT_PER_YEAR
}

/// Determine the position within the current Sothic cycle for a given
/// Julian Day Number, referenced to the Censorinus epoch (139 CE).
///
/// Returns `(cycle_number, year_within_cycle, drift_days)`:
/// - `cycle_number`: which Sothic cycle (0 = the cycle containing 139 CE)
/// - `year_within_cycle`: civil year position (0-1460) within the cycle
/// - `drift_days`: accumulated calendar drift in days
#[must_use]
pub fn sothic_position(jdn: f64) -> (i32, u32, f64) {
    let days_from_epoch = jdn - CENSORINUS_EPOCH_JDN;
    let civil_years_from_epoch = days_from_epoch / 365.0;

    let cycle_number =
        (civil_years_from_epoch / f64::from(SOTHIC_CYCLE_CIVIL_YEARS)).floor() as i32;
    let year_in_cycle = ((civil_years_from_epoch % f64::from(SOTHIC_CYCLE_CIVIL_YEARS))
        + f64::from(SOTHIC_CYCLE_CIVIL_YEARS))
        % f64::from(SOTHIC_CYCLE_CIVIL_YEARS);
    let year_in_cycle_u32 = year_in_cycle as u32;
    let drift = sothic_drift(year_in_cycle_u32);

    (cycle_number, year_in_cycle_u32, drift)
}

/// Predict the Julian Day Number of the next heliacal rising of Sopdet
/// after a given JDN, for a given geographic latitude.
///
/// The heliacal rising of Sirius occurs when it first becomes visible
/// at dawn after its period of invisibility (conjunction with the Sun).
/// At Memphis (30 degrees N), this historically occurred around July 19
/// (Julian). The date shifts by roughly 1 day per degree of latitude.
///
/// This is an approximation based on the Sothic cycle drift model.
/// For precise astronomical computation, external ephemeris data is needed.
///
/// # Errors
///
/// Returns [`SankhyaError::InvalidDate`] if the latitude is outside
/// the observable range for Sirius (-60 to +60 degrees).
pub fn next_sopdet_rising(jdn: f64, latitude: f64) -> Result<f64> {
    if !(-60.0..=60.0).contains(&latitude) {
        return Err(SankhyaError::InvalidDate(format!(
            "latitude {latitude} outside observable range for Sirius (-60 to +60)"
        )));
    }

    // Base heliacal rising at Memphis (30 degrees N): ~July 19 Julian
    // Approximate: JDN offset from Jan 1 to July 19 = 200 days
    // Latitude adjustment: roughly +1 day per degree north of 30 degrees N
    let latitude_offset = latitude - 30.0;

    // Find the year of the input JDN relative to J2000.0
    let year_approx = 2000.0 + (jdn - 2_451_545.0) / 365.25;
    let jan1_jdn = 2_451_545.0 + (year_approx.floor() - 2000.0) * 365.25;

    // Approximate heliacal rising for this year
    let rising_jdn = jan1_jdn + 200.0 + latitude_offset;

    // If the rising already passed this year, return next year's
    if rising_jdn <= jdn {
        Ok(rising_jdn + 365.25)
    } else {
        Ok(rising_jdn)
    }
}

// ---------------------------------------------------------------------------
// Hieroglyphic numeral display (requires lipi)
// ---------------------------------------------------------------------------

/// Render a number in Egyptian hieroglyphic notation.
///
/// The Egyptian system is additive decimal: each power of 10 has a distinct
/// hieroglyph, and the number is composed by repeating symbols. For example,
/// 23 = 𓎆𓎆𓏺𓏺𓏺 (two heel-bones + three strokes).
///
/// Symbols (from the Rhind Papyrus and temple inscriptions):
/// - 𓏺 = 1 (stroke)
/// - 𓎆 = 10 (heel bone)
/// - 𓍢 = 100 (coil of rope)
/// - 𓆼 = 1,000 (lotus flower)
/// - 𓂭 = 10,000 (bent finger)
/// - 𓆐 = 100,000 (tadpole)
/// - 𓁨 = 1,000,000 (god Heh)
///
/// Requires the `lipi` feature.
///
/// # Errors
///
/// Returns [`SankhyaError::OverflowError`] if the number exceeds what the
/// hieroglyphic system can represent (> 9,999,999).
#[cfg(feature = "lipi")]
#[must_use = "returns the hieroglyphic string without side effects"]
pub fn to_hieroglyphic(n: u64) -> Result<String> {
    if n == 0 {
        return Ok(String::new());
    }
    if n > 9_999_999 {
        return Err(SankhyaError::OverflowError(format!(
            "cannot represent {n} in Egyptian hieroglyphic numerals (max 9,999,999)"
        )));
    }

    let system = lipi::script::numerals::egyptian_hieroglyphic();
    let powers: &[u64] = &[1_000_000, 100_000, 10_000, 1_000, 100, 10, 1];
    let mut result = String::new();
    let mut remainder = n;

    for &power in powers {
        let count = remainder / power;
        remainder %= power;
        if count > 0
            && let Some(ch) = system.char_for(power as u32)
        {
            for _ in 0..count {
                result.push_str(ch);
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decompose_two_thirds() {
        let result = decompose(2, 3).unwrap();
        // 2/3 = 1/2 + 1/6
        assert_eq!(result, vec![2, 6]);
        // Verify: 1/2 + 1/6 = 3/6 + 1/6 = 4/6 = 2/3
        let sum: f64 = result.iter().map(|&d| 1.0 / d as f64).sum();
        assert!((sum - 2.0 / 3.0).abs() < 1e-15);
    }

    #[test]
    fn decompose_unit_fraction() {
        let result = decompose(1, 5).unwrap();
        assert_eq!(result, vec![5]);
    }

    #[test]
    fn multiply_12_13() {
        assert_eq!(egyptian_multiply(12, 13), 156);
    }

    #[test]
    fn multiply_commutative() {
        assert_eq!(egyptian_multiply(7, 11), egyptian_multiply(11, 7));
    }

    #[test]
    fn divide_7_3() {
        let (q, rem) = egyptian_divide(7, 3).unwrap();
        assert_eq!(q, 2);
        assert_eq!(rem, vec![3]); // remainder 1/3
    }

    #[test]
    fn decan_lookup() {
        let d = decan_from_longitude(0.0);
        assert_eq!(d.number, 1);
        assert_eq!(d.name, "Kenmet");

        let d = decan_from_longitude(195.0);
        assert_eq!(d.number, 20); // Sopdet (190-200)
    }

    #[test]
    fn decan_negative_longitude() {
        let d = decan_from_longitude(-10.0);
        assert_eq!(d.number, 36); // 350 degrees
    }

    // -- Sothic cycle tests --

    #[test]
    fn sopdet_is_decan_20() {
        let s = sopdet();
        assert_eq!(s.number, 20);
        assert_eq!(s.name, "Sopdet");
    }

    #[test]
    fn sothic_drift_at_epoch_is_zero() {
        let drift = sothic_drift(0);
        assert!((drift - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn sothic_drift_after_4_years() {
        // 4 years * 0.25 days/year = 1 day drift
        let drift = sothic_drift(4);
        assert!((drift - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn sothic_drift_full_cycle_resets() {
        // At exactly 1461 years, the cycle resets
        let drift = sothic_drift(SOTHIC_CYCLE_CIVIL_YEARS);
        assert!((drift - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn sothic_position_at_censorinus_epoch() {
        let (cycle, year, drift) = sothic_position(CENSORINUS_EPOCH_JDN);
        assert_eq!(cycle, 0);
        assert_eq!(year, 0);
        assert!((drift - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn sothic_cycle_days_consistent() {
        // 365 * 1461 = 533,265
        assert_eq!(365 * u64::from(SOTHIC_CYCLE_CIVIL_YEARS), SOTHIC_CYCLE_DAYS);
    }

    #[test]
    fn next_sopdet_rising_memphis() {
        // At Memphis (30 degrees N), should return a valid JDN
        let rising = next_sopdet_rising(2_451_545.0, 30.0).unwrap(); // J2000.0
        assert!(rising > 2_451_545.0);
    }

    #[test]
    fn next_sopdet_rising_invalid_latitude() {
        assert!(next_sopdet_rising(2_451_545.0, 80.0).is_err());
    }

    #[cfg(feature = "lipi")]
    mod hieroglyphic_tests {
        use super::*;

        #[test]
        fn hieroglyphic_single_digit() {
            let s = to_hieroglyphic(3).unwrap();
            assert_eq!(s, "𓏺𓏺𓏺");
        }

        #[test]
        fn hieroglyphic_mixed() {
            // 23 = two 10s + three 1s
            let s = to_hieroglyphic(23).unwrap();
            assert_eq!(s, "𓎆𓎆𓏺𓏺𓏺");
        }

        #[test]
        fn hieroglyphic_powers() {
            let s = to_hieroglyphic(1_000).unwrap();
            assert_eq!(s, "𓆼");
            let s = to_hieroglyphic(1_000_000).unwrap();
            assert_eq!(s, "𓁨");
        }

        #[test]
        fn hieroglyphic_zero() {
            assert_eq!(to_hieroglyphic(0).unwrap(), "");
        }

        #[test]
        fn hieroglyphic_overflow() {
            assert!(to_hieroglyphic(10_000_000).is_err());
        }
    }
}
