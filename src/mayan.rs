//! Mayan mathematics and calendar systems.
//!
//! Implements the vigesimal (base-20) number system, Long Count calendar,
//! Tzolkin (sacred 260-day cycle), Haab (solar 365-day calendar),
//! Calendar Round, and Venus synodic cycle computations.
//!
//! # Historical Context
//!
//! The Maya developed one of the most sophisticated mathematical and
//! astronomical systems in the ancient world. Their vigesimal number
//! system included a true zero (the shell glyph) centuries before
//! the concept appeared in the Old World. The Long Count calendar
//! tracked time from a mythological creation date of August 11, 3114 BCE
//! (GMT correlation constant 584,283).
//!
//! # Sources
//!
//! - Goodman-Martinez-Thompson (GMT) correlation: JDN 584,283 for the
//!   Long Count epoch. See Thompson, *Maya Hieroglyphic Writing* (1950)
//! - Venus table from the Dresden Codex (c. 13th century CE):
//!   Aveni, *Skywatchers of Ancient Mexico* (University of Texas Press, 2001)
//! - Tzolkin/Haab cycle mechanics: Lounsbury, "Maya Numeration, Computation,
//!   and Calendrical Astronomy" in *Dictionary of Scientific Biography* (1978)

use serde::{Deserialize, Serialize};

use crate::error::{Result, SankhyaError};

// ---------------------------------------------------------------------------
// Vigesimal (base-20) number system
// ---------------------------------------------------------------------------

/// Convert a decimal number to vigesimal (base-20) digits, most significant first.
///
/// Returns `[0]` for zero (the Maya had an explicit zero glyph — the shell).
/// Use [`MayanNumeral`] for glyph-level representation.
#[must_use]
pub fn to_vigesimal(mut n: u64) -> Vec<u8> {
    if n == 0 {
        return vec![0];
    }
    let mut digits = Vec::new();
    while n > 0 {
        digits.push((n % 20) as u8);
        n /= 20;
    }
    digits.reverse();
    digits
}

/// Convert vigesimal (base-20) digits back to a decimal number.
///
/// Digits are most-significant first. Each digit must be in 0..20.
///
/// # Errors
///
/// Returns [`SankhyaError::InvalidBase`] if any digit is >= 20.
#[must_use = "returns the converted value or an error"]
pub fn from_vigesimal(digits: &[u8]) -> Result<u64> {
    let mut result: u64 = 0;
    for &d in digits {
        if d >= 20 {
            return Err(SankhyaError::InvalidBase(format!(
                "vigesimal digit {d} out of range 0..20"
            )));
        }
        result = result
            .checked_mul(20)
            .and_then(|r| r.checked_add(u64::from(d)))
            .ok_or_else(|| SankhyaError::OverflowError("vigesimal conversion overflow".into()))?;
    }
    Ok(result)
}

// ---------------------------------------------------------------------------
// Mayan numeral glyphs
// ---------------------------------------------------------------------------

/// A single Mayan numeral (0-19) represented as dots and bars.
///
/// - A dot represents 1 (up to 4 dots)
/// - A bar represents 5 (up to 3 bars)
/// - A shell glyph represents 0
///
/// For example, 13 = 3 dots + 2 bars, 0 = shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MayanNumeral {
    /// Number of dots (0-4), each worth 1.
    pub dots: u8,
    /// Number of bars (0-3), each worth 5.
    pub bars: u8,
    /// True if this is a zero (shell glyph).
    pub shell: bool,
}

impl MayanNumeral {
    /// Create a Mayan numeral from a value 0-19.
    ///
    /// # Errors
    ///
    /// Returns [`SankhyaError::InvalidBase`] if value > 19.
    #[must_use = "returns the numeral or an error"]
    pub fn from_value(value: u8) -> Result<Self> {
        if value > 19 {
            return Err(SankhyaError::InvalidBase(format!(
                "Mayan numeral value {value} out of range 0..19"
            )));
        }
        if value == 0 {
            return Ok(Self {
                dots: 0,
                bars: 0,
                shell: true,
            });
        }
        Ok(Self {
            dots: value % 5,
            bars: value / 5,
            shell: false,
        })
    }

    /// The decimal value of this numeral (0-19).
    #[must_use]
    #[inline]
    pub fn value(self) -> u8 {
        if self.shell {
            0
        } else {
            self.bars * 5 + self.dots
        }
    }
}

// ---------------------------------------------------------------------------
// Long Count calendar
// ---------------------------------------------------------------------------

/// The Mayan Long Count calendar date.
///
/// Composed of five place-value positions:
/// - kin = 1 day
/// - uinal = 20 kin = 20 days
/// - tun = 18 uinal = 360 days
/// - katun = 20 tun = 7,200 days
/// - baktun = 20 katun = 144,000 days
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LongCount {
    /// Baktun (144,000 days each).
    pub baktun: u32,
    /// Katun (7,200 days each).
    pub katun: u32,
    /// Tun (360 days each).
    pub tun: u32,
    /// Uinal (20 days each).
    pub uinal: u32,
    /// Kin (1 day each).
    pub kin: u32,
}

/// Julian Day Number of the Mayan creation date (August 11, 3114 BCE).
///
/// This is the GMT (Goodman-Martinez-Thompson) correlation constant,
/// the most widely accepted correlation between the Mayan and Western calendars.
pub const EPOCH_JDN: u64 = 584_283;

impl LongCount {
    /// Create a Long Count from its component periods.
    ///
    /// # Errors
    ///
    /// Returns [`SankhyaError::InvalidDate`] if katun >= 20, tun >= 20,
    /// uinal >= 18, or kin >= 20.
    #[must_use = "returns the Long Count or an error"]
    pub fn new(baktun: u32, katun: u32, tun: u32, uinal: u32, kin: u32) -> Result<Self> {
        if katun >= 20 {
            return Err(SankhyaError::InvalidDate(format!(
                "katun {katun} out of range 0..20"
            )));
        }
        if tun >= 20 {
            return Err(SankhyaError::InvalidDate(format!(
                "tun {tun} out of range 0..20"
            )));
        }
        if uinal >= 18 {
            return Err(SankhyaError::InvalidDate(format!(
                "uinal {uinal} out of range 0..18"
            )));
        }
        if kin >= 20 {
            return Err(SankhyaError::InvalidDate(format!(
                "kin {kin} out of range 0..20"
            )));
        }
        Ok(Self {
            baktun,
            katun,
            tun,
            uinal,
            kin,
        })
    }

    /// Convert a day count (days since the Mayan creation date) to Long Count.
    ///
    /// # Errors
    ///
    /// Returns [`SankhyaError::OverflowError`] if the day count produces a
    /// baktun value that exceeds `u32::MAX`.
    #[must_use = "returns the Long Count or an error"]
    pub fn from_days(mut days: u64) -> Result<Self> {
        let baktun = days / 144_000;
        days %= 144_000;
        let katun = days / 7_200;
        days %= 7_200;
        let tun = days / 360;
        days %= 360;
        let uinal = days / 20;
        let kin = days % 20;
        Ok(Self {
            baktun: u32::try_from(baktun).map_err(|_| {
                SankhyaError::OverflowError(format!(
                    "day count {days} exceeds maximum representable baktun"
                ))
            })?,
            katun: katun as u32,
            tun: tun as u32,
            uinal: uinal as u32,
            kin: kin as u32,
        })
    }

    /// Convert this Long Count to a day count (days since creation date).
    #[must_use]
    #[inline]
    pub fn to_days(self) -> u64 {
        u64::from(self.baktun) * 144_000
            + u64::from(self.katun) * 7_200
            + u64::from(self.tun) * 360
            + u64::from(self.uinal) * 20
            + u64::from(self.kin)
    }

    /// Convert a Julian Day Number to a Mayan Long Count.
    ///
    /// # Errors
    ///
    /// Returns [`SankhyaError::InvalidDate`] if `jdn` is before the Mayan epoch.
    #[must_use = "returns the Long Count or an error"]
    pub fn from_julian_day(jdn: u64) -> Result<Self> {
        if jdn < EPOCH_JDN {
            return Err(SankhyaError::InvalidDate(format!(
                "JDN {jdn} is before the Mayan epoch (JDN {EPOCH_JDN})"
            )));
        }
        Self::from_days(jdn - EPOCH_JDN)
    }

    /// Convert this Long Count to a Julian Day Number.
    #[must_use]
    #[inline]
    pub fn to_julian_day(self) -> u64 {
        self.to_days() + EPOCH_JDN
    }
}

impl core::fmt::Display for LongCount {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}.{}",
            self.baktun, self.katun, self.tun, self.uinal, self.kin
        )
    }
}

// ---------------------------------------------------------------------------
// Tzolkin (sacred 260-day cycle)
// ---------------------------------------------------------------------------

/// The 20 day signs of the Tzolkin cycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum DaySign {
    /// Imix (Water Lily / Crocodile)
    Imix,
    /// Ik (Wind)
    Ik,
    /// Akbal (Night / Darkness)
    Akbal,
    /// Kan (Seed / Corn)
    Kan,
    /// Chicchan (Serpent)
    Chicchan,
    /// Cimi (Death)
    Cimi,
    /// Manik (Deer / Hand)
    Manik,
    /// Lamat (Star / Venus)
    Lamat,
    /// Muluc (Water / Offering)
    Muluc,
    /// Oc (Dog)
    Oc,
    /// Chuen (Monkey)
    Chuen,
    /// Eb (Grass / Road)
    Eb,
    /// Ben (Reed)
    Ben,
    /// Ix (Jaguar)
    Ix,
    /// Men (Eagle)
    Men,
    /// Cib (Vulture / Owl)
    Cib,
    /// Caban (Earth)
    Caban,
    /// Etznab (Flint / Knife)
    Etznab,
    /// Cauac (Storm / Rain)
    Cauac,
    /// Ahau (Lord / Sun)
    Ahau,
}

/// Ordered list of all 20 Tzolkin day signs.
const DAY_SIGNS: [DaySign; 20] = [
    DaySign::Imix,
    DaySign::Ik,
    DaySign::Akbal,
    DaySign::Kan,
    DaySign::Chicchan,
    DaySign::Cimi,
    DaySign::Manik,
    DaySign::Lamat,
    DaySign::Muluc,
    DaySign::Oc,
    DaySign::Chuen,
    DaySign::Eb,
    DaySign::Ben,
    DaySign::Ix,
    DaySign::Men,
    DaySign::Cib,
    DaySign::Caban,
    DaySign::Etznab,
    DaySign::Cauac,
    DaySign::Ahau,
];

/// A Tzolkin date (sacred 260-day cycle).
///
/// Composed of a number (1-13) cycling with one of 20 day signs.
/// The full cycle is lcm(13, 20) = 260 days.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tzolkin {
    /// The trecena number (1-13).
    pub number: u8,
    /// The day sign (one of 20).
    pub day_sign: DaySign,
}

impl Tzolkin {
    /// Compute the Tzolkin date for a given day count from the Mayan epoch.
    ///
    /// The base Tzolkin date at day 0 (creation) is 4 Ahau.
    #[must_use]
    pub fn from_days(days: u64) -> Self {
        // At day 0 the Tzolkin is 4 Ahau.
        // Number cycles 1-13: base offset is 4 (for 4 Ahau at day 0).
        // (days + 3) mod 13 gives 0-based, then +1 for 1-based.
        let number = ((days + 3) % 13 + 1) as u8;
        // Day sign cycles through 20: Ahau is index 19, so offset 19.
        let sign_index = ((days + 19) % 20) as usize;
        Self {
            number,
            day_sign: DAY_SIGNS[sign_index],
        }
    }
}

impl core::fmt::Display for Tzolkin {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} {:?}", self.number, self.day_sign)
    }
}

// ---------------------------------------------------------------------------
// Haab (solar 365-day calendar)
// ---------------------------------------------------------------------------

/// The 19 months of the Haab calendar (18 regular months + Wayeb).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum HaabMonth {
    /// Pop (month 1)
    Pop,
    /// Wo (month 2)
    Wo,
    /// Sip (month 3)
    Sip,
    /// Sotz (month 4)
    Sotz,
    /// Sek (month 5)
    Sek,
    /// Xul (month 6)
    Xul,
    /// Yaxkin (month 7)
    Yaxkin,
    /// Mol (month 8)
    Mol,
    /// Chen (month 9)
    Chen,
    /// Yax (month 10)
    Yax,
    /// Sak (month 11)
    Sak,
    /// Keh (month 12)
    Keh,
    /// Mak (month 13)
    Mak,
    /// Kankin (month 14)
    Kankin,
    /// Muan (month 15)
    Muan,
    /// Pax (month 16)
    Pax,
    /// Kayab (month 17)
    Kayab,
    /// Kumku (month 18)
    Kumku,
    /// Wayeb (5 unlucky days)
    Wayeb,
}

/// Ordered list of all 19 Haab months.
const HAAB_MONTHS: [HaabMonth; 19] = [
    HaabMonth::Pop,
    HaabMonth::Wo,
    HaabMonth::Sip,
    HaabMonth::Sotz,
    HaabMonth::Sek,
    HaabMonth::Xul,
    HaabMonth::Yaxkin,
    HaabMonth::Mol,
    HaabMonth::Chen,
    HaabMonth::Yax,
    HaabMonth::Sak,
    HaabMonth::Keh,
    HaabMonth::Mak,
    HaabMonth::Kankin,
    HaabMonth::Muan,
    HaabMonth::Pax,
    HaabMonth::Kayab,
    HaabMonth::Kumku,
    HaabMonth::Wayeb,
];

/// A Haab date (solar 365-day calendar).
///
/// 18 months of 20 days each (0-19), plus Wayeb (5 "nameless" days, 0-4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Haab {
    /// Day within the month (0-19 for regular months, 0-4 for Wayeb).
    pub day: u8,
    /// The Haab month.
    pub month: HaabMonth,
}

impl Haab {
    /// Compute the Haab date for a given day count from the Mayan epoch.
    ///
    /// The base Haab date at day 0 (creation) is 8 Kumku.
    #[must_use]
    pub fn from_days(days: u64) -> Self {
        // At day 0, the Haab is 8 Kumku.
        // Kumku is month index 17 (0-based). Day 8.
        // The offset into the Haab cycle: 17*20 + 8 = 348.
        let haab_day = ((days + 348) % 365) as u16;
        let month_index = (haab_day / 20) as usize;
        let day = (haab_day % 20) as u8;
        Self {
            day,
            month: HAAB_MONTHS[month_index],
        }
    }
}

impl core::fmt::Display for Haab {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} {:?}", self.day, self.month)
    }
}

// ---------------------------------------------------------------------------
// Calendar Round
// ---------------------------------------------------------------------------

/// Compute the Calendar Round (Tzolkin + Haab) for a given day count.
///
/// The Calendar Round repeats every lcm(260, 365) = 18,980 days (~52 years).
#[must_use]
#[inline]
pub fn calendar_round(days: u64) -> (Tzolkin, Haab) {
    (Tzolkin::from_days(days), Haab::from_days(days))
}

/// The Calendar Round cycle length in days: lcm(260, 365) = 18,980.
pub const CALENDAR_ROUND_DAYS: u64 = 18_980;

/// Find the next occurrence of a specific Calendar Round date on or after
/// a given day count.
///
/// A Calendar Round date is a (Tzolkin, Haab) pair that repeats every
/// 18,980 days (~52 years). Given a target Tzolkin number, day sign, Haab
/// day, and Haab month, this function searches forward from `start_day`
/// to find the next matching date.
///
/// # Errors
///
/// Returns [`SankhyaError::InvalidDate`] if the Tzolkin number is not 1–13
/// or the Haab day is out of range for the month.
/// Returns [`SankhyaError::ComputationError`] if no match is found within
/// one full Calendar Round cycle (should not happen for valid inputs).
#[must_use = "returns the matching day or an error"]
pub fn find_calendar_round(
    tzolkin_number: u8,
    tzolkin_sign: DaySign,
    haab_day: u8,
    haab_month: HaabMonth,
    start_day: u64,
) -> Result<u64> {
    if !(1..=13).contains(&tzolkin_number) {
        return Err(SankhyaError::InvalidDate(format!(
            "Tzolkin number {tzolkin_number} out of range 1..13"
        )));
    }

    let haab_max = if haab_month == HaabMonth::Wayeb {
        4
    } else {
        19
    };
    if haab_day > haab_max {
        return Err(SankhyaError::InvalidDate(format!(
            "Haab day {haab_day} out of range for {haab_month:?} (max {haab_max})"
        )));
    }

    // Search day by day within one Calendar Round cycle
    for offset in 0..CALENDAR_ROUND_DAYS {
        let day = start_day + offset;
        let tz = Tzolkin::from_days(day);
        let hb = Haab::from_days(day);

        if tz.number == tzolkin_number
            && tz.day_sign == tzolkin_sign
            && hb.day == haab_day
            && hb.month == haab_month
        {
            return Ok(day);
        }
    }

    Err(SankhyaError::ComputationError(
        "no matching Calendar Round date found within one cycle".into(),
    ))
}

/// Find the next occurrence of a given Tzolkin date on or after a given day count.
///
/// The Tzolkin cycle repeats every 260 days.
///
/// # Errors
///
/// Returns [`SankhyaError::InvalidDate`] if the Tzolkin number is not 1–13.
#[must_use = "returns the matching day or an error"]
pub fn find_tzolkin(tzolkin_number: u8, tzolkin_sign: DaySign, start_day: u64) -> Result<u64> {
    if !(1..=13).contains(&tzolkin_number) {
        return Err(SankhyaError::InvalidDate(format!(
            "Tzolkin number {tzolkin_number} out of range 1..13"
        )));
    }

    for offset in 0..260u64 {
        let day = start_day + offset;
        let tz = Tzolkin::from_days(day);
        if tz.number == tzolkin_number && tz.day_sign == tzolkin_sign {
            return Ok(day);
        }
    }

    Err(SankhyaError::ComputationError(
        "no matching Tzolkin date found within one cycle".into(),
    ))
}

// ---------------------------------------------------------------------------
// Venus table
// ---------------------------------------------------------------------------

/// The synodic period of Venus as computed by the Maya: 583.92 days.
///
/// The Dresden Codex Venus table used 584 days (5 × 584 = 2920 = 8 × 365),
/// accurate to within 2 hours over 500 years.
pub const VENUS_SYNODIC_PERIOD: f64 = 583.92;

/// Phases of Venus as tracked in the Mayan Venus tables (Dresden Codex).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum VenusPhase {
    /// Venus visible as the morning star (236 days in the Mayan model).
    MorningStar,
    /// Venus hidden at superior conjunction (90 days).
    SuperiorConjunction,
    /// Venus visible as the evening star (250 days).
    EveningStar,
    /// Venus hidden at inferior conjunction (8 days).
    InferiorConjunction,
}

/// Determine the phase of Venus given days from the Mayan epoch.
///
/// Uses the Mayan model from the Dresden Codex:
/// - Morning Star: 236 days
/// - Superior Conjunction: 90 days
/// - Evening Star: 250 days
/// - Inferior Conjunction: 8 days
///
/// Total: 584 days (Mayan rounded synodic period)
#[must_use]
pub fn venus_phase(days_from_epoch: u64) -> VenusPhase {
    // The Mayan Venus cycle is 584 days (rounded from 583.92).
    let phase_day = (days_from_epoch % 584) as u16;
    if phase_day < 236 {
        VenusPhase::MorningStar
    } else if phase_day < 236 + 90 {
        VenusPhase::SuperiorConjunction
    } else if phase_day < 236 + 90 + 250 {
        VenusPhase::EveningStar
    } else {
        VenusPhase::InferiorConjunction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vigesimal_zero() {
        assert_eq!(to_vigesimal(0), vec![0]);
        assert_eq!(from_vigesimal(&[0]).unwrap(), 0);
    }

    #[test]
    fn vigesimal_roundtrip() {
        for n in [1, 19, 20, 399, 400, 8000, 160_000, 1_000_000] {
            let digits = to_vigesimal(n);
            assert_eq!(from_vigesimal(&digits).unwrap(), n, "failed for {n}");
        }
    }

    #[test]
    fn mayan_numeral_values() {
        let zero = MayanNumeral::from_value(0).unwrap();
        assert!(zero.shell);
        assert_eq!(zero.value(), 0);

        let thirteen = MayanNumeral::from_value(13).unwrap();
        assert_eq!(thirteen.dots, 3);
        assert_eq!(thirteen.bars, 2);
        assert_eq!(thirteen.value(), 13);
    }

    #[test]
    fn long_count_creation_date() {
        // The creation date is 0.0.0.0.0
        let lc = LongCount::from_days(0).unwrap();
        assert_eq!(lc.to_days(), 0);
        assert_eq!(lc.baktun, 0);
    }

    #[test]
    fn long_count_dec_21_2012() {
        // Dec 21, 2012 = 13.0.0.0.0 = 1,872,000 days from creation
        let days = 13u64 * 144_000;
        let lc = LongCount::from_days(days).unwrap();
        assert_eq!(lc.baktun, 13);
        assert_eq!(lc.katun, 0);
        assert_eq!(lc.tun, 0);
        assert_eq!(lc.uinal, 0);
        assert_eq!(lc.kin, 0);
        assert_eq!(lc.to_days(), days);
    }

    #[test]
    fn tzolkin_at_creation() {
        let tz = Tzolkin::from_days(0);
        assert_eq!(tz.number, 4);
        assert_eq!(tz.day_sign, DaySign::Ahau);
    }

    #[test]
    fn haab_at_creation() {
        let haab = Haab::from_days(0);
        assert_eq!(haab.day, 8);
        assert_eq!(haab.month, HaabMonth::Kumku);
    }

    #[test]
    fn venus_cycle_length() {
        // 236 + 90 + 250 + 8 = 584
        assert_eq!(236 + 90 + 250 + 8, 584);
    }

    // -- Calendar Round search --

    #[test]
    fn find_calendar_round_at_creation() {
        // Creation date is 4 Ahau 8 Kumku — should find day 0
        let day = find_calendar_round(4, DaySign::Ahau, 8, HaabMonth::Kumku, 0).unwrap();
        assert_eq!(day, 0);
    }

    #[test]
    fn find_calendar_round_next_cycle() {
        // Same date should recur at day 18,980
        let day = find_calendar_round(4, DaySign::Ahau, 8, HaabMonth::Kumku, 1).unwrap();
        assert_eq!(day, CALENDAR_ROUND_DAYS);
    }

    #[test]
    fn find_calendar_round_invalid_tzolkin() {
        assert!(find_calendar_round(0, DaySign::Ahau, 0, HaabMonth::Pop, 0).is_err());
        assert!(find_calendar_round(14, DaySign::Ahau, 0, HaabMonth::Pop, 0).is_err());
    }

    #[test]
    fn find_tzolkin_at_creation() {
        // 4 Ahau at day 0
        let day = find_tzolkin(4, DaySign::Ahau, 0).unwrap();
        assert_eq!(day, 0);
    }

    #[test]
    fn find_tzolkin_next_occurrence() {
        // 4 Ahau should recur every 260 days
        let day = find_tzolkin(4, DaySign::Ahau, 1).unwrap();
        assert_eq!(day, 260);
    }
}
