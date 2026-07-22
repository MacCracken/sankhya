//! Aztec (Mexica) calendar systems.
//!
//! Implements the Tonalpohualli (260-day sacred cycle) and Xiuhpohualli
//! (365-day solar cycle), the two interlocking calendar systems of the
//! Aztec civilization. Structurally parallel to the Mayan Tzolkin and Haab.
//!
//! # Historical Context
//!
//! The Aztec calendar system was shared across Mesoamerican cultures
//! (Mixtec, Zapotec, and others) with local variations in day-sign names.
//! The Tonalpohualli ("count of days") combined 13 numbers with 20 day
//! signs to produce a 260-day ritual cycle used for divination and
//! religious scheduling. The Xiuhpohualli ("year count") had 18 months
//! of 20 days plus 5 nemontemi ("empty/unlucky days").
//!
//! The two cycles meshed into a Calendar Round of 52 Xiuhpohualli years
//! (18,980 days), identical in length to the Mayan Calendar Round.
//! The New Fire Ceremony (Xiuhmolpilli) marked the completion of each
//! 52-year cycle.
//!
//! Correlation: this implementation uses the Caso correlation, which
//! places the fall of Tenochtitlan (August 13, 1521 CE) on the Aztec
//! date 1 Coatl in the Tonalpohualli.
//!
//! # Sources
//!
//! - Caso, *Los Calendarios Prehispánicos* (UNAM, 1967) — Caso correlation
//! - Day sign names and order: Sahagún, *Historia General de las Cosas de
//!   Nueva España* (Florentine Codex, 16th century)
//! - Month names and structure: Townsend, *The Aztecs* (3rd ed., Thames &
//!   Hudson, 2009), ch. 14
//! - Calendar Round mechanics: Aveni, *Skywatchers of Ancient Mexico*
//!   (University of Texas Press, 2001)

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Tonalpohualli (260-day sacred cycle)
// ---------------------------------------------------------------------------

/// The 20 day signs of the Tonalpohualli.
///
/// Each day sign (in Nahuatl) has an associated deity, direction, and
/// divinatory meaning. The signs cycle independently of the 13-number
/// trecena to produce the 260-day sacred count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum TonalpohualliSign {
    /// Cipactli (Crocodile / Caiman)
    Cipactli,
    /// Ehecatl (Wind)
    Ehecatl,
    /// Calli (House)
    Calli,
    /// Cuetzpalin (Lizard)
    Cuetzpalin,
    /// Coatl (Serpent)
    Coatl,
    /// Miquiztli (Death)
    Miquiztli,
    /// Mazatl (Deer)
    Mazatl,
    /// Tochtli (Rabbit)
    Tochtli,
    /// Atl (Water)
    Atl,
    /// Itzcuintli (Dog)
    Itzcuintli,
    /// Ozomatli (Monkey)
    Ozomatli,
    /// Malinalli (Grass / Dead Grass)
    Malinalli,
    /// Acatl (Reed)
    Acatl,
    /// Ocelotl (Jaguar)
    Ocelotl,
    /// Cuauhtli (Eagle)
    Cuauhtli,
    /// Cozcacuauhtli (Vulture)
    Cozcacuauhtli,
    /// Ollin (Movement / Earthquake)
    Ollin,
    /// Tecpatl (Flint Knife)
    Tecpatl,
    /// Quiahuitl (Rain)
    Quiahuitl,
    /// Xochitl (Flower)
    Xochitl,
}

const TONALPOHUALLI_SIGNS: [TonalpohualliSign; 20] = [
    TonalpohualliSign::Cipactli,
    TonalpohualliSign::Ehecatl,
    TonalpohualliSign::Calli,
    TonalpohualliSign::Cuetzpalin,
    TonalpohualliSign::Coatl,
    TonalpohualliSign::Miquiztli,
    TonalpohualliSign::Mazatl,
    TonalpohualliSign::Tochtli,
    TonalpohualliSign::Atl,
    TonalpohualliSign::Itzcuintli,
    TonalpohualliSign::Ozomatli,
    TonalpohualliSign::Malinalli,
    TonalpohualliSign::Acatl,
    TonalpohualliSign::Ocelotl,
    TonalpohualliSign::Cuauhtli,
    TonalpohualliSign::Cozcacuauhtli,
    TonalpohualliSign::Ollin,
    TonalpohualliSign::Tecpatl,
    TonalpohualliSign::Quiahuitl,
    TonalpohualliSign::Xochitl,
];

impl core::fmt::Display for TonalpohualliSign {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = match self {
            Self::Cipactli => "Cipactli",
            Self::Ehecatl => "Ehecatl",
            Self::Calli => "Calli",
            Self::Cuetzpalin => "Cuetzpalin",
            Self::Coatl => "Coatl",
            Self::Miquiztli => "Miquiztli",
            Self::Mazatl => "Mazatl",
            Self::Tochtli => "Tochtli",
            Self::Atl => "Atl",
            Self::Itzcuintli => "Itzcuintli",
            Self::Ozomatli => "Ozomatli",
            Self::Malinalli => "Malinalli",
            Self::Acatl => "Acatl",
            Self::Ocelotl => "Ocelotl",
            Self::Cuauhtli => "Cuauhtli",
            Self::Cozcacuauhtli => "Cozcacuauhtli",
            Self::Ollin => "Ollin",
            Self::Tecpatl => "Tecpatl",
            Self::Quiahuitl => "Quiahuitl",
            Self::Xochitl => "Xochitl",
        };
        write!(f, "{name}")
    }
}

/// A Tonalpohualli date (sacred 260-day cycle).
///
/// Composed of a number (1–13) cycling with one of 20 day signs.
/// The full cycle is lcm(13, 20) = 260 days.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tonalpohualli {
    /// The trecena number (1–13).
    pub number: u8,
    /// The day sign (one of 20).
    pub sign: TonalpohualliSign,
}

impl core::fmt::Display for Tonalpohualli {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} {}", self.number, self.sign)
    }
}

// ---------------------------------------------------------------------------
// Xiuhpohualli (365-day solar cycle)
// ---------------------------------------------------------------------------

/// The 19 months of the Xiuhpohualli (18 named months + Nemontemi).
///
/// Each named month has 20 days (0–19). Nemontemi has 5 "empty" days
/// considered unlucky.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum XiuhpohuallMonth {
    /// Atlcahualo (month 1, "Ceasing of Water")
    Atlcahualo,
    /// Tlacaxipehualiztli (month 2, "Flaying of Men")
    Tlacaxipehualiztli,
    /// Tozoztontli (month 3, "Small Vigil")
    Tozoztontli,
    /// Huey Tozoztli (month 4, "Great Vigil")
    HueyTozoztli,
    /// Toxcatl (month 5, "Dryness")
    Toxcatl,
    /// Etzalcualiztli (month 6, "Eating of Maize and Beans")
    Etzalcualiztli,
    /// Tecuilhuitontli (month 7, "Small Feast of the Lords")
    Tecuilhuitontli,
    /// Huey Tecuilhuitl (month 8, "Great Feast of the Lords")
    HueyTecuilhuitl,
    /// Tlaxochimaco (month 9, "Offering of Flowers")
    Tlaxochimaco,
    /// Xocotl Huetzi (month 10, "Fall of Xocotl")
    XocotlHuetzi,
    /// Ochpaniztli (month 11, "Sweeping")
    Ochpaniztli,
    /// Teotleco (month 12, "Arrival of the Gods")
    Teotleco,
    /// Tepeilhuitl (month 13, "Feast of the Mountains")
    Tepeilhuitl,
    /// Quecholli (month 14, "Precious Feather")
    Quecholli,
    /// Panquetzaliztli (month 15, "Raising of Banners")
    Panquetzaliztli,
    /// Atemoztli (month 16, "Descent of Water")
    Atemoztli,
    /// Tititl (month 17, "Stretching")
    Tititl,
    /// Izcalli (month 18, "Growth")
    Izcalli,
    /// Nemontemi (5 unlucky days)
    Nemontemi,
}

const XIUHPOHUALL_MONTHS: [XiuhpohuallMonth; 19] = [
    XiuhpohuallMonth::Atlcahualo,
    XiuhpohuallMonth::Tlacaxipehualiztli,
    XiuhpohuallMonth::Tozoztontli,
    XiuhpohuallMonth::HueyTozoztli,
    XiuhpohuallMonth::Toxcatl,
    XiuhpohuallMonth::Etzalcualiztli,
    XiuhpohuallMonth::Tecuilhuitontli,
    XiuhpohuallMonth::HueyTecuilhuitl,
    XiuhpohuallMonth::Tlaxochimaco,
    XiuhpohuallMonth::XocotlHuetzi,
    XiuhpohuallMonth::Ochpaniztli,
    XiuhpohuallMonth::Teotleco,
    XiuhpohuallMonth::Tepeilhuitl,
    XiuhpohuallMonth::Quecholli,
    XiuhpohuallMonth::Panquetzaliztli,
    XiuhpohuallMonth::Atemoztli,
    XiuhpohuallMonth::Tititl,
    XiuhpohuallMonth::Izcalli,
    XiuhpohuallMonth::Nemontemi,
];

impl core::fmt::Display for XiuhpohuallMonth {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = match self {
            Self::Atlcahualo => "Atlcahualo",
            Self::Tlacaxipehualiztli => "Tlacaxipehualiztli",
            Self::Tozoztontli => "Tozoztontli",
            Self::HueyTozoztli => "Huey Tozoztli",
            Self::Toxcatl => "Toxcatl",
            Self::Etzalcualiztli => "Etzalcualiztli",
            Self::Tecuilhuitontli => "Tecuilhuitontli",
            Self::HueyTecuilhuitl => "Huey Tecuilhuitl",
            Self::Tlaxochimaco => "Tlaxochimaco",
            Self::XocotlHuetzi => "Xocotl Huetzi",
            Self::Ochpaniztli => "Ochpaniztli",
            Self::Teotleco => "Teotleco",
            Self::Tepeilhuitl => "Tepeilhuitl",
            Self::Quecholli => "Quecholli",
            Self::Panquetzaliztli => "Panquetzaliztli",
            Self::Atemoztli => "Atemoztli",
            Self::Tititl => "Tititl",
            Self::Izcalli => "Izcalli",
            Self::Nemontemi => "Nemontemi",
        };
        write!(f, "{name}")
    }
}

/// A Xiuhpohualli date (solar 365-day cycle).
///
/// 18 months of 20 days each (1–20), plus Nemontemi (1–5 unlucky days).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Xiuhpohualli {
    /// Day within the month (1–20 for regular months, 1–5 for Nemontemi).
    pub day: u8,
    /// The Xiuhpohualli month.
    pub month: XiuhpohuallMonth,
}

impl core::fmt::Display for Xiuhpohualli {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} {}", self.day, self.month)
    }
}

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Correlation constant: JDN of the Aztec reference date.
///
/// Uses the Caso correlation: the fall of Tenochtitlan on August 13, 1521 CE
/// (JDN 2275856) corresponds to Tonalpohualli date 1 Coatl. This gives
/// an offset for computing Tonalpohualli from JDN.
///
/// For the Xiuhpohualli, the same correlation places August 13, 1521 as
/// day 1 of Xocotl Huetzi (month 10).
const AZTEC_CORRELATION_JDN: i64 = 2_275_856;

/// Tonalpohualli number (1-based) at the correlation date.
const CORRELATION_TONAL_NUMBER: i64 = 1;

/// Tonalpohualli sign index (0-based) at the correlation date.
/// Coatl = index 4.
const CORRELATION_TONAL_SIGN: i64 = 4;

/// Xiuhpohualli day-of-year (0-based) at the correlation date.
/// Day 1 of Xocotl Huetzi (month 10, 0-indexed month 9) = 9*20 + 0 = 180.
const CORRELATION_XIUH_DOY: i64 = 180;

/// The Calendar Round cycle length in days: lcm(260, 365) = 18,980.
pub const AZTEC_CALENDAR_ROUND_DAYS: u64 = 18_980;

// ---------------------------------------------------------------------------
// Conversion functions
// ---------------------------------------------------------------------------

/// Compute the Tonalpohualli (260-day) date for a given Julian Day Number.
#[must_use]
pub fn tonalpohualli_from_jdn(jdn: f64) -> Tonalpohualli {
    let jdn_int = (jdn + 0.5).floor() as i64;
    let days = jdn_int - AZTEC_CORRELATION_JDN;

    // Number cycles 1–13
    let number = ((days + CORRELATION_TONAL_NUMBER - 1).rem_euclid(13) + 1) as u8;

    // Sign cycles through 20
    let sign_idx = (days + CORRELATION_TONAL_SIGN).rem_euclid(20) as usize;

    Tonalpohualli {
        number,
        sign: TONALPOHUALLI_SIGNS[sign_idx],
    }
}

/// Compute the Xiuhpohualli (365-day) date for a given Julian Day Number.
#[must_use]
pub fn xiuhpohualli_from_jdn(jdn: f64) -> Xiuhpohualli {
    let jdn_int = (jdn + 0.5).floor() as i64;
    let days = jdn_int - AZTEC_CORRELATION_JDN;

    let doy = (days + CORRELATION_XIUH_DOY).rem_euclid(365);

    let month_idx = (doy / 20).min(18) as usize;
    let day = (doy - (month_idx as i64) * 20 + 1) as u8;

    Xiuhpohualli {
        day,
        month: XIUHPOHUALL_MONTHS[month_idx],
    }
}

/// Compute the Aztec Calendar Round (Tonalpohualli + Xiuhpohualli) for a
/// given Julian Day Number.
///
/// The Calendar Round repeats every lcm(260, 365) = 18,980 days (~52 years).
/// The New Fire Ceremony (Xiuhmolpilli) marked each cycle's completion.
#[must_use]
#[inline]
pub fn aztec_calendar_round(jdn: f64) -> (Tonalpohualli, Xiuhpohualli) {
    (tonalpohualli_from_jdn(jdn), xiuhpohualli_from_jdn(jdn))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correlation_date_tonalpohualli() {
        // August 13, 1521 = JDN 2275856 = 1 Coatl
        let t = tonalpohualli_from_jdn(2_275_855.5);
        assert_eq!(t.number, 1);
        assert_eq!(t.sign, TonalpohualliSign::Coatl);
    }

    #[test]
    fn correlation_date_xiuhpohualli() {
        // August 13, 1521 = day 1 of Xocotl Huetzi
        let x = xiuhpohualli_from_jdn(2_275_855.5);
        assert_eq!(x.month, XiuhpohuallMonth::XocotlHuetzi);
        assert_eq!(x.day, 1);
    }

    #[test]
    fn tonalpohualli_260_day_cycle() {
        // Same Tonalpohualli date should repeat every 260 days
        let t1 = tonalpohualli_from_jdn(2_275_855.5);
        let t2 = tonalpohualli_from_jdn(2_275_855.5 + 260.0);
        assert_eq!(t1, t2);
    }

    #[test]
    fn xiuhpohualli_365_day_cycle() {
        // Same Xiuhpohualli date should repeat every 365 days
        let x1 = xiuhpohualli_from_jdn(2_275_855.5);
        let x2 = xiuhpohualli_from_jdn(2_275_855.5 + 365.0);
        assert_eq!(x1, x2);
    }

    #[test]
    fn calendar_round_52_year_cycle() {
        // Calendar Round repeats every 18,980 days
        let (t1, x1) = aztec_calendar_round(2_275_855.5);
        let (t2, x2) = aztec_calendar_round(2_275_855.5 + 18_980.0);
        assert_eq!(t1, t2);
        assert_eq!(x1, x2);
    }

    #[test]
    fn tonalpohualli_all_signs_cycle() {
        // After 20 consecutive days, all signs should appear
        let mut signs = std::collections::HashSet::new();
        for i in 0..20 {
            let t = tonalpohualli_from_jdn(2_275_855.5 + f64::from(i));
            signs.insert(format!("{:?}", t.sign));
        }
        assert_eq!(signs.len(), 20);
    }

    #[test]
    fn tonalpohualli_number_range() {
        // Numbers should always be 1–13
        for i in 0..260 {
            let t = tonalpohualli_from_jdn(2_275_855.5 + f64::from(i));
            assert!(
                t.number >= 1 && t.number <= 13,
                "number {} out of range",
                t.number
            );
        }
    }

    #[test]
    fn xiuhpohualli_nemontemi() {
        // Nemontemi has 5 days; find when it occurs
        let mut nemontemi_count = 0;
        for i in 0..365 {
            let x = xiuhpohualli_from_jdn(2_275_855.5 + f64::from(i));
            if x.month == XiuhpohuallMonth::Nemontemi {
                nemontemi_count += 1;
                assert!(x.day >= 1 && x.day <= 5);
            }
        }
        assert_eq!(nemontemi_count, 5);
    }

    #[test]
    fn xiuhpohualli_month_days() {
        // 18 months of 20 days + 5 Nemontemi = 365
        let mut total = 0u32;
        for i in 0..365 {
            let x = xiuhpohualli_from_jdn(2_275_855.5 + f64::from(i));
            if x.day == 1 {
                total += 1; // count month starts
            }
        }
        assert_eq!(total, 19); // 18 regular + 1 Nemontemi
    }

    #[test]
    fn display_tonalpohualli() {
        let t = Tonalpohualli {
            number: 1,
            sign: TonalpohualliSign::Coatl,
        };
        assert_eq!(t.to_string(), "1 Coatl");
    }

    #[test]
    fn display_xiuhpohualli() {
        let x = Xiuhpohualli {
            day: 1,
            month: XiuhpohuallMonth::XocotlHuetzi,
        };
        assert_eq!(x.to_string(), "1 Xocotl Huetzi");
    }

    #[test]
    fn serde_roundtrip_tonalpohualli() {
        let t = tonalpohualli_from_jdn(2_275_855.5);
        let json = serde_json::to_string(&t).unwrap();
        let back: Tonalpohualli = serde_json::from_str(&json).unwrap();
        assert_eq!(t, back);
    }

    #[test]
    fn serde_roundtrip_xiuhpohualli() {
        let x = xiuhpohualli_from_jdn(2_275_855.5);
        let json = serde_json::to_string(&x).unwrap();
        let back: Xiuhpohualli = serde_json::from_str(&json).unwrap();
        assert_eq!(x, back);
    }

    #[test]
    fn sign_display() {
        assert_eq!(TonalpohualliSign::Cipactli.to_string(), "Cipactli");
        assert_eq!(TonalpohualliSign::Xochitl.to_string(), "Xochitl");
        assert_eq!(TonalpohualliSign::Ocelotl.to_string(), "Ocelotl");
    }

    #[test]
    fn month_display() {
        assert_eq!(XiuhpohuallMonth::Atlcahualo.to_string(), "Atlcahualo");
        assert_eq!(XiuhpohuallMonth::Nemontemi.to_string(), "Nemontemi");
        assert_eq!(
            XiuhpohuallMonth::Tlacaxipehualiztli.to_string(),
            "Tlacaxipehualiztli"
        );
    }
}
