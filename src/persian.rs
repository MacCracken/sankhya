//! Persian (Solar Hijri / Jalaali) calendar.
//!
//! Implements the Solar Hijri calendar with Julian Day Number conversions,
//! using the arithmetic Jalaali algorithm. Based on the algorithms in
//! Dershowitz & Reingold, *Calendrical Calculations* (4th ed., Cambridge
//! University Press, 2018), ch. 15, and the Jalaali algorithm by
//! Ahmad Birashk, *A Comparative Calendar of the Iranian, Muslim Lunar,
//! and Christian Eras for Three Thousand Years* (Mazda Publishers, 1993).
//!
//! # Historical Context
//!
//! The Solar Hijri calendar (also called the Iranian or Jalaali calendar)
//! is a solar calendar used officially in Iran and Afghanistan. It was
//! reformed by Omar Khayyam and other astronomers in 1079 CE under the
//! Seljuk Sultan Malik-Shah I. The modern arithmetic version uses a
//! 2820-year grand cycle with precisely distributed leap years, achieving
//! an average year length of 365.24219858... days — more accurate than
//! the Gregorian calendar.
//!
//! The epoch is the vernal equinox of 622 CE (the Hijra), making it a
//! solar counterpart to the lunar Islamic Hijri calendar.

use serde::{Deserialize, Serialize};

use crate::error::{Result, SankhyaError};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Julian Day Number of the Persian (Solar Hijri) epoch: 1 Farvardin,
/// Year 1 AP = March 22, 622 CE Julian (vernal equinox).
pub const PERSIAN_EPOCH_JDN: f64 = 1_948_320.5;

/// Days in each month of a common Persian year.
/// Months 1–6 have 31 days, months 7–11 have 30 days, month 12 has 29 days
/// (30 in leap years).
const PERSIAN_MONTH_DAYS: [u8; 12] = [31, 31, 31, 31, 31, 31, 30, 30, 30, 30, 30, 29];

// ---------------------------------------------------------------------------
// Month enum
// ---------------------------------------------------------------------------

/// The 12 months of the Persian (Solar Hijri) calendar.
///
/// Months 1–6 (Farvardin–Shahrivar) have 31 days each.
/// Months 7–11 (Mehr–Bahman) have 30 days each.
/// Month 12 (Esfand) has 29 days (30 in leap years).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PersianMonth {
    /// Farvardin (month 1, 31 days) — spring equinox
    Farvardin,
    /// Ordibehesht (month 2, 31 days)
    Ordibehesht,
    /// Khordad (month 3, 31 days)
    Khordad,
    /// Tir (month 4, 31 days)
    Tir,
    /// Mordad (month 5, 31 days)
    Mordad,
    /// Shahrivar (month 6, 31 days)
    Shahrivar,
    /// Mehr (month 7, 30 days)
    Mehr,
    /// Aban (month 8, 30 days)
    Aban,
    /// Azar (month 9, 30 days)
    Azar,
    /// Dey (month 10, 30 days)
    Dey,
    /// Bahman (month 11, 30 days)
    Bahman,
    /// Esfand (month 12, 29 or 30 days)
    Esfand,
}

const PERSIAN_MONTHS: [PersianMonth; 12] = [
    PersianMonth::Farvardin,
    PersianMonth::Ordibehesht,
    PersianMonth::Khordad,
    PersianMonth::Tir,
    PersianMonth::Mordad,
    PersianMonth::Shahrivar,
    PersianMonth::Mehr,
    PersianMonth::Aban,
    PersianMonth::Azar,
    PersianMonth::Dey,
    PersianMonth::Bahman,
    PersianMonth::Esfand,
];

impl core::fmt::Display for PersianMonth {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = match self {
            Self::Farvardin => "Farvardin",
            Self::Ordibehesht => "Ordibehesht",
            Self::Khordad => "Khordad",
            Self::Tir => "Tir",
            Self::Mordad => "Mordad",
            Self::Shahrivar => "Shahrivar",
            Self::Mehr => "Mehr",
            Self::Aban => "Aban",
            Self::Azar => "Azar",
            Self::Dey => "Dey",
            Self::Bahman => "Bahman",
            Self::Esfand => "Esfand",
        };
        write!(f, "{name}")
    }
}

// ---------------------------------------------------------------------------
// Date struct
// ---------------------------------------------------------------------------

/// A date in the Persian (Solar Hijri / Jalaali) calendar.
///
/// Year 1 AP (Anno Persico) corresponds to 622 CE, the year of the Hijra.
/// The year begins at the vernal equinox (Nowruz).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersianDate {
    /// Year (Anno Persico). Year 1 = 622 CE.
    pub year: i64,
    /// Month (one of the 12 Persian months).
    pub month: PersianMonth,
    /// Day of month (1–31 for months 1–6, 1–30 for months 7–11, 1–29/30 for Esfand).
    pub day: u8,
}

impl core::fmt::Display for PersianDate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} {} {} AP", self.day, self.month, self.year)
    }
}

// ---------------------------------------------------------------------------
// Leap year (Jalaali algorithm)
// ---------------------------------------------------------------------------

/// Whether a Persian year is a leap year in the Jalaali (arithmetic) calendar.
///
/// Uses the breaks-array algorithm from jalaali-js, the de facto standard
/// implementation. The 2820-year grand cycle is composed of irregular
/// sub-cycles. Within each sub-cycle, leap years are spaced at 4-year
/// intervals with one 5-year gap.
///
/// Reference: github.com/jalaali/jalaali-js (authoritative implementation).
#[must_use]
pub fn persian_is_leap(year: i64) -> bool {
    jalaali_cal(year).0 == 0
}

/// Core Jalaali calendar calculation, direct port of jalaali-js `jalCal`.
///
/// Returns `(leap, leap_j_count, march_day_gregorian)`.
/// `leap` is 0 for leap years, 1/2/3 for the years following a leap year.
fn jalaali_cal(jy: i64) -> (i64, i64, i64) {
    const BREAKS: [i64; 20] = [
        -61, 9, 38, 199, 426, 686, 756, 818, 1111, 1181, 1210, 1635, 2060, 2097, 2192, 2262, 2324,
        2394, 2456, 2526,
    ];

    let gy = jy + 621;
    let mut leap_j = -14i64;
    let mut jp = BREAKS[0];
    let mut jump = 0i64;

    for &jm in &BREAKS[1..] {
        jump = jm - jp;
        if jy < jm {
            break;
        }
        leap_j += (jump / 33) * 8 + (jump % 33) / 4;
        jp = jm;
    }

    let mut n = jy - jp;

    // Leap year determination: adjust N for the 5-year gap at sub-cycle end
    if jump - n < 6 {
        n = n - jump + (jump + 4) / 33 * 33;
    }

    let leap = ((n + 1) % 33 - 1) % 4;
    // Handle the case where modular arithmetic gives -1
    let leap = if leap < 0 { leap + 4 } else { leap };

    // Recompute n for leap count (use original n)
    let n_orig = jy - jp;
    leap_j += (n_orig / 33) * 8 + (n_orig % 33 + 3) / 4;

    if jump % 33 == 4 && jump - n_orig == 4 {
        leap_j += 1;
    }

    // March day of Nowruz (Gregorian)
    let leap_g = gy / 4 - ((gy / 100 + 1) * 3) / 4 - 150;
    let march = 20 + leap_j - leap_g;

    (leap, leap_j, march)
}

/// Days in a Persian year (365 common, 366 leap).
#[must_use]
#[inline]
pub fn persian_year_days(year: i64) -> u16 {
    if persian_is_leap(year) { 366 } else { 365 }
}

// ---------------------------------------------------------------------------
// JDN conversions
// ---------------------------------------------------------------------------

/// Convert a Julian Day Number to a Persian (Solar Hijri) date.
///
/// Uses the jalaali-js algorithm: converts JDN to Gregorian, then uses
/// the `march` day of Nowruz to determine the Jalaali year and day-of-year.
#[must_use]
pub fn jdn_to_persian(jdn: f64) -> PersianDate {
    let gy = crate::gregorian::jdn_to_gregorian(jdn);
    let gy_year = gy.year;

    // Estimate Jalaali year
    let mut jy = gy_year - 621;
    let (_, _, march) = jalaali_cal(jy);

    // JDN of 1 Farvardin of the estimated year
    let nowruz_jdn = gregorian_march_jdn(gy_year, march);

    let day_diff = (jdn - nowruz_jdn).floor() as i64;

    if day_diff < 0 {
        // Before Nowruz of this year — go back one Jalaali year
        jy -= 1;
        let (_, _, march_prev) = jalaali_cal(jy);
        let nowruz_prev = gregorian_march_jdn(gy_year - 1, march_prev);
        let doy = (jdn - nowruz_prev).floor() as i64;
        let (month_idx, day) = month_day_from_doy(doy, persian_is_leap(jy));
        return PersianDate {
            year: jy,
            month: PERSIAN_MONTHS[month_idx],
            day,
        };
    }

    let yd = persian_year_days(jy) as i64;
    if day_diff >= yd {
        // Past the end of this Jalaali year
        let doy = day_diff - yd;
        jy += 1;
        let (month_idx, day) = month_day_from_doy(doy, persian_is_leap(jy));
        return PersianDate {
            year: jy,
            month: PERSIAN_MONTHS[month_idx],
            day,
        };
    }

    let (month_idx, day) = month_day_from_doy(day_diff, persian_is_leap(jy));
    PersianDate {
        year: jy,
        month: PERSIAN_MONTHS[month_idx],
        day,
    }
}

/// Convert a Persian (Solar Hijri) date to a Julian Day Number.
///
/// Uses the jalaali-js algorithm: computes the Gregorian March day of
/// Nowruz, then adds the day-of-year offset.
///
/// # Errors
///
/// Returns [`SankhyaError::InvalidDate`] if the day is out of range for the
/// given month and year.
#[must_use = "returns the JDN or an error"]
pub fn persian_to_jdn(date: &PersianDate) -> Result<f64> {
    let month_idx = PERSIAN_MONTHS
        .iter()
        .position(|&m| m == date.month)
        .unwrap_or(0);

    let max_day = if month_idx == 11 && persian_is_leap(date.year) {
        30
    } else {
        PERSIAN_MONTH_DAYS[month_idx]
    };
    if date.day == 0 || date.day > max_day {
        return Err(SankhyaError::InvalidDate(format!(
            "day {} out of range for {} in year {} AP (max {max_day})",
            date.day, date.month, date.year
        )));
    }

    let (_, _, march) = jalaali_cal(date.year);
    let gy = date.year + 621;

    // JDN of 1 Farvardin = March `march` of Gregorian year `gy`
    let nowruz_jdn = gregorian_march_jdn(gy, march);

    // Day-of-year offset
    let mut day_of_year = i64::from(date.day) - 1;
    for &md in &PERSIAN_MONTH_DAYS[..month_idx] {
        day_of_year += i64::from(md);
    }

    Ok(nowruz_jdn + day_of_year as f64)
}

/// Get the JDN of a specific March day in a Gregorian year.
fn gregorian_march_jdn(gy: i64, march_day: i64) -> f64 {
    // Use the Gregorian module's conversion
    match crate::gregorian::gregorian_to_jdn(&crate::gregorian::GregorianDate {
        year: gy,
        month: crate::gregorian::GregorianMonth::March,
        day: march_day as u8,
    }) {
        Ok(jdn) => jdn,
        // Fallback: approximate (should never happen for valid march_day 19-22)
        Err(_) => crate::gregorian::GREGORIAN_EPOCH_JDN + (gy as f64 * 365.25) + 78.0,
    }
}

/// Convert a day-of-year (0-based) to (month_index, day) for the Persian calendar.
fn month_day_from_doy(mut doy: i64, is_leap: bool) -> (usize, u8) {
    for (i, &md) in PERSIAN_MONTH_DAYS.iter().enumerate() {
        let md_i64 = if i == 11 && is_leap {
            30
        } else {
            i64::from(md)
        };
        if doy < md_i64 {
            return (i, doy as u8 + 1);
        }
        doy -= md_i64;
    }
    // Fallback (shouldn't reach here for valid input)
    (11, doy as u8 + 1)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epoch_roundtrip() {
        let date = jdn_to_persian(PERSIAN_EPOCH_JDN);
        assert_eq!(date.year, 1);
        assert_eq!(date.month, PersianMonth::Farvardin);
        assert_eq!(date.day, 1);

        let jdn = persian_to_jdn(&date).unwrap();
        assert!((jdn - PERSIAN_EPOCH_JDN).abs() < f64::EPSILON);
    }

    #[test]
    fn known_date_nowruz_1404() {
        // 1 Farvardin 1404 AP = March 21, 2025 Gregorian = JDN 2460755.5
        // (Arithmetic Jalaali; observational calendar may differ by ±1 day)
        let date = jdn_to_persian(2_460_755.5);
        assert_eq!(date.year, 1404);
        assert_eq!(date.month, PersianMonth::Farvardin);
        assert_eq!(date.day, 1);
    }

    #[test]
    fn known_date_nowruz_1400() {
        // 1 Farvardin 1400 AP = March 21, 2021 Gregorian = JDN 2459294.5
        let date = jdn_to_persian(2_459_294.5);
        assert_eq!(date.year, 1400);
        assert_eq!(date.month, PersianMonth::Farvardin);
        assert_eq!(date.day, 1);
    }

    #[test]
    fn leap_year_known() {
        // Known leap years: 1399, 1403, 1408
        assert!(persian_is_leap(1399));
        assert!(persian_is_leap(1403));
        assert!(persian_is_leap(1408));
        // Known common years: 1400, 1401, 1402
        assert!(!persian_is_leap(1400));
        assert!(!persian_is_leap(1401));
        assert!(!persian_is_leap(1402));
    }

    #[test]
    fn year_days_values() {
        assert_eq!(persian_year_days(1399), 366);
        assert_eq!(persian_year_days(1400), 365);
    }

    #[test]
    fn esfand_leap_30_days() {
        let date = PersianDate {
            year: 1399,
            month: PersianMonth::Esfand,
            day: 30,
        };
        assert!(persian_to_jdn(&date).is_ok());
    }

    #[test]
    fn esfand_common_rejects_30() {
        let date = PersianDate {
            year: 1400,
            month: PersianMonth::Esfand,
            day: 30,
        };
        assert!(persian_to_jdn(&date).is_err());
    }

    #[test]
    fn invalid_day_zero() {
        let date = PersianDate {
            year: 1400,
            month: PersianMonth::Farvardin,
            day: 0,
        };
        assert!(persian_to_jdn(&date).is_err());
    }

    #[test]
    fn roundtrip_sequential_days() {
        // 1500 consecutive days from Nowruz 1400
        let start = 2_459_294.5; // 1 Farvardin 1400
        for offset in 0..1500 {
            let jdn = start + f64::from(offset);
            let date = jdn_to_persian(jdn);
            let back = persian_to_jdn(&date).unwrap();
            assert!(
                (back - jdn).abs() < f64::EPSILON,
                "roundtrip failed for JDN {jdn}: {date}"
            );
        }
    }

    #[test]
    fn roundtrip_across_leap_boundary() {
        // Test around a leap year boundary (1399 is leap, 1400 is not)
        // Last day of 1399 and first day of 1400
        let last_1399 = persian_to_jdn(&PersianDate {
            year: 1399,
            month: PersianMonth::Esfand,
            day: 30,
        })
        .unwrap();
        let first_1400 = persian_to_jdn(&PersianDate {
            year: 1400,
            month: PersianMonth::Farvardin,
            day: 1,
        })
        .unwrap();
        assert!((first_1400 - last_1399 - 1.0).abs() < f64::EPSILON);

        // Roundtrip both
        let d1 = jdn_to_persian(last_1399);
        assert_eq!(d1.year, 1399);
        assert_eq!(d1.month, PersianMonth::Esfand);
        assert_eq!(d1.day, 30);

        let d2 = jdn_to_persian(first_1400);
        assert_eq!(d2.year, 1400);
        assert_eq!(d2.month, PersianMonth::Farvardin);
        assert_eq!(d2.day, 1);
    }

    #[test]
    fn display_format() {
        let date = PersianDate {
            year: 1404,
            month: PersianMonth::Farvardin,
            day: 1,
        };
        assert_eq!(date.to_string(), "1 Farvardin 1404 AP");
    }

    #[test]
    fn serde_roundtrip() {
        let date = jdn_to_persian(2_459_294.5);
        let json = serde_json::to_string(&date).unwrap();
        let back: PersianDate = serde_json::from_str(&json).unwrap();
        assert_eq!(date, back);
    }

    #[test]
    fn month_display() {
        assert_eq!(PersianMonth::Farvardin.to_string(), "Farvardin");
        assert_eq!(PersianMonth::Esfand.to_string(), "Esfand");
        assert_eq!(PersianMonth::Mehr.to_string(), "Mehr");
    }
}
