//! Roman numeral system.
//!
//! Implements conversion between Roman numerals and decimal values,
//! Roman numeral arithmetic, and validation of proper subtractive notation.
//!
//! # Historical Context
//!
//! Roman numerals emerged in ancient Rome (c. 500 BCE) and remained the
//! dominant numeral system in Europe until the late Middle Ages. The system
//! is additive-subtractive: I=1, V=5, X=10, L=50, C=100, D=500, M=1000.
//! Subtractive pairs (IV=4, IX=9, XL=40, XC=90, CD=400, CM=900) were
//! standardized in medieval usage. The system has no zero and no place value,
//! making arithmetic cumbersome compared to positional systems — a key reason
//! Hindu-Arabic numerals eventually replaced it.
//!
//! # Sources
//!
//! - Ifrah, *The Universal History of Numbers* (Wiley, 2000), ch. 16
//! - Subtractive notation rules from CIL (Corpus Inscriptionum Latinarum)
//!   conventions, standardized in medieval manuscripts

use serde::{Deserialize, Serialize};

use crate::error::{Result, SankhyaError};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// The maximum value representable in standard Roman numerals.
/// Values above 3999 require non-standard extensions (vinculum, etc.).
pub const MAX_VALUE: u32 = 3999;

/// Ordered table of Roman numeral symbols and their values (descending).
const ROMAN_TABLE: &[(u32, &str)] = &[
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

// ---------------------------------------------------------------------------
// Core type
// ---------------------------------------------------------------------------

/// A Roman numeral with its string representation and decimal value.
///
/// Guaranteed to be in the range 1–3999 and in canonical (standard
/// subtractive) form.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RomanNumeral {
    /// The decimal value (1–3999).
    value: u32,
    /// The canonical Roman numeral string.
    text: String,
}

impl RomanNumeral {
    /// Create a Roman numeral from a decimal value.
    ///
    /// # Errors
    ///
    /// Returns [`SankhyaError::InvalidBase`] if `value` is 0 or exceeds 3999.
    #[must_use = "returns the numeral or an error"]
    pub fn from_value(value: u32) -> Result<Self> {
        if value == 0 || value > MAX_VALUE {
            return Err(SankhyaError::InvalidBase(format!(
                "Roman numeral value {value} out of range 1..{MAX_VALUE}"
            )));
        }
        Ok(Self {
            text: to_roman(value),
            value,
        })
    }

    /// Parse a Roman numeral string.
    ///
    /// Accepts both uppercase and lowercase input. The string is validated
    /// for correct subtractive notation and character set.
    ///
    /// # Errors
    ///
    /// Returns [`SankhyaError::InvalidBase`] if the string contains invalid
    /// characters or is not a valid Roman numeral.
    #[must_use = "returns the numeral or an error"]
    pub fn parse(s: &str) -> Result<Self> {
        let value = from_roman(s)?;
        Ok(Self {
            text: to_roman(value),
            value,
        })
    }

    /// The decimal value of this numeral.
    #[must_use]
    #[inline]
    pub fn value(&self) -> u32 {
        self.value
    }

    /// The canonical Roman numeral string.
    #[must_use]
    #[inline]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Add two Roman numerals, returning a new numeral.
    ///
    /// # Errors
    ///
    /// Returns [`SankhyaError::OverflowError`] if the sum exceeds 3999.
    #[must_use = "returns the sum or an error"]
    pub fn add(&self, other: &Self) -> Result<Self> {
        roman_add(self.value, other.value)
    }

    /// Subtract another Roman numeral from this one.
    ///
    /// # Errors
    ///
    /// Returns [`SankhyaError::ComputationError`] if the result would be
    /// zero or negative (Roman numerals have no zero).
    #[must_use = "returns the difference or an error"]
    pub fn subtract(&self, other: &Self) -> Result<Self> {
        roman_subtract(self.value, other.value)
    }

    /// Multiply two Roman numerals.
    ///
    /// # Errors
    ///
    /// Returns [`SankhyaError::OverflowError`] if the product exceeds 3999.
    #[must_use = "returns the product or an error"]
    pub fn multiply(&self, other: &Self) -> Result<Self> {
        roman_multiply(self.value, other.value)
    }

    /// Divide this numeral by another, returning quotient and remainder.
    ///
    /// # Errors
    ///
    /// Returns [`SankhyaError::InvalidFraction`] if the divisor is zero
    /// (which cannot happen with valid `RomanNumeral` values, but guards
    /// against future API changes).
    #[must_use = "returns the quotient and remainder or an error"]
    pub fn divide(&self, other: &Self) -> Result<(Self, Option<Self>)> {
        roman_divide(self.value, other.value)
    }
}

impl core::fmt::Display for RomanNumeral {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.text)
    }
}

// ---------------------------------------------------------------------------
// Conversion functions
// ---------------------------------------------------------------------------

/// Convert a decimal value (1–3999) to a Roman numeral string.
///
/// Returns the canonical subtractive form (e.g., 4 → "IV", not "IIII").
///
/// # Panics
///
/// This is an internal function that assumes valid input. Use
/// [`RomanNumeral::from_value`] for validated conversion.
#[must_use]
fn to_roman(mut n: u32) -> String {
    let mut result = String::new();
    for &(val, sym) in ROMAN_TABLE {
        while n >= val {
            result.push_str(sym);
            n -= val;
        }
    }
    result
}

/// Convert a decimal value to a Roman numeral string.
///
/// # Errors
///
/// Returns [`SankhyaError::InvalidBase`] if `value` is 0 or exceeds 3999.
#[must_use = "returns the Roman numeral string or an error"]
pub fn to_roman_str(value: u32) -> Result<String> {
    if value == 0 || value > MAX_VALUE {
        return Err(SankhyaError::InvalidBase(format!(
            "Roman numeral value {value} out of range 1..{MAX_VALUE}"
        )));
    }
    Ok(to_roman(value))
}

/// Parse a Roman numeral string to its decimal value.
///
/// Accepts uppercase and lowercase. Validates that:
/// - All characters are valid Roman numeral symbols
/// - The string represents a value in the range 1–3999
/// - The parsed value round-trips to the canonical form
///
/// # Errors
///
/// Returns [`SankhyaError::InvalidBase`] if the input is empty, contains
/// invalid characters, or is not a valid Roman numeral.
#[must_use = "returns the decimal value or an error"]
pub fn from_roman(s: &str) -> Result<u32> {
    if s.is_empty() {
        return Err(SankhyaError::InvalidBase(
            "empty string is not a valid Roman numeral".into(),
        ));
    }

    let upper = s.to_uppercase();
    let mut total: u32 = 0;
    let mut prev_value: u32 = 0;

    for ch in upper.chars().rev() {
        let val = char_value(ch)?;
        if val < prev_value {
            total = total
                .checked_sub(val)
                .ok_or_else(|| SankhyaError::ComputationError("Roman numeral underflow".into()))?;
        } else {
            total = total
                .checked_add(val)
                .ok_or_else(|| SankhyaError::OverflowError("Roman numeral overflow".into()))?;
        }
        prev_value = val;
    }

    if total == 0 || total > MAX_VALUE {
        return Err(SankhyaError::InvalidBase(format!(
            "Roman numeral '{s}' produces value {total} out of range 1..{MAX_VALUE}"
        )));
    }

    // Validate canonical form: round-trip check
    let canonical = to_roman(total);
    if canonical != upper {
        return Err(SankhyaError::InvalidBase(format!(
            "'{s}' is not canonical Roman notation (expected '{canonical}' for {total})"
        )));
    }

    Ok(total)
}

/// Map a single Roman numeral character to its value.
fn char_value(ch: char) -> Result<u32> {
    match ch {
        'I' => Ok(1),
        'V' => Ok(5),
        'X' => Ok(10),
        'L' => Ok(50),
        'C' => Ok(100),
        'D' => Ok(500),
        'M' => Ok(1000),
        _ => Err(SankhyaError::InvalidBase(format!(
            "'{ch}' is not a valid Roman numeral character"
        ))),
    }
}

// ---------------------------------------------------------------------------
// Arithmetic
// ---------------------------------------------------------------------------

/// Add two values and return the result as a Roman numeral.
///
/// # Errors
///
/// Returns [`SankhyaError::OverflowError`] if the sum exceeds 3999.
#[must_use = "returns the sum or an error"]
pub fn roman_add(a: u32, b: u32) -> Result<RomanNumeral> {
    let sum = a.checked_add(b).ok_or_else(|| {
        SankhyaError::OverflowError(format!("Roman addition overflow: {a} + {b}"))
    })?;
    RomanNumeral::from_value(sum)
}

/// Subtract b from a and return the result as a Roman numeral.
///
/// # Errors
///
/// Returns [`SankhyaError::ComputationError`] if a ≤ b (Roman numerals
/// have no zero or negative values).
#[must_use = "returns the difference or an error"]
pub fn roman_subtract(a: u32, b: u32) -> Result<RomanNumeral> {
    if a <= b {
        return Err(SankhyaError::ComputationError(format!(
            "Roman subtraction would produce zero or negative: {a} - {b}"
        )));
    }
    RomanNumeral::from_value(a - b)
}

/// Multiply two values and return the result as a Roman numeral.
///
/// # Errors
///
/// Returns [`SankhyaError::OverflowError`] if the product exceeds 3999.
#[must_use = "returns the product or an error"]
pub fn roman_multiply(a: u32, b: u32) -> Result<RomanNumeral> {
    let product = a.checked_mul(b).ok_or_else(|| {
        SankhyaError::OverflowError(format!("Roman multiplication overflow: {a} * {b}"))
    })?;
    RomanNumeral::from_value(product)
}

/// Divide a by b, returning (quotient, remainder) as Roman numerals.
///
/// The remainder is `None` if the division is exact.
///
/// # Errors
///
/// Returns [`SankhyaError::InvalidFraction`] if `b` is zero.
/// Returns [`SankhyaError::ComputationError`] if the quotient is zero
/// (i.e., a < b).
#[must_use = "returns the quotient and remainder or an error"]
pub fn roman_divide(a: u32, b: u32) -> Result<(RomanNumeral, Option<RomanNumeral>)> {
    if b == 0 {
        return Err(SankhyaError::InvalidFraction("division by zero".into()));
    }

    let quotient = a / b;
    let remainder = a % b;

    if quotient == 0 {
        return Err(SankhyaError::ComputationError(format!(
            "Roman division {a} / {b} produces zero quotient"
        )));
    }

    let q = RomanNumeral::from_value(quotient)?;
    let r = if remainder > 0 {
        Some(RomanNumeral::from_value(remainder)?)
    } else {
        None
    };

    Ok((q, r))
}

// ---------------------------------------------------------------------------
// Utility functions
// ---------------------------------------------------------------------------

/// Check if a string is a valid Roman numeral in canonical form.
#[must_use]
pub fn is_valid_roman(s: &str) -> bool {
    from_roman(s).is_ok()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- Conversion --

    #[test]
    fn basic_values() {
        assert_eq!(to_roman_str(1).unwrap(), "I");
        assert_eq!(to_roman_str(4).unwrap(), "IV");
        assert_eq!(to_roman_str(5).unwrap(), "V");
        assert_eq!(to_roman_str(9).unwrap(), "IX");
        assert_eq!(to_roman_str(10).unwrap(), "X");
        assert_eq!(to_roman_str(40).unwrap(), "XL");
        assert_eq!(to_roman_str(50).unwrap(), "L");
        assert_eq!(to_roman_str(90).unwrap(), "XC");
        assert_eq!(to_roman_str(100).unwrap(), "C");
        assert_eq!(to_roman_str(400).unwrap(), "CD");
        assert_eq!(to_roman_str(500).unwrap(), "D");
        assert_eq!(to_roman_str(900).unwrap(), "CM");
        assert_eq!(to_roman_str(1000).unwrap(), "M");
    }

    #[test]
    fn compound_values() {
        assert_eq!(to_roman_str(14).unwrap(), "XIV");
        assert_eq!(to_roman_str(42).unwrap(), "XLII");
        assert_eq!(to_roman_str(99).unwrap(), "XCIX");
        assert_eq!(to_roman_str(399).unwrap(), "CCCXCIX");
        assert_eq!(to_roman_str(1776).unwrap(), "MDCCLXXVI");
        assert_eq!(to_roman_str(1999).unwrap(), "MCMXCIX");
        assert_eq!(to_roman_str(2024).unwrap(), "MMXXIV");
        assert_eq!(to_roman_str(3999).unwrap(), "MMMCMXCIX");
    }

    #[test]
    fn parse_roundtrip() {
        for n in 1..=3999 {
            let roman = to_roman_str(n).unwrap();
            let parsed = from_roman(&roman).unwrap();
            assert_eq!(parsed, n, "roundtrip failed for {n} = {roman}");
        }
    }

    #[test]
    fn parse_lowercase() {
        assert_eq!(from_roman("xiv").unwrap(), 14);
        assert_eq!(from_roman("mcmxcix").unwrap(), 1999);
    }

    #[test]
    fn parse_invalid_chars() {
        assert!(from_roman("ABC").is_err());
        assert!(from_roman("IVX2").is_err());
    }

    #[test]
    fn parse_non_canonical() {
        // IIII should be IV
        assert!(from_roman("IIII").is_err());
        // VV should be X
        assert!(from_roman("VV").is_err());
        // IC is not standard (should be XCIX for 99)
        assert!(from_roman("IC").is_err());
    }

    #[test]
    fn parse_empty() {
        assert!(from_roman("").is_err());
    }

    #[test]
    fn value_zero_errors() {
        assert!(to_roman_str(0).is_err());
        assert!(RomanNumeral::from_value(0).is_err());
    }

    #[test]
    fn value_too_large_errors() {
        assert!(to_roman_str(4000).is_err());
        assert!(RomanNumeral::from_value(4000).is_err());
    }

    // -- Arithmetic --

    #[test]
    fn add_basic() {
        let r = roman_add(10, 5).unwrap();
        assert_eq!(r.value(), 15);
        assert_eq!(r.text(), "XV");
    }

    #[test]
    fn add_overflow() {
        assert!(roman_add(3999, 1).is_err());
    }

    #[test]
    fn subtract_basic() {
        let r = roman_subtract(10, 3).unwrap();
        assert_eq!(r.value(), 7);
        assert_eq!(r.text(), "VII");
    }

    #[test]
    fn subtract_to_zero_errors() {
        assert!(roman_subtract(5, 5).is_err());
        assert!(roman_subtract(3, 7).is_err());
    }

    #[test]
    fn multiply_basic() {
        let r = roman_multiply(7, 8).unwrap();
        assert_eq!(r.value(), 56);
        assert_eq!(r.text(), "LVI");
    }

    #[test]
    fn multiply_overflow() {
        assert!(roman_multiply(100, 100).is_err());
    }

    #[test]
    fn divide_exact() {
        let (q, r) = roman_divide(10, 5).unwrap();
        assert_eq!(q.value(), 2);
        assert!(r.is_none());
    }

    #[test]
    fn divide_with_remainder() {
        let (q, r) = roman_divide(10, 3).unwrap();
        assert_eq!(q.value(), 3);
        let rem = r.unwrap();
        assert_eq!(rem.value(), 1);
    }

    #[test]
    fn divide_by_zero_errors() {
        assert!(roman_divide(10, 0).is_err());
    }

    #[test]
    fn divide_zero_quotient_errors() {
        assert!(roman_divide(3, 5).is_err());
    }

    // -- RomanNumeral type --

    #[test]
    fn numeral_methods() {
        let x = RomanNumeral::from_value(42).unwrap();
        assert_eq!(x.value(), 42);
        assert_eq!(x.text(), "XLII");
        assert_eq!(x.to_string(), "XLII");

        let y = RomanNumeral::parse("XIV").unwrap();
        assert_eq!(y.value(), 14);
    }

    #[test]
    fn numeral_arithmetic() {
        let a = RomanNumeral::from_value(100).unwrap();
        let b = RomanNumeral::from_value(50).unwrap();

        assert_eq!(a.add(&b).unwrap().value(), 150);
        assert_eq!(a.subtract(&b).unwrap().value(), 50);
        assert!(a.multiply(&b).is_err()); // 100 * 50 = 5000 > 3999

        let (q, r) = a.divide(&b).unwrap();
        assert_eq!(q.value(), 2);
        assert!(r.is_none());
    }

    #[test]
    fn is_valid() {
        assert!(is_valid_roman("XIV"));
        assert!(is_valid_roman("mcmxcix"));
        assert!(!is_valid_roman("IIII"));
        assert!(!is_valid_roman("ABC"));
        assert!(!is_valid_roman(""));
    }

    // -- Serde --

    #[test]
    fn serde_roundtrip() {
        let n = RomanNumeral::from_value(1776).unwrap();
        let json = serde_json::to_string(&n).unwrap();
        let back: RomanNumeral = serde_json::from_str(&json).unwrap();
        assert_eq!(n, back);
    }
}
