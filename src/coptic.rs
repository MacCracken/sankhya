//! Coptic (Alexandrian) calendar.
//!
//! Implements the Coptic calendar with Julian Day Number conversions,
//! following the algorithms in Dershowitz & Reingold, *Calendrical
//! Calculations* (4th ed., Cambridge University Press, 2018), ch. 4.
//!
//! # Historical Context
//!
//! The Coptic calendar descends from the ancient Egyptian civil calendar,
//! reformed under Augustus in 25 BCE to add a leap day every four years.
//! It has 12 months of 30 days each, plus a 13th short month (Nasie /
//! Pi Kogi Enavot) of 5 days (6 in leap years). The Coptic era (Anno
//! Martyrum) begins on August 29, 284 CE Julian, commemorating the
//! accession of Diocletian and the era of Christian martyrs in Egypt.
//!
//! The Ethiopian calendar shares this structure but uses a different
//! epoch (August 29, 8 CE Julian) and month names.

use serde::{Deserialize, Serialize};

use crate::error::{Result, SankhyaError};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Julian Day Number of the Coptic epoch: 1 Thout, Year 1 AM
/// (Anno Martyrum) = August 29, 284 CE Julian.
pub const COPTIC_EPOCH_JDN: f64 = 1_825_029.5;

/// Days in each month of a common (non-leap) Coptic year.
/// Months 1–12 have 30 days each; month 13 (Nasie) has 5 days (6 in leap).
const COPTIC_MONTH_DAYS: [u8; 13] = [30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 5];

// ---------------------------------------------------------------------------
// Month enum
// ---------------------------------------------------------------------------

/// The 13 months of the Coptic calendar.
///
/// Months 1–12 have 30 days each. Month 13 (Nasie) has 5 days in common
/// years and 6 in leap years.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CopticMonth {
    /// Thout (month 1, 30 days) — September 11/12
    Thout,
    /// Paopi (month 2, 30 days)
    Paopi,
    /// Hathor (month 3, 30 days)
    Hathor,
    /// Koiak (month 4, 30 days)
    Koiak,
    /// Tobi (month 5, 30 days)
    Tobi,
    /// Meshir (month 6, 30 days)
    Meshir,
    /// Paremhat (month 7, 30 days)
    Paremhat,
    /// Parmouti (month 8, 30 days)
    Parmouti,
    /// Pashons (month 9, 30 days)
    Pashons,
    /// Paoni (month 10, 30 days)
    Paoni,
    /// Epip (month 11, 30 days)
    Epip,
    /// Mesori (month 12, 30 days)
    Mesori,
    /// Nasie / Pi Kogi Enavot (month 13, 5 or 6 epagomenal days)
    Nasie,
}

const COPTIC_MONTHS: [CopticMonth; 13] = [
    CopticMonth::Thout,
    CopticMonth::Paopi,
    CopticMonth::Hathor,
    CopticMonth::Koiak,
    CopticMonth::Tobi,
    CopticMonth::Meshir,
    CopticMonth::Paremhat,
    CopticMonth::Parmouti,
    CopticMonth::Pashons,
    CopticMonth::Paoni,
    CopticMonth::Epip,
    CopticMonth::Mesori,
    CopticMonth::Nasie,
];

impl core::fmt::Display for CopticMonth {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = match self {
            Self::Thout => "Thout",
            Self::Paopi => "Paopi",
            Self::Hathor => "Hathor",
            Self::Koiak => "Koiak",
            Self::Tobi => "Tobi",
            Self::Meshir => "Meshir",
            Self::Paremhat => "Paremhat",
            Self::Parmouti => "Parmouti",
            Self::Pashons => "Pashons",
            Self::Paoni => "Paoni",
            Self::Epip => "Epip",
            Self::Mesori => "Mesori",
            Self::Nasie => "Nasie",
        };
        write!(f, "{name}")
    }
}

// ---------------------------------------------------------------------------
// Date struct
// ---------------------------------------------------------------------------

/// A date in the Coptic (Alexandrian) calendar.
///
/// Year 1 AM (Anno Martyrum) corresponds to 284 CE in the Julian calendar.
/// The Coptic year begins on Thout 1 (August 29 Julian in common years,
/// August 30 Julian in the year before a Julian leap year).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopticDate {
    /// Year (Anno Martyrum). Year 1 = 284 CE.
    pub year: i64,
    /// Month (one of the 13 Coptic months).
    pub month: CopticMonth,
    /// Day of month (1–30 for months 1–12, 1–5 or 1–6 for Nasie).
    pub day: u8,
}

impl core::fmt::Display for CopticDate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} {} {} AM", self.day, self.month, self.year)
    }
}

// ---------------------------------------------------------------------------
// Leap year
// ---------------------------------------------------------------------------

/// Whether a Coptic year is a leap year.
///
/// A Coptic year is a leap year if `year mod 4 == 3`. In leap years,
/// Nasie has 6 days instead of 5.
#[must_use]
#[inline]
pub fn coptic_is_leap(year: i64) -> bool {
    year.rem_euclid(4) == 3
}

/// Days in a Coptic year (365 common, 366 leap).
#[must_use]
#[inline]
pub fn coptic_year_days(year: i64) -> u16 {
    if coptic_is_leap(year) { 366 } else { 365 }
}

// ---------------------------------------------------------------------------
// JDN conversions
// ---------------------------------------------------------------------------

/// Convert a Julian Day Number to a Coptic date.
///
/// The Coptic calendar is a simple fixed calendar: 12 months of 30 days
/// plus 5 or 6 epagomenal days. Algorithm from Dershowitz & Reingold,
/// *Calendrical Calculations* (4th ed., 2018), ch. 4.
#[must_use]
pub fn jdn_to_coptic(jdn: f64) -> CopticDate {
    let days_since_epoch = (jdn - COPTIC_EPOCH_JDN).floor() as i64;

    // Helper: days from epoch to the start of a given 1-based year.
    // Leap years are 3, 7, 11... (year % 4 == 3). The number of leap
    // years before year y is floor(y / 4).
    let year_start = |year: i64| -> i64 {
        let y0 = year - 1;
        365 * y0 + year.div_euclid(4)
    };

    // Estimate year from days. Use 366 as divisor to avoid overestimating,
    // then correct upward. div_euclid handles negative days correctly.
    let mut year = days_since_epoch.div_euclid(366) + 1;

    // Correct upward: advance while next year's start is still <= our day count.
    while year_start(year + 1) <= days_since_epoch {
        year += 1;
    }
    // Correct downward if we overshot.
    while year_start(year) > days_since_epoch {
        year -= 1;
    }

    let day_of_year = days_since_epoch - year_start(year);

    // Month: each of the first 12 months has 30 days
    let month_idx = (day_of_year / 30).min(12) as usize;
    let day = (day_of_year - (month_idx as i64) * 30 + 1) as u8;

    CopticDate {
        year,
        month: COPTIC_MONTHS[month_idx],
        day,
    }
}

/// Convert a Coptic date to a Julian Day Number.
///
/// Algorithm from Dershowitz & Reingold, *Calendrical Calculations*
/// (4th ed., 2018), ch. 4.
///
/// # Errors
///
/// Returns [`SankhyaError::InvalidDate`] if the day is out of range for the
/// given month and year.
#[must_use = "returns the JDN or an error"]
pub fn coptic_to_jdn(date: &CopticDate) -> Result<f64> {
    let month_idx = COPTIC_MONTHS
        .iter()
        .position(|&m| m == date.month)
        .unwrap_or(0);

    let max_day = if month_idx == 12 && coptic_is_leap(date.year) {
        6
    } else {
        COPTIC_MONTH_DAYS[month_idx]
    };
    if date.day == 0 || date.day > max_day {
        return Err(SankhyaError::InvalidDate(format!(
            "day {} out of range for {} in year {} AM (max {max_day})",
            date.day, date.month, date.year
        )));
    }

    let y = date.year - 1; // 0-based year
    // Leap years: year % 4 == 3, so leap count before this year = year.div_euclid(4)
    let days = 365 * y + date.year.div_euclid(4) + 30 * month_idx as i64 + i64::from(date.day) - 1;

    Ok(COPTIC_EPOCH_JDN + days as f64)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epoch_roundtrip() {
        let date = jdn_to_coptic(COPTIC_EPOCH_JDN);
        assert_eq!(date.year, 1);
        assert_eq!(date.month, CopticMonth::Thout);
        assert_eq!(date.day, 1);

        let jdn = coptic_to_jdn(&date).unwrap();
        assert!((jdn - COPTIC_EPOCH_JDN).abs() < f64::EPSILON);
    }

    #[test]
    fn known_date_cross_check() {
        // 1 Thout 1741 AM = September 11, 2024 Gregorian
        // September 11, 2024 = JDN 2460564.5
        let date = jdn_to_coptic(2_460_564.5);
        assert_eq!(date.year, 1741);
        assert_eq!(date.month, CopticMonth::Thout);
        assert_eq!(date.day, 1);
    }

    #[test]
    fn leap_year_rules() {
        assert!(coptic_is_leap(3)); // 3 mod 4 == 3
        assert!(coptic_is_leap(7));
        assert!(!coptic_is_leap(1));
        assert!(!coptic_is_leap(2));
        assert!(!coptic_is_leap(4));
    }

    #[test]
    fn year_days() {
        assert_eq!(coptic_year_days(1), 365);
        assert_eq!(coptic_year_days(3), 366);
        assert_eq!(coptic_year_days(4), 365);
        assert_eq!(coptic_year_days(7), 366);
    }

    #[test]
    fn nasie_leap_6_days() {
        // In leap year 3, Nasie should accept day 6
        let date = CopticDate {
            year: 3,
            month: CopticMonth::Nasie,
            day: 6,
        };
        assert!(coptic_to_jdn(&date).is_ok());
    }

    #[test]
    fn nasie_common_rejects_6() {
        // In common year 1, Nasie should reject day 6
        let date = CopticDate {
            year: 1,
            month: CopticMonth::Nasie,
            day: 6,
        };
        assert!(coptic_to_jdn(&date).is_err());
    }

    #[test]
    fn invalid_day_zero() {
        let date = CopticDate {
            year: 1,
            month: CopticMonth::Thout,
            day: 0,
        };
        assert!(coptic_to_jdn(&date).is_err());
    }

    #[test]
    fn invalid_day_31() {
        let date = CopticDate {
            year: 1,
            month: CopticMonth::Thout,
            day: 31,
        };
        assert!(coptic_to_jdn(&date).is_err());
    }

    #[test]
    fn roundtrip_sequential_days() {
        // 1500 consecutive days from the epoch
        for offset in 0..1500 {
            let jdn = COPTIC_EPOCH_JDN + f64::from(offset);
            let date = jdn_to_coptic(jdn);
            let back = coptic_to_jdn(&date).unwrap();
            assert!(
                (back - jdn).abs() < f64::EPSILON,
                "roundtrip failed for JDN {jdn}: {date}"
            );
        }
    }

    #[test]
    fn roundtrip_wide_range() {
        // Test across a wide range of JDN values
        for jdn_int in (1_000_000..3_000_000).step_by(7_777) {
            let jdn = jdn_int as f64 + 0.5;
            let date = jdn_to_coptic(jdn);
            let back = coptic_to_jdn(&date).unwrap();
            assert!(
                (back - jdn).abs() < f64::EPSILON,
                "roundtrip failed for JDN {jdn}: {date}"
            );
        }
    }

    #[test]
    fn display_format() {
        let date = CopticDate {
            year: 1741,
            month: CopticMonth::Thout,
            day: 1,
        };
        assert_eq!(date.to_string(), "1 Thout 1741 AM");
    }

    #[test]
    fn serde_roundtrip() {
        let date = jdn_to_coptic(COPTIC_EPOCH_JDN + 500.0);
        let json = serde_json::to_string(&date).unwrap();
        let back: CopticDate = serde_json::from_str(&json).unwrap();
        assert_eq!(date, back);
    }

    #[test]
    fn month_display() {
        assert_eq!(CopticMonth::Thout.to_string(), "Thout");
        assert_eq!(CopticMonth::Nasie.to_string(), "Nasie");
        assert_eq!(CopticMonth::Mesori.to_string(), "Mesori");
    }
}
