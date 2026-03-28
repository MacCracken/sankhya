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
}
