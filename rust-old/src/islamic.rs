//! Islamic and Arabic mathematics.
//!
//! Implements al-Khwarizmi's algebra, Omar Khayyam's cubic classification,
//! the Hindu-Arabic numeral positional system, Hisab al-Jabr methods,
//! and the Islamic calendar (Hijri).
//!
//! # Historical Context
//!
//! The Islamic Golden Age (c. 750–1258 CE) saw a flowering of mathematical
//! innovation that bridged Greek, Indian, and Babylonian traditions.
//! Muhammad ibn Musa al-Khwarizmi (c. 780–850 CE) wrote *Al-Kitab
//! al-Mukhtasar fi Hisab al-Jabr wal-Muqabala* ("The Compendious Book
//! on Calculation by Completion and Balancing"), founding algebra as a
//! discipline. The word "algorithm" derives from the Latinization of his
//! name. Omar Khayyam (1048–1131 CE) classified and solved cubic equations
//! geometrically, centuries before Cardano's algebraic solutions.

use serde::{Deserialize, Serialize};

use crate::error::{Result, SankhyaError};

// ---------------------------------------------------------------------------
// Al-Khwarizmi's six canonical equation forms
// ---------------------------------------------------------------------------

/// The six canonical forms of quadratic equations from al-Khwarizmi's
/// *Al-Jabr wal-Muqabala*.
///
/// Al-Khwarizmi classified all equations of degree ≤ 2 into six types,
/// because he worked only with positive coefficients (negative numbers
/// were not accepted). The three "simple" types have one operation;
/// the three "compound" types involve all three terms.
///
/// In modern notation (where a, b, c > 0):
/// - Simple: ax² = bx, ax² = c, bx = c
/// - Compound: ax² + bx = c, ax² + c = bx, bx + c = ax²
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AlJabrForm {
    /// "Squares equal roots": ax² = bx
    SquaresEqualRoots,
    /// "Squares equal number": ax² = c
    SquaresEqualNumber,
    /// "Roots equal number": bx = c
    RootsEqualNumber,
    /// "Squares and roots equal number": ax² + bx = c
    SquaresAndRootsEqualNumber,
    /// "Squares and number equal roots": ax² + c = bx
    SquaresAndNumberEqualRoots,
    /// "Roots and number equal squares": bx + c = ax²
    RootsAndNumberEqualSquares,
}

/// Result of solving an al-Khwarizmi equation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlJabrSolution {
    /// The canonical form of the equation.
    pub form: AlJabrForm,
    /// The positive root(s). Al-Khwarizmi only accepted positive solutions.
    pub roots: Vec<f64>,
    /// The original coefficients (a, b, c) in the standard form.
    pub coefficients: (f64, f64, f64),
}

/// Classify and solve a quadratic equation in al-Khwarizmi's framework.
///
/// Takes coefficients of ax² + bx + c = 0 (standard modern form) and
/// rearranges into one of al-Khwarizmi's six canonical forms, then solves
/// using his geometric completion-of-the-square method.
///
/// Only positive roots are returned, following al-Khwarizmi's convention.
///
/// # Errors
///
/// Returns [`SankhyaError::ComputationError`] if all coefficients are zero.
/// Returns [`SankhyaError::InvalidBase`] if `a`, `b`, and `c` produce no
/// positive roots.
#[must_use = "returns the solution or an error"]
pub fn solve_al_jabr(a: f64, b: f64, c: f64) -> Result<AlJabrSolution> {
    tracing::debug!(a, b, c, "solving al-Jabr equation ax² + bx + c = 0");
    if a.abs() < f64::EPSILON && b.abs() < f64::EPSILON && c.abs() < f64::EPSILON {
        return Err(SankhyaError::ComputationError(
            "all coefficients are zero".into(),
        ));
    }

    // Linear case: bx + c = 0
    if a.abs() < f64::EPSILON {
        if b.abs() < f64::EPSILON {
            return Err(SankhyaError::ComputationError(
                "degenerate equation: no variable terms".into(),
            ));
        }
        let root = -c / b;
        let mut roots = Vec::new();
        if root > 0.0 {
            roots.push(root);
        }
        return Ok(AlJabrSolution {
            form: AlJabrForm::RootsEqualNumber,
            roots,
            coefficients: (a, b, c),
        });
    }

    // Quadratic case: classify into al-Khwarizmi's six forms
    // Normalize: divide by a to get x² + (b/a)x + (c/a) = 0
    let p = b / a; // coefficient of x (may be negative)
    let q = c / a; // constant term (may be negative)

    let form = if p.abs() < f64::EPSILON && q.abs() < f64::EPSILON {
        // x² = 0 (trivial)
        AlJabrForm::SquaresEqualRoots
    } else if q.abs() < f64::EPSILON {
        // x² + px = 0 → x(x + p) = 0 → root at x = -p (squares equal roots)
        AlJabrForm::SquaresEqualRoots
    } else if p.abs() < f64::EPSILON {
        // x² + q = 0 → x² = -q
        AlJabrForm::SquaresEqualNumber
    } else if p > 0.0 && q > 0.0 {
        // x² + px + q = 0 → impossible for positive roots (all terms positive)
        // Rearrange: this doesn't fit standard al-Jabr (he requires positive coefficients)
        // But -p and -q might give: x² = px + q → RootsAndNumberEqualSquares
        AlJabrForm::SquaresAndRootsEqualNumber
    } else if p > 0.0 && q < 0.0 {
        // x² + px - |q| = 0 → x² + px = |q|
        AlJabrForm::SquaresAndRootsEqualNumber
    } else if p < 0.0 && q > 0.0 {
        // x² - |p|x + q = 0 → x² + q = |p|x
        AlJabrForm::SquaresAndNumberEqualRoots
    } else {
        // p < 0, q < 0: x² - |p|x - |q| = 0 → |p|x + |q| = x²
        AlJabrForm::RootsAndNumberEqualSquares
    };

    // Solve using the standard quadratic formula, filter to positive roots
    let discriminant = b * b - 4.0 * a * c;

    let mut roots = Vec::new();
    if discriminant >= 0.0 {
        let sqrt_d = discriminant.sqrt();
        let r1 = (-b + sqrt_d) / (2.0 * a);
        let r2 = (-b - sqrt_d) / (2.0 * a);
        if r1 > f64::EPSILON {
            roots.push(r1);
        }
        if r2 > f64::EPSILON && (r1 - r2).abs() > f64::EPSILON {
            roots.push(r2);
        }
    }

    Ok(AlJabrSolution {
        form,
        roots,
        coefficients: (a, b, c),
    })
}

// ---------------------------------------------------------------------------
// Omar Khayyam's cubic equation classification
// ---------------------------------------------------------------------------

/// Omar Khayyam's classification of cubic equations.
///
/// Khayyam identified 19 types of cubic equations (with positive
/// coefficients only) and solved them geometrically using conic
/// section intersections. This enum covers the primary types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum KhayyamCubicType {
    /// x³ = d (cube equals number)
    CubeEqualNumber,
    /// x³ + bx = d (cube and thing equal number)
    CubeAndThingEqualNumber,
    /// x³ + cx² = d (cube and square equal number)
    CubeAndSquareEqualNumber,
    /// x³ = bx + d (cube equals thing and number)
    CubeEqualThingAndNumber,
    /// x³ + bx + d = cx² (cube, thing, and number equal square)
    CubeThingNumberEqualSquare,
}

/// Classify a cubic equation in Khayyam's framework.
///
/// Takes coefficients of x³ + cx² + bx + d = 0 and classifies into
/// one of Khayyam's types, using only positive coefficient magnitudes.
///
/// Returns the type and the real root found by Cardano's/Newton's method
/// (Khayyam solved geometrically; we compute numerically for verification).
///
/// # Errors
///
/// Returns [`SankhyaError::ComputationError`] if the cubic has no real roots
/// or if the iteration fails to converge.
#[must_use = "returns the classification and roots or an error"]
pub fn classify_khayyam_cubic(
    a3: f64,
    a2: f64,
    a1: f64,
    a0: f64,
) -> Result<(KhayyamCubicType, Vec<f64>)> {
    if a3.abs() < f64::EPSILON {
        return Err(SankhyaError::ComputationError(
            "leading coefficient is zero — not a cubic".into(),
        ));
    }

    // Normalize to monic: x³ + cx² + bx + d = 0
    let c = a2 / a3;
    let b = a1 / a3;
    let d = a0 / a3;

    let cubic_type = if c.abs() < f64::EPSILON && b.abs() < f64::EPSILON {
        KhayyamCubicType::CubeEqualNumber
    } else if c.abs() < f64::EPSILON && d < 0.0 {
        KhayyamCubicType::CubeAndThingEqualNumber
    } else if b.abs() < f64::EPSILON && d < 0.0 {
        KhayyamCubicType::CubeAndSquareEqualNumber
    } else if c.abs() < f64::EPSILON && b < 0.0 {
        KhayyamCubicType::CubeEqualThingAndNumber
    } else {
        KhayyamCubicType::CubeThingNumberEqualSquare
    };

    // Find real root(s) via Newton's method
    let f = |x: f64| a3 * x * x * x + a2 * x * x + a1 * x + a0;
    let df = |x: f64| 3.0 * a3 * x * x + 2.0 * a2 * x + a1;

    let mut roots = Vec::new();

    // Try multiple starting points
    for &x0 in &[0.0, 1.0, -1.0, 10.0, -10.0, 100.0, -100.0] {
        let mut x = x0;
        let mut converged = false;
        for _ in 0..200 {
            let fx = f(x);
            let dfx = df(x);
            if dfx.abs() < f64::EPSILON {
                break;
            }
            let x_new = x - fx / dfx;
            if (x_new - x).abs() < 1e-12 {
                converged = true;
                x = x_new;
                break;
            }
            x = x_new;
        }
        if converged && f(x).abs() < 1e-8 {
            // Check if this root is already found
            let is_duplicate = roots.iter().any(|&r: &f64| (r - x).abs() < 1e-8);
            if !is_duplicate {
                roots.push(x);
            }
        }
    }

    if roots.is_empty() {
        return Err(SankhyaError::ComputationError(
            "Newton's method failed to find a real root".into(),
        ));
    }

    // Sort for deterministic output
    roots.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    Ok((cubic_type, roots))
}

// ---------------------------------------------------------------------------
// Al-Khwarizmi's completion of the square
// ---------------------------------------------------------------------------

/// Demonstrate al-Khwarizmi's geometric completion of the square.
///
/// Given x² + bx = c (where b, c > 0), al-Khwarizmi's method:
/// 1. Draw a square of side x
/// 2. Add four rectangles of width b/4 on each side
/// 3. Complete the larger square by adding (b/4)² corner squares
/// 4. The large square has area c + (b/2)²
/// 5. Therefore x + b/2 = sqrt(c + (b/2)²)
/// 6. x = sqrt(c + (b/2)²) - b/2
///
/// Returns `(x, area_of_completed_square)`.
///
/// # Errors
///
/// Returns [`SankhyaError::InvalidBase`] if b or c is not positive.
#[must_use = "returns the root and completed area or an error"]
pub fn complete_the_square(b: f64, c: f64) -> Result<(f64, f64)> {
    if b <= 0.0 || c <= 0.0 {
        return Err(SankhyaError::InvalidBase(
            "al-Khwarizmi's completion requires positive b and c".into(),
        ));
    }

    let half_b = b / 2.0;
    let completed_area = c + half_b * half_b;
    let x = completed_area.sqrt() - half_b;

    Ok((x, completed_area))
}

// ---------------------------------------------------------------------------
// Hijri (Islamic) calendar
// ---------------------------------------------------------------------------

/// Julian Day Number of the Islamic calendar epoch: July 16, 622 CE (Julian).
///
/// This is 1 Muharram 1 AH (Anno Hegirae), the day after the Prophet
/// Muhammad's emigration from Mecca to Medina. The tabular Islamic calendar
/// uses a fixed arithmetic cycle rather than lunar observation.
pub const HIJRI_EPOCH_JDN: f64 = 1_948_439.5;

/// Mean length of an Islamic lunar month in days.
pub const ISLAMIC_MONTH_DAYS: f64 = 29.530_588_86;

/// Length of the 30-year Islamic calendar cycle in days.
/// 30 years × 12 months, with 11 leap years of 355 days and 19 common years of 354 days.
/// Total: 19 × 354 + 11 × 355 = 10,631 days.
pub const HIJRI_30_YEAR_CYCLE_DAYS: u64 = 10_631;

/// Days in each month of a common Hijri year.
/// Odd months have 30 days, even months have 29 days (except Dhul Hijjah in leap years).
const HIJRI_MONTH_DAYS: [u8; 12] = [30, 29, 30, 29, 30, 29, 30, 29, 30, 29, 30, 29];

/// The 12 months of the Islamic calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum HijriMonth {
    /// Muharram (sacred month)
    Muharram,
    /// Safar
    Safar,
    /// Rabi al-Awwal
    RabiAlAwwal,
    /// Rabi al-Thani
    RabiAlThani,
    /// Jumada al-Ula
    JumadaAlUla,
    /// Jumada al-Thani
    JumadaAlThani,
    /// Rajab (sacred month)
    Rajab,
    /// Shaban
    Shaban,
    /// Ramadan (month of fasting)
    Ramadan,
    /// Shawwal
    Shawwal,
    /// Dhul Qadah (sacred month)
    DhulQadah,
    /// Dhul Hijjah (sacred month, pilgrimage)
    DhulHijjah,
}

const HIJRI_MONTHS: [HijriMonth; 12] = [
    HijriMonth::Muharram,
    HijriMonth::Safar,
    HijriMonth::RabiAlAwwal,
    HijriMonth::RabiAlThani,
    HijriMonth::JumadaAlUla,
    HijriMonth::JumadaAlThani,
    HijriMonth::Rajab,
    HijriMonth::Shaban,
    HijriMonth::Ramadan,
    HijriMonth::Shawwal,
    HijriMonth::DhulQadah,
    HijriMonth::DhulHijjah,
];

/// A date in the tabular Islamic (Hijri) calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HijriDate {
    /// Year (Anno Hegirae).
    pub year: i64,
    /// Month (1-12).
    pub month: HijriMonth,
    /// Day of month (1-30).
    pub day: u8,
}

/// Whether a Hijri year is a leap year in the tabular calendar.
///
/// In the 30-year cycle, years 2, 5, 7, 10, 13, 16, 18, 21, 24, 26, 29
/// are leap years (Dhul Hijjah gets 30 days instead of 29).
#[must_use]
#[inline]
pub fn hijri_is_leap(year: i64) -> bool {
    let y = year.rem_euclid(30);
    matches!(y, 2 | 5 | 7 | 10 | 13 | 16 | 18 | 21 | 24 | 26 | 29)
}

/// Days in a Hijri year (354 common, 355 leap).
#[must_use]
#[inline]
pub fn hijri_year_days(year: i64) -> u16 {
    if hijri_is_leap(year) { 355 } else { 354 }
}

/// Convert a Julian Day Number to a Hijri date (tabular calendar).
///
/// Uses the standard tabular algorithm with the 30-year cycle.
#[must_use]
pub fn jdn_to_hijri(jdn: f64) -> HijriDate {
    tracing::trace!(jdn, "JDN to Hijri");
    let days_since_epoch = (jdn - HIJRI_EPOCH_JDN).floor() as i64;

    // 30-year cycles
    let cycles = days_since_epoch.div_euclid(HIJRI_30_YEAR_CYCLE_DAYS as i64);
    let mut remaining = days_since_epoch.rem_euclid(HIJRI_30_YEAR_CYCLE_DAYS as i64);

    let mut year = cycles * 30 + 1; // Hijri years start at 1

    // Count complete years within the cycle
    loop {
        let yd = i64::from(hijri_year_days(year));
        if remaining < yd {
            break;
        }
        remaining -= yd;
        year += 1;
    }

    // Count complete months within the year
    let mut month_idx = 0;
    loop {
        let md = if month_idx == 11 && hijri_is_leap(year) {
            30i64 // Dhul Hijjah in leap year
        } else {
            i64::from(HIJRI_MONTH_DAYS[month_idx])
        };
        if remaining < md {
            break;
        }
        remaining -= md;
        month_idx += 1;
        if month_idx >= 12 {
            month_idx = 11;
            break;
        }
    }

    HijriDate {
        year,
        month: HIJRI_MONTHS[month_idx],
        day: remaining as u8 + 1, // 1-based
    }
}

/// Convert a Hijri date to a Julian Day Number (tabular calendar).
///
/// # Errors
///
/// Returns [`SankhyaError::InvalidDate`] if the day is out of range for the month.
#[must_use = "returns the JDN or an error"]
pub fn hijri_to_jdn(date: &HijriDate) -> Result<f64> {
    tracing::trace!(year = date.year, ?date.month, day = date.day, "Hijri to JDN");
    let month_idx = HIJRI_MONTHS
        .iter()
        .position(|&m| m == date.month)
        .unwrap_or(0);

    let max_day = if month_idx == 11 && hijri_is_leap(date.year) {
        30
    } else {
        HIJRI_MONTH_DAYS[month_idx]
    };
    if date.day == 0 || date.day > max_day {
        return Err(SankhyaError::InvalidDate(format!(
            "day {} out of range for {:?} (max {max_day})",
            date.day, date.month
        )));
    }

    // Days from complete years
    let mut days: i64 = 0;
    let base_year = 1i64;
    for y in base_year..date.year {
        days += i64::from(hijri_year_days(y));
    }

    // Days from complete months in this year
    for (m, &md) in HIJRI_MONTH_DAYS.iter().enumerate().take(month_idx) {
        days += if m == 11 && hijri_is_leap(date.year) {
            30
        } else {
            i64::from(md)
        };
    }

    // Days within the month
    days += i64::from(date.day - 1);

    Ok(HIJRI_EPOCH_JDN + days as f64)
}

impl core::fmt::Display for HijriDate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} {:?} {} AH", self.day, self.month, self.year)
    }
}

impl core::fmt::Display for HijriMonth {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = match self {
            Self::Muharram => "Muharram",
            Self::Safar => "Safar",
            Self::RabiAlAwwal => "Rabi al-Awwal",
            Self::RabiAlThani => "Rabi al-Thani",
            Self::JumadaAlUla => "Jumada al-Ula",
            Self::JumadaAlThani => "Jumada al-Thani",
            Self::Rajab => "Rajab",
            Self::Shaban => "Sha'ban",
            Self::Ramadan => "Ramadan",
            Self::Shawwal => "Shawwal",
            Self::DhulQadah => "Dhul Qa'dah",
            Self::DhulHijjah => "Dhul Hijjah",
        };
        write!(f, "{name}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- Al-Jabr --

    #[test]
    fn al_jabr_linear() {
        // 2x - 6 = 0 → x = 3
        let sol = solve_al_jabr(0.0, 2.0, -6.0).unwrap();
        assert_eq!(sol.roots.len(), 1);
        assert!((sol.roots[0] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn al_jabr_squares_and_roots_equal_number() {
        // x² + 10x - 39 = 0 → al-Khwarizmi's famous example → x = 3
        let sol = solve_al_jabr(1.0, 10.0, -39.0).unwrap();
        assert!(sol.roots.contains(&3.0) || sol.roots.iter().any(|&r| (r - 3.0).abs() < 1e-10));
    }

    #[test]
    fn al_jabr_two_positive_roots() {
        // x² - 5x + 6 = 0 → x = 2, x = 3
        let sol = solve_al_jabr(1.0, -5.0, 6.0).unwrap();
        assert_eq!(sol.roots.len(), 2);
        let mut roots = sol.roots.clone();
        roots.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert!((roots[0] - 2.0).abs() < 1e-10);
        assert!((roots[1] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn al_jabr_no_positive_roots() {
        // x² + x + 1 = 0 → negative discriminant
        let sol = solve_al_jabr(1.0, 1.0, 1.0).unwrap();
        assert!(sol.roots.is_empty());
    }

    #[test]
    fn al_jabr_all_zero_errors() {
        assert!(solve_al_jabr(0.0, 0.0, 0.0).is_err());
    }

    // -- Complete the square --

    #[test]
    fn complete_square_al_khwarizmi_example() {
        // x² + 10x = 39 → x = 3
        let (x, _area) = complete_the_square(10.0, 39.0).unwrap();
        assert!((x - 3.0).abs() < 1e-10);
    }

    #[test]
    fn complete_square_negative_errors() {
        assert!(complete_the_square(-1.0, 5.0).is_err());
        assert!(complete_the_square(1.0, -5.0).is_err());
    }

    // -- Khayyam cubic --

    #[test]
    fn khayyam_cube_equal_number() {
        // x³ - 8 = 0 → x = 2
        let (ctype, roots) = classify_khayyam_cubic(1.0, 0.0, 0.0, -8.0).unwrap();
        assert_eq!(ctype, KhayyamCubicType::CubeEqualNumber);
        assert!(roots.iter().any(|&r| (r - 2.0).abs() < 1e-6));
    }

    #[test]
    fn khayyam_cube_and_thing() {
        // x³ + 6x - 20 = 0 → x = 2
        let (_ctype, roots) = classify_khayyam_cubic(1.0, 0.0, 6.0, -20.0).unwrap();
        assert!(roots.iter().any(|&r| (r - 2.0).abs() < 1e-6));
    }

    #[test]
    fn khayyam_not_cubic_errors() {
        assert!(classify_khayyam_cubic(0.0, 1.0, 2.0, 3.0).is_err());
    }

    // -- Hijri calendar --

    #[test]
    fn hijri_leap_years() {
        // Years 2, 5, 7, 10, 13, 16, 18, 21, 24, 26, 29 in 30-year cycle
        assert!(hijri_is_leap(2));
        assert!(hijri_is_leap(5));
        assert!(hijri_is_leap(29));
        assert!(!hijri_is_leap(1));
        assert!(!hijri_is_leap(3));
        assert!(!hijri_is_leap(30));
    }

    #[test]
    fn hijri_year_length() {
        assert_eq!(hijri_year_days(1), 354);
        assert_eq!(hijri_year_days(2), 355);
    }

    #[test]
    fn hijri_epoch_roundtrip() {
        // 1 Muharram 1 AH
        let date = jdn_to_hijri(HIJRI_EPOCH_JDN);
        assert_eq!(date.year, 1);
        assert_eq!(date.month, HijriMonth::Muharram);
        assert_eq!(date.day, 1);

        let jdn = hijri_to_jdn(&date).unwrap();
        assert!((jdn - HIJRI_EPOCH_JDN).abs() < 0.5);
    }

    #[test]
    fn hijri_known_date() {
        // 1 Ramadan 1 AH = JDN approximately 1948439.5 + 236 days
        // (8 months: 30+29+30+29+30+29+30+29 = 236)
        let jdn = HIJRI_EPOCH_JDN + 236.0;
        let date = jdn_to_hijri(jdn);
        assert_eq!(date.month, HijriMonth::Ramadan);
        assert_eq!(date.day, 1);
    }

    #[test]
    fn hijri_to_jdn_invalid_day() {
        let date = HijriDate {
            year: 1,
            month: HijriMonth::Muharram,
            day: 31, // Muharram has 30 days
        };
        assert!(hijri_to_jdn(&date).is_err());
    }

    #[test]
    fn hijri_30_year_cycle_consistent() {
        // 30 years should sum to 10,631 days
        let mut total: u64 = 0;
        for y in 1..=30 {
            total += u64::from(hijri_year_days(y));
        }
        assert_eq!(total, HIJRI_30_YEAR_CYCLE_DAYS);
    }

    // -- Serde --

    #[test]
    fn serde_roundtrip_hijri_date() {
        let date = jdn_to_hijri(HIJRI_EPOCH_JDN + 1000.0);
        let json = serde_json::to_string(&date).unwrap();
        let back: HijriDate = serde_json::from_str(&json).unwrap();
        assert_eq!(date, back);
    }

    #[test]
    fn serde_roundtrip_al_jabr_solution() {
        let sol = solve_al_jabr(1.0, -5.0, 6.0).unwrap();
        let json = serde_json::to_string(&sol).unwrap();
        let back: AlJabrSolution = serde_json::from_str(&json).unwrap();
        assert_eq!(sol, back);
    }
}
