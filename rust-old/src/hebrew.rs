//! Hebrew (Jewish) calendar.
//!
//! Implements the Hebrew lunisolar calendar with Julian Day Number
//! conversions, following the algorithms in Dershowitz & Reingold,
//! *Calendrical Calculations* (4th ed., Cambridge University Press, 2018),
//! ch. 8.
//!
//! # Historical Context
//!
//! The Hebrew calendar is a lunisolar calendar used for Jewish religious
//! observances and as the official calendar of Israel. It was standardized
//! by Hillel II in approximately 359 CE, replacing the earlier system of
//! observational new moons and intercalation by the Sanhedrin.
//!
//! The calendar uses the 19-year Metonic cycle (discovered independently
//! by Meton of Athens and Babylonian astronomers) to synchronize lunar
//! months with solar years: 12 common years of 12 months and 7 leap years
//! of 13 months per 19-year cycle. The molad (new moon) is computed
//! arithmetically, and four dehiyyot (postponement rules) adjust Rosh
//! Hashana to avoid religious conflicts.
//!
//! Year lengths vary: 353, 354, 355 (common) or 383, 384, 385 (leap),
//! producing six distinct year types.

use serde::{Deserialize, Serialize};

use crate::error::{Result, SankhyaError};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Julian Day Number of the Hebrew epoch: 1 Tishrei, Year 1 AM
/// (Anno Mundi). The molad of creation is Monday, September 7, 3761 BCE
/// proleptic Julian = JDN 347997. The midnight-based JDN is 347996.5.
pub const HEBREW_EPOCH_JDN: f64 = 347_996.5;

/// Parts (chalakim) in an hour. 1 hour = 1080 parts.
const PARTS_PER_HOUR: i64 = 1080;

/// Parts in a day. 1 day = 24 * 1080 = 25920 parts.
const PARTS_PER_DAY: i64 = 24 * PARTS_PER_HOUR;

/// Mean synodic month in parts: 29 days, 12 hours, 793 parts.
/// = 29 * 25920 + 12 * 1080 + 793 = 765433 parts.
const MONTH_PARTS: i64 = 29 * PARTS_PER_DAY + 12 * PARTS_PER_HOUR + 793;

// ---------------------------------------------------------------------------
// Month enum
// ---------------------------------------------------------------------------

/// The months of the Hebrew calendar.
///
/// In a common year there are 12 months. In a leap year, Adar is replaced
/// by Adar I and Adar II (13 months total). This enum includes all 13
/// possible months.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum HebrewMonth {
    /// Tishrei (month 1, 30 days) — Rosh Hashana
    Tishrei,
    /// Cheshvan (month 2, 29 or 30 days)
    Cheshvan,
    /// Kislev (month 3, 29 or 30 days)
    Kislev,
    /// Tevet (month 4, 29 days)
    Tevet,
    /// Shevat (month 5, 30 days)
    Shevat,
    /// Adar (month 6 in common years, 29 days)
    Adar,
    /// Adar I (month 6 in leap years, 30 days)
    AdarI,
    /// Adar II (month 7 in leap years, 29 days)
    AdarII,
    /// Nisan (month 7/8, 30 days)
    Nisan,
    /// Iyar (month 8/9, 29 days)
    Iyar,
    /// Sivan (month 9/10, 30 days)
    Sivan,
    /// Tammuz (month 10/11, 29 days)
    Tammuz,
    /// Av (month 11/12, 30 days)
    Av,
    /// Elul (month 12/13, 29 days)
    Elul,
}

impl core::fmt::Display for HebrewMonth {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = match self {
            Self::Tishrei => "Tishrei",
            Self::Cheshvan => "Cheshvan",
            Self::Kislev => "Kislev",
            Self::Tevet => "Tevet",
            Self::Shevat => "Shevat",
            Self::Adar => "Adar",
            Self::AdarI => "Adar I",
            Self::AdarII => "Adar II",
            Self::Nisan => "Nisan",
            Self::Iyar => "Iyar",
            Self::Sivan => "Sivan",
            Self::Tammuz => "Tammuz",
            Self::Av => "Av",
            Self::Elul => "Elul",
        };
        write!(f, "{name}")
    }
}

/// Months in order for a common (non-leap) year.
const COMMON_MONTHS: [HebrewMonth; 12] = [
    HebrewMonth::Tishrei,
    HebrewMonth::Cheshvan,
    HebrewMonth::Kislev,
    HebrewMonth::Tevet,
    HebrewMonth::Shevat,
    HebrewMonth::Adar,
    HebrewMonth::Nisan,
    HebrewMonth::Iyar,
    HebrewMonth::Sivan,
    HebrewMonth::Tammuz,
    HebrewMonth::Av,
    HebrewMonth::Elul,
];

/// Months in order for a leap year.
const LEAP_MONTHS: [HebrewMonth; 13] = [
    HebrewMonth::Tishrei,
    HebrewMonth::Cheshvan,
    HebrewMonth::Kislev,
    HebrewMonth::Tevet,
    HebrewMonth::Shevat,
    HebrewMonth::AdarI,
    HebrewMonth::AdarII,
    HebrewMonth::Nisan,
    HebrewMonth::Iyar,
    HebrewMonth::Sivan,
    HebrewMonth::Tammuz,
    HebrewMonth::Av,
    HebrewMonth::Elul,
];

// ---------------------------------------------------------------------------
// Date struct
// ---------------------------------------------------------------------------

/// A date in the Hebrew (Jewish) calendar.
///
/// Year 1 AM (Anno Mundi) corresponds to 3761 BCE in the proleptic
/// Julian calendar. The year begins on 1 Tishrei (Rosh Hashana).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HebrewDate {
    /// Year (Anno Mundi). Year 1 = 3761 BCE.
    pub year: i64,
    /// Month.
    pub month: HebrewMonth,
    /// Day of month (1–30).
    pub day: u8,
}

impl core::fmt::Display for HebrewDate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} {} {} AM", self.day, self.month, self.year)
    }
}

// ---------------------------------------------------------------------------
// Leap year and year properties
// ---------------------------------------------------------------------------

/// Whether a Hebrew year is a leap year (13 months instead of 12).
///
/// Years 3, 6, 8, 11, 14, 17, 19 in the 19-year Metonic cycle are leap.
#[must_use]
#[inline]
pub fn hebrew_is_leap(year: i64) -> bool {
    let r = ((7 * year) + 1).rem_euclid(19);
    r < 7
}

/// Compute the molad (mean new moon) of Tishrei for a given year,
/// in parts (chalakim) since the epoch.
///
/// The molad of Tishrei year 1 is: Monday, 5 hours, 204 parts
/// (= day 2, 5h 204p from Sunday 0h). The mean synodic month is
/// 29 days, 12 hours, 793 parts (29.530594 days).
///
/// Algorithm from Dershowitz & Reingold, *Calendrical Calculations*
/// (4th ed., 2018), ch. 8. Constants from Maimonides, *Mishneh Torah*,
/// Hilchot Kiddush HaChodesh.
fn molad_tishrei(year: i64) -> i64 {
    // Months elapsed since creation to the start of this year
    let months_elapsed = months_before_year(year);

    // Molad of year 1: day 1 (Monday), 5 hours, 204 parts from Sunday 0h
    // = 1 * 25920 + 5 * 1080 + 204 = 31524 parts
    let molad_epoch = PARTS_PER_DAY + 5 * PARTS_PER_HOUR + 204;

    molad_epoch + months_elapsed * MONTH_PARTS
}

/// Total months elapsed from creation to the start of a Hebrew year.
fn months_before_year(year: i64) -> i64 {
    let y = year - 1;
    let cycles = y.div_euclid(19);
    let remainder = y.rem_euclid(19);

    // Each 19-year cycle has 235 months (12*19 + 7 leap months)
    let mut months = cycles * 235;

    // Count months in partial cycle
    for i in 0..remainder {
        let yr = i + 1;
        months += if hebrew_is_leap_in_cycle(yr) { 13 } else { 12 };
    }

    months
}

/// Whether a year position (1-based) within a 19-year cycle is a leap year.
fn hebrew_is_leap_in_cycle(pos: i64) -> bool {
    matches!(pos, 3 | 6 | 8 | 11 | 14 | 17 | 19)
}

/// Compute the JDN of 1 Tishrei (Rosh Hashana) for a given year,
/// applying the four dehiyyot (postponement rules).
///
/// The four dehiyyot (D&R ch. 8, §8.1):
/// 1. Lo ADU Rosh — Rosh Hashana cannot fall on Sunday, Wednesday, or Friday
/// 2. Molad Zaken — if the molad is at or after noon (18h), postpone one day
/// 3. GaTRaD — in a common year, if molad falls on Tuesday ≥ 9h 204p, postpone to Thursday
/// 4. BeTUTeKPaT — after a leap year, if molad falls on Monday ≥ 15h 589p, postpone to Tuesday
fn rosh_hashana_jdn(year: i64) -> i64 {
    let molad = molad_tishrei(year);
    let day = molad.div_euclid(PARTS_PER_DAY);
    let parts_in_day = molad.rem_euclid(PARTS_PER_DAY);

    // Day of week (0=Sunday, 1=Monday, ..., 6=Saturday)
    let dow = day.rem_euclid(7);

    // Convert molad day to JDN
    // Molad epoch (1 Tishrei year 1) corresponds to JDN 347997
    // (Monday, October 7, 3761 BCE proleptic Julian)
    // The molad day count starts at day 1 = Monday of creation week.
    // day 1 = JDN 347997, so JDN = day + 347996
    let mut jdn = day + 347_996;

    // Dehiyya 1 (Lo ADU Rosh): Rosh Hashana cannot fall on Sunday (1),
    // Wednesday (4), or Friday (6).
    let mut postpone = 0;
    if dow == 0 || dow == 3 || dow == 5 {
        postpone = 1;
    }

    // Dehiyya 2 (Molad Zaken): If the molad is at or after noon (18 hours = 18*1080 parts),
    // postpone by one day.
    if parts_in_day >= 18 * PARTS_PER_HOUR {
        postpone = 1;
        // Check if the postponed day also falls on ADU
        let new_dow = (dow + 1).rem_euclid(7);
        if new_dow == 0 || new_dow == 3 || new_dow == 5 {
            postpone = 2;
        }
    }

    // Dehiyya 3 (GaTRaD): In a common year, if the molad of Tishrei falls
    // on Tuesday at or after 9 hours 204 parts, postpone to Thursday.
    if !hebrew_is_leap(year) && dow == 2 && parts_in_day >= 9 * PARTS_PER_HOUR + 204 {
        postpone = 2; // Tuesday -> Thursday
    }

    // Dehiyya 4 (BeTUTeKPaT): In a year following a leap year, if the molad
    // falls on Monday at or after 15 hours 589 parts, postpone to Tuesday.
    if hebrew_is_leap(year - 1) && dow == 1 && parts_in_day >= 15 * PARTS_PER_HOUR + 589 {
        postpone = 1;
    }

    jdn += postpone;
    jdn
}

/// Days in a Hebrew year.
///
/// Returns 353, 354, 355 (common) or 383, 384, 385 (leap), depending
/// on the year type (deficient/regular/complete × common/leap).
/// Computed from the difference between consecutive Rosh Hashana dates.
/// See Dershowitz & Reingold, *Calendrical Calculations* (4th ed.), §8.2.
#[must_use]
pub fn hebrew_year_days(year: i64) -> u16 {
    (rosh_hashana_jdn(year + 1) - rosh_hashana_jdn(year)) as u16
}

/// Days in a specific Hebrew month for a given year.
fn hebrew_month_days(year: i64, month: HebrewMonth) -> u8 {
    let year_len = hebrew_year_days(year);
    match month {
        HebrewMonth::Tishrei => 30,
        HebrewMonth::Cheshvan => {
            // 30 in "complete" years (355 or 385), 29 otherwise
            if year_len % 10 == 5 { 30 } else { 29 }
        }
        HebrewMonth::Kislev => {
            // 29 in "deficient" years (353 or 383), 30 otherwise
            if year_len % 10 == 3 { 29 } else { 30 }
        }
        HebrewMonth::Tevet => 29,
        HebrewMonth::Shevat => 30,
        HebrewMonth::Adar => 29,
        HebrewMonth::AdarI => 30,
        HebrewMonth::AdarII => 29,
        HebrewMonth::Nisan => 30,
        HebrewMonth::Iyar => 29,
        HebrewMonth::Sivan => 30,
        HebrewMonth::Tammuz => 29,
        HebrewMonth::Av => 30,
        HebrewMonth::Elul => 29,
    }
}

// ---------------------------------------------------------------------------
// JDN conversions
// ---------------------------------------------------------------------------

/// Convert a Julian Day Number to a Hebrew date.
///
/// Uses the Rosh Hashana JDN computation to find the year, then
/// decomposes the day-of-year into month and day.
#[must_use]
pub fn jdn_to_hebrew(jdn: f64) -> HebrewDate {
    tracing::trace!(jdn, "JDN to Hebrew");
    let jdn_int = (jdn + 0.5).floor() as i64;

    // Estimate year: approximately (jdn - epoch) / 365.25 + 1
    let approx = ((jdn_int - 347_996) as f64 / 365.25).floor() as i64 + 1;
    let mut year = approx;

    // Correct: find the year whose Rosh Hashana is <= jdn_int
    while rosh_hashana_jdn(year + 1) <= jdn_int {
        year += 1;
    }
    while rosh_hashana_jdn(year) > jdn_int {
        year -= 1;
    }

    let day_of_year = jdn_int - rosh_hashana_jdn(year);
    let is_leap = hebrew_is_leap(year);

    // Decompose day-of-year into month and day
    let months: &[HebrewMonth] = if is_leap {
        &LEAP_MONTHS
    } else {
        &COMMON_MONTHS
    };

    let mut remaining = day_of_year;
    let mut month = months[0];
    for &m in months {
        let md = i64::from(hebrew_month_days(year, m));
        if remaining < md {
            month = m;
            break;
        }
        remaining -= md;
        month = m;
    }

    HebrewDate {
        year,
        month,
        day: remaining as u8 + 1,
    }
}

/// Convert a Hebrew date to a Julian Day Number.
///
/// # Errors
///
/// Returns [`SankhyaError::InvalidDate`] if the day is out of range for the
/// given month and year, or if the month is invalid for the year type
/// (e.g., Adar I in a common year).
#[must_use = "returns the JDN or an error"]
pub fn hebrew_to_jdn(date: &HebrewDate) -> Result<f64> {
    tracing::trace!(year = date.year, ?date.month, day = date.day, "Hebrew to JDN");
    let is_leap = hebrew_is_leap(date.year);

    // Validate month for year type
    if !is_leap && (date.month == HebrewMonth::AdarI || date.month == HebrewMonth::AdarII) {
        return Err(SankhyaError::InvalidDate(format!(
            "{} is not valid in common year {} AM",
            date.month, date.year
        )));
    }
    if is_leap && date.month == HebrewMonth::Adar {
        return Err(SankhyaError::InvalidDate(format!(
            "Adar is not valid in leap year {} AM (use Adar I or Adar II)",
            date.year
        )));
    }

    let max_day = hebrew_month_days(date.year, date.month);
    if date.day == 0 || date.day > max_day {
        return Err(SankhyaError::InvalidDate(format!(
            "day {} out of range for {} in year {} AM (max {max_day})",
            date.day, date.month, date.year
        )));
    }

    let months: &[HebrewMonth] = if is_leap {
        &LEAP_MONTHS
    } else {
        &COMMON_MONTHS
    };

    let mut day_of_year = i64::from(date.day) - 1;
    for &m in months {
        if m == date.month {
            break;
        }
        day_of_year += i64::from(hebrew_month_days(date.year, m));
    }

    let jdn = rosh_hashana_jdn(date.year) + day_of_year;
    Ok(jdn as f64 - 0.5)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epoch_roundtrip() {
        let date = jdn_to_hebrew(HEBREW_EPOCH_JDN);
        assert_eq!(date.year, 1);
        assert_eq!(date.month, HebrewMonth::Tishrei);
        assert_eq!(date.day, 1);

        let jdn = hebrew_to_jdn(&date).unwrap();
        assert!((jdn - HEBREW_EPOCH_JDN).abs() < f64::EPSILON);
    }

    #[test]
    fn known_date_passover_5785() {
        // 15 Nisan 5785 AM = April 12/13, 2025 Gregorian
        // (Hebrew day begins at sunset, arithmetic calendar uses JDN midnight)
        // Verify the roundtrip: construct the date and check it resolves correctly.
        let date = HebrewDate {
            year: 5785,
            month: HebrewMonth::Nisan,
            day: 15,
        };
        let jdn = hebrew_to_jdn(&date).unwrap();
        let back = jdn_to_hebrew(jdn);
        assert_eq!(back, date);
    }

    #[test]
    fn leap_year_metonic() {
        // Years 3, 6, 8, 11, 14, 17, 19 in the 19-year cycle
        assert!(hebrew_is_leap(3));
        assert!(hebrew_is_leap(6));
        assert!(hebrew_is_leap(8));
        assert!(hebrew_is_leap(11));
        assert!(hebrew_is_leap(14));
        assert!(hebrew_is_leap(17));
        assert!(hebrew_is_leap(19));
        // Non-leap
        assert!(!hebrew_is_leap(1));
        assert!(!hebrew_is_leap(2));
        assert!(!hebrew_is_leap(4));
        assert!(!hebrew_is_leap(5));
    }

    #[test]
    fn year_lengths_valid() {
        // Check first 100 years — all should be 353/354/355/383/384/385
        for y in 1..=100 {
            let days = hebrew_year_days(y);
            assert!(
                matches!(days, 353 | 354 | 355 | 383 | 384 | 385),
                "year {y}: invalid length {days}"
            );
            // Leap years should be 383-385, common 353-355
            if hebrew_is_leap(y) {
                assert!(days >= 383, "leap year {y} has only {days} days");
            } else {
                assert!(days <= 355, "common year {y} has {days} days");
            }
        }
    }

    #[test]
    fn adar_in_leap_year_errors() {
        // Adar (plain) should be rejected in a leap year
        let date = HebrewDate {
            year: 5784, // leap year
            month: HebrewMonth::Adar,
            day: 1,
        };
        assert!(hebrew_to_jdn(&date).is_err());
    }

    #[test]
    fn adar_i_in_common_year_errors() {
        let date = HebrewDate {
            year: 5785, // common year
            month: HebrewMonth::AdarI,
            day: 1,
        };
        assert!(hebrew_to_jdn(&date).is_err());
    }

    #[test]
    fn invalid_day_zero() {
        let date = HebrewDate {
            year: 5785,
            month: HebrewMonth::Tishrei,
            day: 0,
        };
        assert!(hebrew_to_jdn(&date).is_err());
    }

    #[test]
    fn roundtrip_sequential_days() {
        // 1500 consecutive days around a modern year
        let start_jdn = 2_460_000.5;
        for offset in 0..1500 {
            let jdn = start_jdn + f64::from(offset);
            let date = jdn_to_hebrew(jdn);
            let back = hebrew_to_jdn(&date).unwrap();
            assert!(
                (back - jdn).abs() < f64::EPSILON,
                "roundtrip failed for JDN {jdn}: {date}"
            );
        }
    }

    #[test]
    fn roundtrip_across_leap_boundary() {
        // Test around a leap/common year transition
        for y in 5780..=5790 {
            let rh_jdn = rosh_hashana_jdn(y) as f64 - 0.5;
            let date = jdn_to_hebrew(rh_jdn);
            assert_eq!(date.year, y);
            assert_eq!(date.month, HebrewMonth::Tishrei);
            assert_eq!(date.day, 1);
            let back = hebrew_to_jdn(&date).unwrap();
            assert!(
                (back - rh_jdn).abs() < f64::EPSILON,
                "Rosh Hashana roundtrip failed for year {y}"
            );
        }
    }

    #[test]
    fn display_format() {
        let date = HebrewDate {
            year: 5785,
            month: HebrewMonth::Nisan,
            day: 15,
        };
        assert_eq!(date.to_string(), "15 Nisan 5785 AM");
    }

    #[test]
    fn serde_roundtrip() {
        let date = jdn_to_hebrew(2_460_000.5);
        let json = serde_json::to_string(&date).unwrap();
        let back: HebrewDate = serde_json::from_str(&json).unwrap();
        assert_eq!(date, back);
    }

    #[test]
    fn month_display() {
        assert_eq!(HebrewMonth::Tishrei.to_string(), "Tishrei");
        assert_eq!(HebrewMonth::AdarI.to_string(), "Adar I");
        assert_eq!(HebrewMonth::AdarII.to_string(), "Adar II");
        assert_eq!(HebrewMonth::Elul.to_string(), "Elul");
    }
}
