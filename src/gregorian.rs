//! Gregorian calendar.
//!
//! Implements the proleptic Gregorian calendar with Julian Day Number
//! conversions, following the algorithms in Dershowitz & Reingold,
//! *Calendrical Calculations* (4th ed., Cambridge University Press, 2018).
//!
//! # Historical Context
//!
//! The Gregorian calendar was introduced by Pope Gregory XIII in October 1582
//! to correct the Julian calendar's accumulated drift (~10 days by the 16th
//! century). It refined the leap year rule: divisible by 4, except centuries
//! not divisible by 400. This keeps the calendar synchronized with the vernal
//! equinox to within 1 day per 3,236 years.
//!
//! This implementation uses the proleptic Gregorian calendar, extending the
//! Gregorian rules backward before 1582 for computational uniformity. Year 0
//! corresponds to 1 BCE in historical reckoning (astronomical year numbering).

use serde::{Deserialize, Serialize};

use crate::error::{Result, SankhyaError};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Julian Day Number of the Gregorian epoch: January 1, 1 CE (proleptic).
///
/// JDN 1,721,425.5 corresponds to midnight at the start of January 1,
/// year 1 in the proleptic Gregorian calendar.
pub const GREGORIAN_EPOCH_JDN: f64 = 1_721_425.5;

/// Days in each month of a common (non-leap) Gregorian year.
const GREGORIAN_MONTH_DAYS: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

// ---------------------------------------------------------------------------
// Month enum
// ---------------------------------------------------------------------------

/// The 12 months of the Gregorian calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum GregorianMonth {
    /// January (month 1, 31 days)
    January,
    /// February (month 2, 28 days, 29 in leap years)
    February,
    /// March (month 3, 31 days)
    March,
    /// April (month 4, 30 days)
    April,
    /// May (month 5, 31 days)
    May,
    /// June (month 6, 30 days)
    June,
    /// July (month 7, 31 days)
    July,
    /// August (month 8, 31 days)
    August,
    /// September (month 9, 30 days)
    September,
    /// October (month 10, 31 days)
    October,
    /// November (month 11, 30 days)
    November,
    /// December (month 12, 31 days)
    December,
}

const GREGORIAN_MONTHS: [GregorianMonth; 12] = [
    GregorianMonth::January,
    GregorianMonth::February,
    GregorianMonth::March,
    GregorianMonth::April,
    GregorianMonth::May,
    GregorianMonth::June,
    GregorianMonth::July,
    GregorianMonth::August,
    GregorianMonth::September,
    GregorianMonth::October,
    GregorianMonth::November,
    GregorianMonth::December,
];

impl core::fmt::Display for GregorianMonth {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = match self {
            Self::January => "January",
            Self::February => "February",
            Self::March => "March",
            Self::April => "April",
            Self::May => "May",
            Self::June => "June",
            Self::July => "July",
            Self::August => "August",
            Self::September => "September",
            Self::October => "October",
            Self::November => "November",
            Self::December => "December",
        };
        write!(f, "{name}")
    }
}

// ---------------------------------------------------------------------------
// Date struct
// ---------------------------------------------------------------------------

/// A date in the proleptic Gregorian calendar.
///
/// Uses astronomical year numbering: year 0 = 1 BCE, year -1 = 2 BCE, etc.
/// Positive years are CE, negative years are BCE offset by one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GregorianDate {
    /// Year (astronomical numbering: 0 = 1 BCE, negative = BCE).
    pub year: i64,
    /// Month.
    pub month: GregorianMonth,
    /// Day of month (1-28/29/30/31 depending on month and leap year).
    pub day: u8,
}

impl core::fmt::Display for GregorianDate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.year >= 1 {
            write!(f, "{} {} {} CE", self.day, self.month, self.year)
        } else {
            write!(f, "{} {} {} BCE", self.day, self.month, 1 - self.year)
        }
    }
}

// ---------------------------------------------------------------------------
// Leap year
// ---------------------------------------------------------------------------

/// Whether a Gregorian year is a leap year.
///
/// A year is a leap year if:
/// - It is divisible by 4, AND
/// - It is NOT divisible by 100, UNLESS
/// - It is also divisible by 400.
///
/// Examples: 2000 (leap), 1900 (not), 2024 (leap), 1600 (leap).
#[must_use]
#[inline]
pub fn gregorian_is_leap(year: i64) -> bool {
    (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0)
}

/// Days in a Gregorian year (365 common, 366 leap).
#[must_use]
#[inline]
pub fn gregorian_year_days(year: i64) -> u16 {
    if gregorian_is_leap(year) { 366 } else { 365 }
}

// ---------------------------------------------------------------------------
// JDN conversions (Dershowitz & Reingold algorithm)
// ---------------------------------------------------------------------------

/// Convert a Julian Day Number to a proleptic Gregorian date.
///
/// Uses a March-based computational calendar algorithm that works correctly
/// for all dates (including deep proleptic dates before 1 CE). Based on
/// the algorithms in Dershowitz & Reingold, *Calendrical Calculations*
/// (4th ed., 2018) and Richards, *Mapping Time* (Oxford, 1998).
#[must_use]
pub fn jdn_to_gregorian(jdn: f64) -> GregorianDate {
    tracing::trace!(jdn, "JDN to Gregorian");
    // Shift to midnight-based integer day count
    let j = (jdn + 0.5).floor() as i64;

    // March-based computational calendar (March 1 = day 0 of the year).
    // This avoids the leap day complication since Feb is the last month.
    // Reference epoch: March 1, year 0 (proleptic Gregorian) = JDN 1721120
    let days = j - 1_721_120;

    // 400-year cycle: 146097 days (97 leap years in 400 years)
    let n400 = days.div_euclid(146_097);
    let d400 = days.rem_euclid(146_097);

    // 100-year cycle: 36524 days (24 leap years in 100 years, except century)
    let n100 = (d400 / 36_524).min(3);
    let d100 = d400 - n100 * 36_524;

    // 4-year cycle: 1461 days
    let n4 = d100 / 1_461;
    let d4 = d100 - n4 * 1_461;

    // Single year: 365 days
    let n1 = (d4 / 365).min(3);
    let day_of_year = d4 - n1 * 365;

    // March-based month (0 = March, 11 = February)
    let mp = (5 * day_of_year + 2) / 153;
    let day = (day_of_year - (153 * mp + 2) / 5 + 1) as u8;

    // Convert March-based month to January-based
    let month_idx = if mp < 10 {
        (mp + 2) as usize
    } else {
        (mp - 10) as usize
    };

    let mut year = 400 * n400 + 100 * n100 + 4 * n4 + n1;
    if mp >= 10 {
        year += 1;
    }

    GregorianDate {
        year,
        month: GREGORIAN_MONTHS[month_idx],
        day,
    }
}

/// Convert a proleptic Gregorian date to a Julian Day Number.
///
/// Algorithm from Meeus, *Astronomical Algorithms* (2nd ed., 1998), ch. 7.
/// Returns midnight JDN (ending in .5).
///
/// # Errors
///
/// Returns [`SankhyaError::InvalidDate`] if the day is out of range for the
/// given month and year.
#[must_use = "returns the JDN or an error"]
pub fn gregorian_to_jdn(date: &GregorianDate) -> Result<f64> {
    tracing::trace!(year = date.year, ?date.month, day = date.day, "Gregorian to JDN");
    let month_idx = GREGORIAN_MONTHS
        .iter()
        .position(|&m| m == date.month)
        .unwrap_or(0);

    let max_day = if month_idx == 1 && gregorian_is_leap(date.year) {
        29
    } else {
        GREGORIAN_MONTH_DAYS[month_idx]
    };
    if date.day == 0 || date.day > max_day {
        return Err(SankhyaError::InvalidDate(format!(
            "day {} out of range for {} {} (max {max_day})",
            date.day, date.month, date.year
        )));
    }

    // March-based computational calendar (exact inverse of jdn_to_gregorian).
    // Convert Jan/Feb to months 10/11 of the previous year.
    let (y, mp) = if month_idx < 2 {
        (date.year - 1, (month_idx + 10) as i64)
    } else {
        (date.year, (month_idx - 2) as i64)
    };

    // Reconstruct day count from March 1, year 0 = JDN 1721120
    let days = 365 * y + y.div_euclid(4) - y.div_euclid(100)
        + y.div_euclid(400)
        + (153 * mp + 2) / 5
        + i64::from(date.day)
        - 1;

    Ok((days + 1_721_120) as f64 - 0.5)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epoch_roundtrip() {
        let date = jdn_to_gregorian(GREGORIAN_EPOCH_JDN);
        assert_eq!(date.year, 1);
        assert_eq!(date.month, GregorianMonth::January);
        assert_eq!(date.day, 1);

        let jdn = gregorian_to_jdn(&date).unwrap();
        assert!((jdn - GREGORIAN_EPOCH_JDN).abs() < f64::EPSILON);
    }

    #[test]
    fn known_date_j2000() {
        // J2000.0 = January 1, 2000 CE = JDN 2451545.0
        let date = jdn_to_gregorian(2_451_545.0);
        assert_eq!(date.year, 2000);
        assert_eq!(date.month, GregorianMonth::January);
        assert_eq!(date.day, 1);
    }

    #[test]
    fn known_date_mayan_end() {
        // December 21, 2012 = JDN 2456282.5
        let date = jdn_to_gregorian(2_456_282.5);
        assert_eq!(date.year, 2012);
        assert_eq!(date.month, GregorianMonth::December);
        assert_eq!(date.day, 21);
    }

    #[test]
    fn known_date_gregorian_reform() {
        // October 15, 1582 = JDN 2299160.5 (first day of Gregorian calendar)
        let date = jdn_to_gregorian(2_299_160.5);
        assert_eq!(date.year, 1582);
        assert_eq!(date.month, GregorianMonth::October);
        assert_eq!(date.day, 15);
    }

    #[test]
    fn leap_year_rules() {
        assert!(gregorian_is_leap(2000)); // div by 400
        assert!(!gregorian_is_leap(1900)); // div by 100 but not 400
        assert!(gregorian_is_leap(2024)); // div by 4
        assert!(gregorian_is_leap(1600)); // div by 400
        assert!(!gregorian_is_leap(2023)); // not div by 4
        assert!(gregorian_is_leap(0)); // year 0 (1 BCE) is leap
        assert!(!gregorian_is_leap(-1)); // year -1 (2 BCE) not leap
    }

    #[test]
    fn year_days() {
        assert_eq!(gregorian_year_days(2024), 366);
        assert_eq!(gregorian_year_days(2023), 365);
        assert_eq!(gregorian_year_days(2000), 366);
        assert_eq!(gregorian_year_days(1900), 365);
    }

    #[test]
    fn roundtrip_modern_dates() {
        // Use .5 JDN values (midnight convention, matching gregorian_to_jdn output)
        for jdn in [
            2_451_544.5, // 2000-01-01 midnight
            2_456_282.5, // 2012-12-21
            2_460_676.5, // 2025-01-01
            2_299_160.5, // 1582-10-15
        ] {
            let date = jdn_to_gregorian(jdn);
            let back = gregorian_to_jdn(&date).unwrap();
            assert!(
                (back - jdn).abs() < f64::EPSILON,
                "roundtrip failed for JDN {jdn}: got {back}"
            );
        }
    }

    #[test]
    fn roundtrip_ancient_dates() {
        // Test across a wide range including BCE dates (midnight JDNs)
        for jdn_int in (0..3_000_000).step_by(10_000) {
            let jdn = jdn_int as f64 + 0.5;
            let date = jdn_to_gregorian(jdn);
            let back = gregorian_to_jdn(&date).unwrap();
            assert!(
                (back - jdn).abs() < f64::EPSILON,
                "roundtrip failed for JDN {jdn}: year={}, month={:?}, day={}, back={back}",
                date.year,
                date.month,
                date.day
            );
        }
    }

    #[test]
    fn roundtrip_sequential_days() {
        // Test 1000 consecutive days around J2000.0 (midnight JDNs)
        for offset in 0..1000 {
            let jdn = 2_451_544.5 + f64::from(offset);
            let date = jdn_to_gregorian(jdn);
            let back = gregorian_to_jdn(&date).unwrap();
            assert!(
                (back - jdn).abs() < f64::EPSILON,
                "roundtrip failed for JDN {jdn}"
            );
        }
    }

    #[test]
    fn leap_feb_29() {
        // February 29, 2024 should be valid
        let date = GregorianDate {
            year: 2024,
            month: GregorianMonth::February,
            day: 29,
        };
        assert!(gregorian_to_jdn(&date).is_ok());
    }

    #[test]
    fn non_leap_feb_29_errors() {
        let date = GregorianDate {
            year: 2023,
            month: GregorianMonth::February,
            day: 29,
        };
        assert!(gregorian_to_jdn(&date).is_err());
    }

    #[test]
    fn invalid_day_zero() {
        let date = GregorianDate {
            year: 2000,
            month: GregorianMonth::January,
            day: 0,
        };
        assert!(gregorian_to_jdn(&date).is_err());
    }

    #[test]
    fn invalid_day_32() {
        let date = GregorianDate {
            year: 2000,
            month: GregorianMonth::January,
            day: 32,
        };
        assert!(gregorian_to_jdn(&date).is_err());
    }

    #[test]
    fn display_ce() {
        let date = GregorianDate {
            year: 2025,
            month: GregorianMonth::April,
            day: 1,
        };
        assert_eq!(date.to_string(), "1 April 2025 CE");
    }

    #[test]
    fn display_bce() {
        let date = GregorianDate {
            year: -43,
            month: GregorianMonth::March,
            day: 15,
        };
        assert_eq!(date.to_string(), "15 March 44 BCE");
    }

    #[test]
    fn serde_roundtrip() {
        let date = jdn_to_gregorian(2_451_545.0);
        let json = serde_json::to_string(&date).unwrap();
        let back: GregorianDate = serde_json::from_str(&json).unwrap();
        assert_eq!(date, back);
    }
}
