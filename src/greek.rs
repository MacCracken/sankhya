//! Greek mathematics.
//!
//! Implements the golden ratio, Fibonacci sequence, Sieve of Eratosthenes,
//! Euclidean algorithm, Archimedes' pi approximation, and the Antikythera
//! mechanism gear ratios.
//!
//! # Historical Context
//!
//! Greek mathematics (c. 600 BCE - 300 CE) emphasized deductive proof and
//! geometric construction. Euclid's Elements (c. 300 BCE) systematized
//! number theory and geometry. Eratosthenes' prime sieve and Archimedes'
//! polygon method for approximating pi are still taught today. The
//! Antikythera mechanism (c. 150-100 BCE) is the earliest known analog
//! computer, using bronze gears to predict astronomical positions.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[cfg(feature = "varna")]
use crate::error::{Result, SankhyaError};

// ---------------------------------------------------------------------------
// Golden ratio and Fibonacci
// ---------------------------------------------------------------------------

/// The golden ratio: phi = (1 + sqrt(5)) / 2.
///
/// Known to the ancient Greeks through the geometry of the regular
/// pentagon and appearing in Euclid's Elements (Book VI, Proposition 30)
/// as "extreme and mean ratio."
pub const PHI: f64 = 1.618_033_988_749_895;

/// Compute the ratio of consecutive Fibonacci numbers F(n+1)/F(n),
/// which approaches the golden ratio phi as n increases.
///
/// Returns the ratio for the given n (n >= 1). For n=0, returns 1.0.
#[must_use]
pub fn fibonacci_ratio(n: usize) -> f64 {
    if n == 0 {
        return 1.0;
    }

    let mut a: f64 = 1.0;
    let mut b: f64 = 1.0;
    for _ in 0..n {
        let temp = b;
        b += a;
        a = temp;
    }
    b / a
}

// ---------------------------------------------------------------------------
// Sieve of Eratosthenes
// ---------------------------------------------------------------------------

/// Find all primes up to `limit` using the Sieve of Eratosthenes.
///
/// Eratosthenes of Cyrene (c. 276-194 BCE) devised this algorithm
/// for finding prime numbers. It works by iteratively marking the
/// multiples of each prime starting from 2.
///
/// Returns an empty vec if limit < 2.
#[must_use]
pub fn sieve(limit: usize) -> Vec<usize> {
    if limit < 2 {
        return Vec::new();
    }

    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    if limit >= 1 {
        is_prime[1] = false;
    }

    let mut p = 2;
    while p * p <= limit {
        if is_prime[p] {
            let mut multiple = p * p;
            while multiple <= limit {
                is_prime[multiple] = false;
                multiple += p;
            }
        }
        p += 1;
    }

    is_prime
        .iter()
        .enumerate()
        .filter_map(|(i, &prime)| if prime { Some(i) } else { None })
        .collect()
}

// ---------------------------------------------------------------------------
// Euclidean algorithm
// ---------------------------------------------------------------------------

/// Compute the greatest common divisor using Euclid's algorithm.
///
/// From Euclid's Elements, Book VII, Propositions 1-2 (c. 300 BCE).
/// This is one of the oldest known algorithms still in active use.
///
/// Returns 0 if both inputs are 0.
#[must_use]
#[inline]
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// Compute the least common multiple.
#[must_use]
#[inline]
pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    a / gcd(a, b) * b
}

// ---------------------------------------------------------------------------
// Archimedes' pi approximation
// ---------------------------------------------------------------------------

/// Approximate pi using Archimedes' method of exhaustion.
///
/// Archimedes (c. 287-212 BCE) bounded pi by computing the perimeters
/// of regular polygons inscribed in and circumscribed around a circle.
/// Starting with a hexagon (6 sides) and repeatedly doubling the number
/// of sides, he obtained:
///   3 + 10/71 < pi < 3 + 1/7
///   (3.14084... < pi < 3.14285...)
///
/// `iterations` controls the number of doublings starting from a hexagon.
/// Returns `(lower_bound, upper_bound)`.
#[must_use]
pub fn archimedes_pi(iterations: u32) -> (f64, f64) {
    // Start with a regular hexagon (6 sides)
    // Start with known values for hexagon (6 sides) in unit-radius circle:
    // - Inscribed semi-perimeter: n * sin(pi/n) where n=6: 6 * sin(30 deg) = 6 * 0.5 = 3
    // - Circumscribed semi-perimeter: n * tan(pi/n) where n=6: 6 * tan(30 deg) = 6/sqrt(3) = 2*sqrt(3)

    let mut inner = 3.0_f64; // n * sin(pi/n), initially 6 * sin(pi/6) = 3
    let mut outer = 2.0 * 3.0_f64.sqrt(); // n * tan(pi/n), initially 6 * tan(pi/6) = 2*sqrt(3)

    for _ in 0..iterations {
        // Double the number of sides.
        // Archimedes' recursion:
        // For doubling sides, the new outer (circumscribed) is the harmonic mean:
        //   outer_new = 2 * inner * outer / (inner + outer)
        // The new inner (inscribed) is the geometric mean:
        //   inner_new = sqrt(inner * outer_new)

        let outer_new = 2.0 * inner * outer / (inner + outer);
        let inner_new = (inner * outer_new).sqrt();

        inner = inner_new;
        outer = outer_new;
    }

    // inner < pi < outer
    (inner, outer)
}

// ---------------------------------------------------------------------------
// Greek isopsephy (alphabetic numeral values)
// ---------------------------------------------------------------------------

/// Compute the isopsephy value of a Greek word.
///
/// Isopsephy (Greek: ἰσοψηφία, "equal pebbles") is the practice of assigning
/// numeric values to Greek letters and summing them for a word or phrase.
/// It was widely used in antiquity for numerology, word games, and even
/// cryptographic identification (e.g., the "number of the beast" in
/// Revelation uses Greek isopsephy).
///
/// Each letter maps to a value: α=1, β=2, ... θ=9, ι=10, κ=20, ... π=80,
/// ρ=100, σ=200, ... ω=800. The word value is the additive sum.
///
/// Requires the `varna` feature for the numeral system tables.
///
/// # Errors
///
/// Returns [`SankhyaError::InvalidBase`] if the input is empty or contains
/// characters with no isopsephy mapping.
#[cfg(feature = "varna")]
#[must_use = "returns the computed isopsephy value without side effects"]
pub fn isopsephy(word: &str) -> Result<u32> {
    if word.is_empty() {
        return Err(SankhyaError::InvalidBase(
            "empty string has no isopsephy value".into(),
        ));
    }
    let system = varna::script::numerals::greek_isopsephy();
    system.string_value(word).ok_or_else(|| {
        SankhyaError::InvalidBase(format!(
            "word contains characters with no isopsephy mapping: {word}"
        ))
    })
}

/// Convert a number to its Greek alphabetic numeral representation.
///
/// The Greek alphabetic numeral system represents numbers 1-800 using
/// the 24 letters of the Greek alphabet (plus archaic letters for 6, 90,
/// and 900 in the full system — this implementation covers the standard
/// 24-letter range).
///
/// Numbers are decomposed additively: 358 = τ (300) + ν (50) + η (8) = "τνη".
///
/// Requires the `varna` feature.
///
/// # Errors
///
/// Returns [`SankhyaError::InvalidBase`] if `n` is 0 or exceeds the
/// representable range.
#[cfg(feature = "varna")]
#[must_use = "returns the numeral string without side effects"]
pub fn to_greek_numeral(n: u32) -> Result<String> {
    if n == 0 {
        return Err(SankhyaError::InvalidBase(
            "zero has no Greek numeral representation".into(),
        ));
    }

    let system = varna::script::numerals::greek_isopsephy();

    // Decompose into hundreds, tens, units using available letter values.
    // Available values in descending order from the isopsephy table.
    let available: &[u32] = &[
        800, 700, 600, 500, 400, 300, 200, 100, 80, 70, 60, 50, 40, 30, 20, 10, 9, 8, 7, 5, 4, 3,
        2, 1,
    ];

    let mut remainder = n;
    let mut result = String::new();

    for &val in available {
        if remainder >= val
            && let Some(ch) = system.char_for(val)
        {
            result.push_str(ch);
            remainder -= val;
        }
        if remainder == 0 {
            break;
        }
    }

    if remainder > 0 {
        return Err(SankhyaError::InvalidBase(format!(
            "cannot represent {n} in Greek alphabetic numerals (remainder {remainder})"
        )));
    }

    Ok(result)
}

// ---------------------------------------------------------------------------
// Antikythera mechanism
// ---------------------------------------------------------------------------

/// An astronomical cycle tracked by the Antikythera mechanism.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AntikytheraCycle {
    /// Name of the astronomical cycle.
    pub name: &'static str,
    /// Period in years (approximate).
    pub period_years: &'static str,
    /// Gear tooth count used in the mechanism.
    pub gear_teeth: u32,
    /// Description of the cycle.
    pub description: &'static str,
}

/// Return the known gear ratios of the Antikythera mechanism.
///
/// The Antikythera mechanism (c. 150-100 BCE) is an ancient Greek
/// analog computer found in a shipwreck off the island of Antikythera.
/// It used a system of at least 30 bronze gears to predict:
/// - Solar and lunar positions
/// - Lunar phases
/// - Eclipse predictions (Saros and Exeligmos cycles)
/// - Dates of the Olympic games
///
/// Returns a map from cycle name to gear tooth count and description.
#[must_use]
pub fn antikythera_gear_ratios() -> BTreeMap<&'static str, AntikytheraCycle> {
    let cycles = vec![
        AntikytheraCycle {
            name: "Metonic",
            period_years: "19",
            gear_teeth: 235,
            description: "19 tropical years = 235 synodic months (Meton of Athens, 432 BCE)",
        },
        AntikytheraCycle {
            name: "Saros",
            period_years: "18.03",
            gear_teeth: 223,
            description: "223 synodic months — eclipse repeat cycle",
        },
        AntikytheraCycle {
            name: "Exeligmos",
            period_years: "54.09",
            gear_teeth: 669,
            description: "3 Saros cycles = 669 synodic months (exact eclipse repeat)",
        },
        AntikytheraCycle {
            name: "Callippic",
            period_years: "76",
            gear_teeth: 940,
            description: "4 Metonic cycles - 1 day = 940 synodic months (Callippus, 330 BCE)",
        },
        AntikytheraCycle {
            name: "Olympiad",
            period_years: "4",
            gear_teeth: 57,
            description: "4-year cycle of Panhellenic games (Olympic, Nemean, Isthmian, Pythian)",
        },
        AntikytheraCycle {
            name: "Synodic Month",
            period_years: "0.0809",
            gear_teeth: 127,
            description: "Lunar month: 29.53059 days — ratio 127/223 encodes month count",
        },
        AntikytheraCycle {
            name: "Sidereal Year",
            period_years: "1",
            gear_teeth: 365,
            description: "Solar year: 365.25 days — primary input gear",
        },
    ];

    let mut map = BTreeMap::new();
    for cycle in cycles {
        map.insert(cycle.name, cycle);
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn golden_ratio_value() {
        let expected = (1.0_f64 + 5.0_f64.sqrt()) / 2.0;
        assert!((PHI - expected).abs() < 1e-15);
    }

    #[test]
    fn fibonacci_ratio_converges_to_phi() {
        let ratio = fibonacci_ratio(30);
        assert!((ratio - PHI).abs() < 1e-10);
    }

    #[test]
    fn sieve_small() {
        let primes = sieve(30);
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn sieve_below_2() {
        assert!(sieve(0).is_empty());
        assert!(sieve(1).is_empty());
    }

    #[test]
    fn gcd_basic() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(7, 0), 7);
        assert_eq!(gcd(13, 13), 13);
    }

    #[test]
    fn lcm_basic() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(0, 5), 0);
    }

    #[test]
    fn archimedes_pi_bounds() {
        let (lower, upper) = archimedes_pi(10);
        assert!(lower < std::f64::consts::PI);
        assert!(upper > std::f64::consts::PI);
        // After 10 iterations (6 * 2^10 = 6144 sides), should be close
        assert!((lower - std::f64::consts::PI).abs() < 1e-6);
        assert!((upper - std::f64::consts::PI).abs() < 1e-6);
    }

    #[test]
    fn antikythera_metonic() {
        let ratios = antikythera_gear_ratios();
        let metonic = ratios.get("Metonic").unwrap();
        assert_eq!(metonic.gear_teeth, 235);
        assert_eq!(metonic.period_years, "19");
    }

    #[cfg(feature = "varna")]
    mod isopsephy_tests {
        use super::*;

        #[test]
        fn isopsephy_single_letter() {
            assert_eq!(isopsephy("α").unwrap(), 1);
            assert_eq!(isopsephy("ω").unwrap(), 800);
        }

        #[test]
        fn isopsephy_word() {
            // "πι" = 80 + 10 = 90
            assert_eq!(isopsephy("πι").unwrap(), 90);
            // "αω" = 1 + 800 = 801
            assert_eq!(isopsephy("αω").unwrap(), 801);
        }

        #[test]
        fn isopsephy_empty_errors() {
            assert!(isopsephy("").is_err());
        }

        #[test]
        fn isopsephy_non_greek_errors() {
            assert!(isopsephy("hello").is_err());
        }

        #[test]
        fn to_greek_numeral_basic() {
            // 1 = α
            assert_eq!(to_greek_numeral(1).unwrap(), "α");
            // 10 = ι
            assert_eq!(to_greek_numeral(10).unwrap(), "ι");
            // 100 = ρ
            assert_eq!(to_greek_numeral(100).unwrap(), "ρ");
        }

        #[test]
        fn to_greek_numeral_compound() {
            // 358 = τ(300) + ν(50) + η(8) = "τνη"
            assert_eq!(to_greek_numeral(358).unwrap(), "τνη");
        }

        #[test]
        fn to_greek_numeral_roundtrip() {
            // Encode then decode
            let numeral = to_greek_numeral(358).unwrap();
            let value = isopsephy(&numeral).unwrap();
            assert_eq!(value, 358);
        }

        #[test]
        fn to_greek_numeral_zero_errors() {
            assert!(to_greek_numeral(0).is_err());
        }
    }
}
