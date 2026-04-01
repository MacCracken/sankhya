//! Babylonian mathematics.
//!
//! Implements the sexagesimal (base-60) number system, Saros eclipse cycle,
//! reciprocal tables for regular numbers, Plimpton 322 Pythagorean triples,
//! and the Babylonian square root method (Heron's method).
//!
//! # Historical Context
//!
//! The Babylonians (c. 2000-300 BCE) developed one of the earliest
//! positional number systems using base 60. This survives today in our
//! 60-minute hours and 360-degree circles. They compiled extensive
//! mathematical tables on clay tablets, including the famous Plimpton 322
//! tablet containing Pythagorean triples, predating Pythagoras by over
//! a millennium.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::error::{Result, SankhyaError};

// ---------------------------------------------------------------------------
// Sexagesimal (base-60) number system
// ---------------------------------------------------------------------------

/// Convert a decimal number to sexagesimal (base-60) digits, most significant first.
#[must_use]
pub fn to_sexagesimal(mut n: u64) -> Vec<u8> {
    if n == 0 {
        return vec![0];
    }
    let mut digits = Vec::new();
    while n > 0 {
        digits.push((n % 60) as u8);
        n /= 60;
    }
    digits.reverse();
    digits
}

/// Convert sexagesimal (base-60) digits back to a decimal number.
///
/// # Errors
///
/// Returns [`SankhyaError::InvalidBase`] if any digit is >= 60.
pub fn from_sexagesimal(digits: &[u8]) -> Result<u64> {
    let mut result: u64 = 0;
    for &d in digits {
        if d >= 60 {
            return Err(SankhyaError::InvalidBase(format!(
                "sexagesimal digit {d} out of range 0..60"
            )));
        }
        result = result
            .checked_mul(60)
            .and_then(|r| r.checked_add(u64::from(d)))
            .ok_or_else(|| SankhyaError::OverflowError("sexagesimal conversion overflow".into()))?;
    }
    Ok(result)
}

// ---------------------------------------------------------------------------
// Babylonian numeral
// ---------------------------------------------------------------------------

/// A single Babylonian sexagesimal digit (0-59).
///
/// Babylonian cuneiform used two symbols: a vertical wedge (units, 1-9)
/// and a corner wedge (tens, 10-50). Each digit 0-59 is composed of
/// some number of tens and units wedges.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BabylonianNumeral {
    /// Number of ten-wedges (0-5).
    pub tens: u8,
    /// Number of unit-wedges (0-9).
    pub units: u8,
}

impl BabylonianNumeral {
    /// Create a numeral from a value 0-59.
    ///
    /// # Errors
    ///
    /// Returns [`SankhyaError::InvalidBase`] if value >= 60.
    pub fn from_value(value: u8) -> Result<Self> {
        if value >= 60 {
            return Err(SankhyaError::InvalidBase(format!(
                "Babylonian digit {value} out of range 0..60"
            )));
        }
        Ok(Self {
            tens: value / 10,
            units: value % 10,
        })
    }

    /// The decimal value of this numeral (0-59).
    #[must_use]
    #[inline]
    pub fn value(self) -> u8 {
        self.tens * 10 + self.units
    }
}

// ---------------------------------------------------------------------------
// Saros cycle
// ---------------------------------------------------------------------------

/// The Saros cycle in days: approximately 6585.32 days (223 synodic months).
///
/// The Babylonians discovered that eclipses repeat after 223 synodic months
/// (6585 days, 7 hours, 43 minutes). This was recorded on the "Saros Canon"
/// tablets found at Babylon, dating to around 500 BCE.
pub const SAROS_DAYS: f64 = 6585.3211;

/// Predict the Julian Day Number of the next eclipse in the Saros series.
///
/// Given the JDN of an observed eclipse, returns the predicted JDN
/// of the next occurrence one Saros cycle later.
#[must_use]
#[inline]
pub fn saros_cycle(eclipse_jdn: f64) -> f64 {
    eclipse_jdn + SAROS_DAYS
}

// ---------------------------------------------------------------------------
// Reciprocal tables (regular numbers)
// ---------------------------------------------------------------------------

/// Generate the Babylonian reciprocal table for regular numbers up to 81.
///
/// A "regular number" in base 60 is one whose only prime factors are
/// 2, 3, and 5 (the prime factors of 60). These numbers have finite
/// sexagesimal reciprocals.
///
/// Returns a map from regular number to its sexagesimal reciprocal
/// (as a vector of base-60 digits representing the fraction).
///
/// For example: 2 -> \[30\] (meaning 30/60 = 1/2),
///              3 -> \[20\] (meaning 20/60 = 1/3),
///              4 -> \[15\] (meaning 15/60 = 1/4).
#[must_use]
pub fn reciprocal_table() -> BTreeMap<u64, Vec<u8>> {
    // Known Babylonian reciprocal pairs.
    // The reciprocal of n is computed as the sexagesimal representation of 60/n
    // (or 3600/n for two-digit reciprocals, etc.)
    let pairs: &[(u64, &[u8])] = &[
        (2, &[30]),             // 1/2 = 0;30
        (3, &[20]),             // 1/3 = 0;20
        (4, &[15]),             // 1/4 = 0;15
        (5, &[12]),             // 1/5 = 0;12
        (6, &[10]),             // 1/6 = 0;10
        (8, &[7, 30]),          // 1/8 = 0;7,30
        (9, &[6, 40]),          // 1/9 = 0;6,40
        (10, &[6]),             // 1/10 = 0;6
        (12, &[5]),             // 1/12 = 0;5
        (15, &[4]),             // 1/15 = 0;4
        (16, &[3, 45]),         // 1/16 = 0;3,45
        (18, &[3, 20]),         // 1/18 = 0;3,20
        (20, &[3]),             // 1/20 = 0;3
        (24, &[2, 30]),         // 1/24 = 0;2,30
        (25, &[2, 24]),         // 1/25 = 0;2,24
        (27, &[2, 13, 20]),     // 1/27 = 0;2,13,20
        (30, &[2]),             // 1/30 = 0;2
        (32, &[1, 52, 30]),     // 1/32 = 0;1,52,30
        (36, &[1, 40]),         // 1/36 = 0;1,40
        (40, &[1, 30]),         // 1/40 = 0;1,30
        (45, &[1, 20]),         // 1/45 = 0;1,20
        (48, &[1, 15]),         // 1/48 = 0;1,15
        (50, &[1, 12]),         // 1/50 = 0;1,12
        (54, &[1, 6, 40]),      // 1/54 = 0;1,6,40
        (60, &[1]),             // 1/60 = 0;1
        (64, &[0, 56, 15]),     // 1/64 = 0;0,56,15
        (72, &[0, 50]),         // 1/72 = 0;0,50
        (80, &[0, 45]),         // 1/80 = 0;0,45
        (81, &[0, 44, 26, 40]), // 1/81 = 0;0,44,26,40
    ];

    let mut table = BTreeMap::new();
    for &(n, recip) in pairs {
        table.insert(n, recip.to_vec());
    }
    table
}

// ---------------------------------------------------------------------------
// Plimpton 322 Pythagorean triples
// ---------------------------------------------------------------------------

/// Generate the 15 Pythagorean triples from the Plimpton 322 tablet.
///
/// Plimpton 322 is a Babylonian clay tablet (c. 1800 BCE) containing
/// a table of Pythagorean triples. The tablet lists the short side (b),
/// the diagonal (c = hypotenuse), and a ratio column. The triples are
/// returned as (a, b, c) where a^2 + b^2 = c^2.
///
/// These triples were generated using the parametric form:
/// a = p^2 - q^2, b = 2pq, c = p^2 + q^2 for appropriate p, q values.
#[must_use]
pub fn generate_plimpton_triples() -> Vec<(u64, u64, u64)> {
    // The 15 rows of Plimpton 322, reconstructed as (a, b, c).
    // Row ordering follows the tablet (sorted by decreasing angle).
    vec![
        (119, 120, 169),
        (3367, 3456, 4825),
        (4601, 4800, 6649),
        (12709, 13500, 18541),
        (65, 72, 97),
        (319, 360, 481),
        (2291, 2700, 3541),
        (799, 960, 1249),
        (481, 600, 769),
        (4961, 6480, 8161),
        (45, 60, 75),
        (1679, 2400, 2929),
        (161, 240, 289),
        (1771, 2700, 3229),
        (56, 90, 106),
    ]
}

// ---------------------------------------------------------------------------
// Babylonian square root (Heron's method)
// ---------------------------------------------------------------------------

/// Compute the square root using the Babylonian/Heron's method.
///
/// This iterative method was known to the Babylonians as early as
/// 1700 BCE (the YBC 7289 tablet shows sqrt(2) accurate to 6 decimal places).
///
/// The algorithm: starting with an initial guess x, repeatedly compute
/// x = (x + n/x) / 2 until convergence.
///
/// # Errors
///
/// Returns [`SankhyaError::ComputationError`] if `n` is negative.
/// Returns [`SankhyaError::InvalidBase`] if `iterations` is zero.
pub fn babylonian_sqrt(n: f64, iterations: u32) -> Result<f64> {
    if n < 0.0 {
        return Err(SankhyaError::ComputationError(
            "cannot compute square root of negative number".into(),
        ));
    }
    if n == 0.0 {
        return Ok(0.0);
    }
    if iterations == 0 {
        return Err(SankhyaError::InvalidBase(
            "iterations must be at least 1".into(),
        ));
    }
    // Initial guess: n/2 (or 1 if n < 2)
    let mut x = if n < 2.0 { 1.0 } else { n / 2.0 };
    for _ in 0..iterations {
        x = (x + n / x) / 2.0;
    }
    Ok(x)
}

// ---------------------------------------------------------------------------
// Cuneiform display (requires lipi)
// ---------------------------------------------------------------------------

/// Render a sexagesimal digit (0-59) in cuneiform notation.
///
/// Uses the Babylonian cuneiform numeral system from lipi: 𒐕 (diš) for
/// units 1-9, 𒌋/𒌋𒌋/𒌍 for tens 10/20/30. Digits above 30 are composed
/// additively (e.g., 42 = 𒌍 + 𒐖 + 𒌋 = "𒌍𒌋𒐖").
///
/// Returns a space `" "` for zero (Babylonians had no zero symbol in
/// early periods).
///
/// Requires the `lipi` feature.
///
/// # Errors
///
/// Returns [`SankhyaError::InvalidBase`] if `digit` >= 60.
#[cfg(feature = "lipi")]
pub fn cuneiform_digit(digit: u8) -> Result<String> {
    if digit >= 60 {
        return Err(SankhyaError::InvalidBase(format!(
            "cuneiform digit {digit} out of range 0..60"
        )));
    }
    if digit == 0 {
        return Ok(" ".into());
    }

    let system = lipi::script::numerals::babylonian_sexagesimal();
    let tens = digit / 10;
    let units = digit % 10;
    let mut result = String::new();

    // Tens: use the highest available symbol, then compose
    if tens > 0 {
        // Available tens symbols: 10, 20, 30
        let mut remaining_tens = tens;
        for &val in &[30u8, 20, 10] {
            if remaining_tens * 10 >= val
                && let Some(ch) = system.char_for(u32::from(val))
            {
                result.push_str(ch);
                remaining_tens -= val / 10;
            }
            if remaining_tens == 0 {
                break;
            }
        }
    }

    if units > 0
        && let Some(ch) = system.char_for(u32::from(units))
    {
        result.push_str(ch);
    }

    Ok(result)
}

/// Render a full number in cuneiform sexagesimal notation.
///
/// Digits are separated by a middle dot `·` for readability,
/// matching the modern convention for displaying sexagesimal.
///
/// Requires the `lipi` feature.
///
/// # Errors
///
/// Returns [`SankhyaError::InvalidBase`] if any internal digit is invalid.
#[cfg(feature = "lipi")]
pub fn to_cuneiform(n: u64) -> Result<String> {
    let digits = to_sexagesimal(n);
    let mut parts = Vec::with_capacity(digits.len());
    for &d in &digits {
        parts.push(cuneiform_digit(d)?);
    }
    Ok(parts.join("·"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sexagesimal_zero() {
        assert_eq!(to_sexagesimal(0), vec![0]);
        assert_eq!(from_sexagesimal(&[0]).unwrap(), 0);
    }

    #[test]
    fn sexagesimal_roundtrip() {
        for n in [1, 59, 60, 3599, 3600, 216_000, 1_000_000] {
            let digits = to_sexagesimal(n);
            assert_eq!(from_sexagesimal(&digits).unwrap(), n, "failed for {n}");
        }
    }

    #[test]
    fn babylonian_numeral_value() {
        let n = BabylonianNumeral::from_value(42).unwrap();
        assert_eq!(n.tens, 4);
        assert_eq!(n.units, 2);
        assert_eq!(n.value(), 42);
    }

    #[test]
    fn plimpton_triples_valid() {
        let triples = generate_plimpton_triples();
        assert_eq!(triples.len(), 15);
        for (a, b, c) in &triples {
            assert_eq!(a * a + b * b, c * c, "invalid triple: ({a}, {b}, {c})");
        }
    }

    #[test]
    fn sqrt_2_accuracy() {
        let result = babylonian_sqrt(2.0, 10).unwrap();
        assert!((result - std::f64::consts::SQRT_2).abs() < 1e-15);
    }

    #[test]
    fn saros_cycle_test() {
        let next = saros_cycle(2451545.0); // J2000.0
        assert!((next - (2451545.0 + SAROS_DAYS)).abs() < 1e-10);
    }

    #[cfg(feature = "lipi")]
    mod cuneiform_tests {
        use super::*;

        #[test]
        fn cuneiform_digit_units() {
            let s = cuneiform_digit(1).unwrap();
            assert_eq!(s, "𒐕");
            let s = cuneiform_digit(9).unwrap();
            assert_eq!(s, "𒐝");
        }

        #[test]
        fn cuneiform_digit_tens() {
            let s = cuneiform_digit(10).unwrap();
            assert_eq!(s, "𒌋");
            let s = cuneiform_digit(30).unwrap();
            assert_eq!(s, "𒌍");
        }

        #[test]
        fn cuneiform_digit_composite() {
            // 42 = 30 + 10 + 2 = 𒌍𒌋𒐖
            let s = cuneiform_digit(42).unwrap();
            assert!(s.contains("𒌍"));
            assert!(s.contains("𒐖"));
        }

        #[test]
        fn cuneiform_digit_zero() {
            assert_eq!(cuneiform_digit(0).unwrap(), " ");
        }

        #[test]
        fn cuneiform_digit_out_of_range() {
            assert!(cuneiform_digit(60).is_err());
        }

        #[test]
        fn to_cuneiform_basic() {
            // 60 = [1, 0] in sexagesimal
            let s = to_cuneiform(60).unwrap();
            assert!(s.contains('·'));
            assert!(s.contains("𒐕"));
        }
    }
}
