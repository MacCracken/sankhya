//! Cross-civilizational epoch correlation.
//!
//! Correlates epochs and astronomical cycles across all six civilizations
//! in sankhya, using the precession of the equinoxes as the master clock
//! and the Younger Dryas boundary (~12,800 BP) as the anchor point.
//!
//! # Historical Context
//!
//! Every civilization sankhya covers preserves a tradition of Seven Sages
//! who survived a catastrophic flood and restarted civilization: the Vedic
//! Saptarishi, Babylonian Apkallu, Egyptian Shemsu Hor, Mayan Popol Vuh
//! creators, Greek Deucalion tradition, and Chinese Fuxi/Nuwa. These
//! traditions converge on the Younger Dryas boundary (c. 10,800 BCE),
//! which falls in the Age of Leo according to the precessional cycle.
//!
//! The precession of the equinoxes — the ~25,920-year "Great Year" — is
//! encoded across civilizations: Babylonian 72 years per degree, Egyptian
//! 36 decans × 720 years, Vedic half-Deva-Yuga. This module uses the
//! canonical ancient value (25,920 years) rather than the modern
//! astronomical value (~25,772 years), as the purpose is to model what
//! the ancients encoded.
//!
//! # Sources
//!
//! - Precession rate (1°/72 years): Hipparchus (c. 130 BCE), transmitted
//!   via Ptolemy, *Almagest* VII.2. Modern value: ~1°/71.6 years
//! - Younger Dryas Impact Hypothesis: Firestone et al., "Evidence for an
//!   extraterrestrial impact 12,900 years ago", *PNAS* 104(41), 2007
//! - Seven Sages traditions: Hancock, *Fingerprints of the Gods* (1995);
//!   primary sources cited per tradition in [`seven_sages()`]
//! - J2000.0 reference epoch: IAU 1976 System of Astronomical Constants
//! - Radiocarbon BP reference (1950 CE): Stuiver & Polach, *Radiocarbon* 19(3), 1977
//! - Calendar conversion algorithms: Dershowitz & Reingold, *Calendrical
//!   Calculations* (4th ed., 2018)

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::error::{Result, SankhyaError};

// ---------------------------------------------------------------------------
// Constants: Precession of the equinoxes (the Great Year)
// ---------------------------------------------------------------------------

/// The Great Year: canonical ancient period of axial precession in Julian years.
///
/// The Earth's rotational axis traces a cone through the celestial sphere,
/// completing one full cycle in approximately 25,920 years. This number
/// appears encoded across civilizations:
/// - Babylonian: 72 years per degree (72 × 360 = 25,920)
/// - Hindu: twelve precessional ages of 2,160 years
/// - Egyptian: 36 decans × 720 years = 25,920
///
/// Modern astronomy gives ~25,772 years. The canonical 25,920 is used here
/// for its clean factorization (72 × 360 = 2,160 × 12) and because it
/// models the ancient encoding, not modern observation.
pub const GREAT_YEAR_YEARS: f64 = 25_920.0;

/// Days in one Great Year (25,920 × 365.25).
pub const GREAT_YEAR_DAYS: f64 = 25_920.0 * 365.25;

/// Annual precession rate in degrees per Julian year.
/// 360° / 25,920 years = 1/72 degree per year.
pub const PRECESSION_RATE_DEG_PER_YEAR: f64 = 1.0 / 72.0;

/// Duration of one precessional age in years (one twelfth of the Great Year).
/// 25,920 / 12 = 2,160 years per age.
pub const PRECESSIONAL_AGE_YEARS: f64 = 2_160.0;

/// Duration of one precessional age in days.
pub const PRECESSIONAL_AGE_DAYS: f64 = 2_160.0 * 365.25;

// ---------------------------------------------------------------------------
// Constants: Younger Dryas boundary
// ---------------------------------------------------------------------------

/// Julian Day Number for the Younger Dryas boundary (~10,800 BCE).
///
/// The Younger Dryas onset marks a catastrophic climate reversal at the end
/// of the last Ice Age. The Younger Dryas Impact Hypothesis (Firestone et al.
/// 2007) proposes a cometary airburst as the trigger. This date falls in the
/// Age of Leo, the epoch of the Seven Sages across all traditions.
///
/// Computed: J2000.0 (JDN 2,451,545) − 12,800 Julian years.
pub const YOUNGER_DRYAS_JDN: f64 = 2_451_545.0 - 12_800.0 * 365.25;

/// The Younger Dryas boundary in years Before Present (BP, Present = 1950 CE).
pub const YOUNGER_DRYAS_BP: f64 = 12_800.0;

// ---------------------------------------------------------------------------
// Constants: Reference epochs
// ---------------------------------------------------------------------------

/// J2000.0 reference epoch (January 1, 2000, 12:00 TT).
pub const J2000_JDN: f64 = 2_451_545.0;

/// Julian year in days.
pub const JULIAN_YEAR_DAYS: f64 = 365.25;

/// JDN for January 0.5, 1950 CE — the radiocarbon BP reference point.
pub const BP_REFERENCE_JDN: f64 = 2_433_282.5;

/// Mean synodic month in days (used by Metonic and Saros cycles).
pub const SYNODIC_MONTH_DAYS: f64 = 29.530_588_86;

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

/// The twelve precessional ages, corresponding to zodiacal constellations
/// through which the vernal equinox precesses in reverse order.
///
/// The Age of Leo is anchored to the Younger Dryas boundary (~10,800 BCE).
/// Ages proceed westward through the ecliptic as the vernal point precesses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PrecessionalAge {
    /// Age of Leo (~10,800 BCE – ~8,640 BCE) — the Younger Dryas epoch
    Leo,
    /// Age of Cancer (~8,640 BCE – ~6,480 BCE)
    Cancer,
    /// Age of Gemini (~6,480 BCE – ~4,320 BCE)
    Gemini,
    /// Age of Taurus (~4,320 BCE – ~2,160 BCE)
    Taurus,
    /// Age of Aries (~2,160 BCE – ~0 CE)
    Aries,
    /// Age of Pisces (~0 CE – ~2,160 CE)
    Pisces,
    /// Age of Aquarius (~2,160 CE – ~4,320 CE)
    Aquarius,
    /// Age of Capricorn
    Capricorn,
    /// Age of Sagittarius
    Sagittarius,
    /// Age of Scorpio
    Scorpio,
    /// Age of Libra
    Libra,
    /// Age of Virgo
    Virgo,
}

/// The twelve ages in order, starting from Leo (index 0 at the Younger Dryas).
const AGE_ORDER: [PrecessionalAge; 12] = [
    PrecessionalAge::Leo,
    PrecessionalAge::Cancer,
    PrecessionalAge::Gemini,
    PrecessionalAge::Taurus,
    PrecessionalAge::Aries,
    PrecessionalAge::Pisces,
    PrecessionalAge::Aquarius,
    PrecessionalAge::Capricorn,
    PrecessionalAge::Sagittarius,
    PrecessionalAge::Scorpio,
    PrecessionalAge::Libra,
    PrecessionalAge::Virgo,
];

/// Civilizations covered by the Seven Sages tradition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Civilization {
    /// Vedic / Indian
    Vedic,
    /// Babylonian / Mesopotamian
    Babylonian,
    /// Egyptian
    Egyptian,
    /// Mayan / Mesoamerican
    Mayan,
    /// Greek
    Greek,
    /// Chinese
    Chinese,
}

/// Named astronomical cycles that can be queried and compared.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CycleName {
    /// Precession of the equinoxes (~25,920 years)
    Precession,
    /// Egyptian Sothic cycle (1,461 civil years)
    Sothic,
    /// Babylonian Saros eclipse cycle (~18.03 years)
    Saros,
    /// Mayan Venus synodic period (~583.92 days)
    VenusSynodic,
    /// Mayan Calendar Round (18,980 days)
    CalendarRound,
    /// Greek Metonic cycle (19 years = 235 synodic months)
    Metonic,
    /// One precessional age (2,160 years)
    PrecessionalAge,
}

// ---------------------------------------------------------------------------
// Structs
// ---------------------------------------------------------------------------

/// A cross-cultural Seven Sages tradition record.
///
/// Each civilization preserves a memory of primordial sages who survived
/// a catastrophic flood and restarted civilization. This struct captures
/// the common structure across traditions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SagesTradition {
    /// The civilization that preserves this tradition.
    pub civilization: Civilization,
    /// Name of the sage group (e.g., "Saptarishi", "Apkallu").
    pub group_name: Cow<'static, str>,
    /// Individual sage names, where known from the textual tradition.
    pub sage_names: Vec<Cow<'static, str>>,
    /// Primary source text(s).
    pub source_texts: Vec<Cow<'static, str>>,
    /// Associated star or constellation.
    pub associated_stars: Cow<'static, str>,
    /// Brief description of the flood/catastrophe narrative.
    pub catastrophe_narrative: Cow<'static, str>,
}

/// Result of a precessional age calculation.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AgePosition {
    /// The current precessional age.
    pub age: PrecessionalAge,
    /// Years elapsed within this age (0.0 to ~2,160.0).
    pub years_into_age: f64,
    /// Fraction of the current age elapsed (0.0 to 1.0).
    pub fraction: f64,
    /// Ecliptic longitude of the vernal equinox point in degrees (0–360).
    pub vernal_point_longitude: f64,
}

/// A point in time expressed simultaneously in multiple calendar systems.
///
/// All fields derived from a single JDN anchor. This is the "Rosetta Stone"
/// of sankhya — one date, every system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultiCalendarDate {
    /// The Julian Day Number (the common currency).
    pub jdn: f64,
    /// Mayan Long Count (`None` if before the Mayan epoch).
    pub mayan_long_count: Option<crate::mayan::LongCount>,
    /// Mayan Tzolkin date (`None` if before the Mayan epoch).
    pub tzolkin: Option<crate::mayan::Tzolkin>,
    /// Mayan Haab date (`None` if before the Mayan epoch).
    pub haab: Option<crate::mayan::Haab>,
    /// Egyptian Sothic cycle position: (cycle_number, year_in_cycle, drift_days).
    pub sothic_position: (i32, u32, f64),
    /// Current precessional age and position.
    pub precessional_age: AgePosition,
    /// Approximate Julian calendar year (negative = BCE).
    pub julian_year: f64,
    /// Proleptic Gregorian calendar date.
    pub gregorian: crate::gregorian::GregorianDate,
    /// Coptic (Alexandrian) calendar date.
    pub coptic: crate::coptic::CopticDate,
    /// Persian (Solar Hijri / Jalaali) calendar date.
    pub persian: crate::persian::PersianDate,
    /// Hebrew (Jewish) calendar date.
    pub hebrew: crate::hebrew::HebrewDate,
    /// Chinese sexagenary (60-year) cycle year.
    pub chinese_sexagenary: crate::chinese::SexagenaryYear,
    /// Aztec Tonalpohualli (260-day sacred cycle) date.
    pub aztec_tonalpohualli: crate::aztec::Tonalpohualli,
    /// Aztec Xiuhpohualli (365-day solar cycle) date.
    pub aztec_xiuhpohualli: crate::aztec::Xiuhpohualli,
    /// Historical eras active at this date (requires `itihas` feature).
    #[cfg(feature = "itihas")]
    pub eras: Vec<itihas::era::Era>,
    /// Civilizations active at this date (requires `itihas` feature).
    #[cfg(feature = "itihas")]
    pub civilizations: Vec<itihas::civilization::Civilization>,
    /// Events at this year (requires `itihas` feature).
    #[cfg(feature = "itihas")]
    pub events: Vec<itihas::event::Event>,
    /// Saptarishi archetype profiles from avatara (requires `avatara` feature).
    #[cfg(feature = "avatara")]
    pub saptarishi_profiles: Vec<(String, avatara::ArchetypeProfile)>,
}

/// A cycle alignment event: multiple cycles reaching integer multiples
/// simultaneously within a tolerance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CycleAlignment {
    /// JDN of the alignment.
    pub jdn: f64,
    /// Which cycles align at this point.
    pub cycles: Vec<CycleName>,
    /// Maximum residual error in days across aligned cycles.
    pub max_residual_days: f64,
}

// ---------------------------------------------------------------------------
// Static data: Seven Sages traditions
// ---------------------------------------------------------------------------

/// Build the Seven Sages traditions across all six civilizations.
///
/// Each entry documents a civilization's memory of antediluvian sages
/// who preserved knowledge through the catastrophe and restarted
/// civilization. The tradition is remarkably consistent across cultures
/// that had no known contact at the time of its encoding.
///
/// Returns a fresh `Vec` each call (data uses `Cow::Borrowed` for zero-copy).
#[must_use]
pub fn seven_sages() -> Vec<SagesTradition> {
    use Cow::Borrowed as B;
    vec![
        SagesTradition {
            civilization: Civilization::Vedic,
            group_name: B("Saptarishi"),
            sage_names: vec![
                B("Marichi"),
                B("Atri"),
                B("Angiras"),
                B("Pulastya"),
                B("Pulaha"),
                B("Kratu"),
                B("Vasishtha"),
            ],
            source_texts: vec![
                B("Shatapatha Brahmana"),
                B("Matsya Purana"),
                B("Vishnu Purana"),
            ],
            associated_stars: B("Ursa Major (Saptarishi Mandala)"),
            catastrophe_narrative: B(
                "Survived the pralaya (great dissolution) guided by Matsya (fish avatar of Vishnu); restarted the Vedic tradition after the flood.",
            ),
        },
        SagesTradition {
            civilization: Civilization::Babylonian,
            group_name: B("Apkallu"),
            sage_names: vec![
                B("Adapa (Uanna)"),
                B("Uannedugga"),
                B("Enmedugga"),
                B("Enmegalamma"),
                B("Enmebulugga"),
                B("An-Enlilda"),
                B("Utuabzu"),
            ],
            source_texts: vec![
                B("Bit Meseri incantations"),
                B("Berossus Babyloniaca"),
                B("Eridu Genesis"),
            ],
            associated_stars: B("Associated with Ea/Enki and the Abzu (cosmic waters)"),
            catastrophe_narrative: B(
                "Seven antediluvian sages sent by Ea before the Great Flood; taught the arts of civilization to humanity.",
            ),
        },
        SagesTradition {
            civilization: Civilization::Egyptian,
            group_name: B("Shemsu Hor"),
            sage_names: vec![B(
                "The Seven Builder Gods (individual names partially lost)",
            )],
            source_texts: vec![
                B("Edfu Building Texts"),
                B("Turin King List (Zep Tepi rulers)"),
            ],
            associated_stars: B("Sirius (Sopdet) and Orion (Sah)"),
            catastrophe_narrative: B(
                "The Edfu texts describe seven sages arriving from an island destroyed by flood in Zep Tepi (the First Time), who established the sacred mounds.",
            ),
        },
        SagesTradition {
            civilization: Civilization::Mayan,
            group_name: B("Popol Vuh Creators"),
            sage_names: vec![
                B("Tepeu"),
                B("Gucumatz"),
                B("Huracan"),
                B("Chipi-Caculha"),
                B("Raxa-Caculha"),
                B("Ixpiyacoc"),
                B("Ixmucane"),
            ],
            source_texts: vec![B("Popol Vuh"), B("Chilam Balam")],
            associated_stars: B("Pleiades (Tzab-ek) and Orion"),
            catastrophe_narrative: B(
                "Multiple cycles of creation and destruction; the current (fourth) world was preceded by a great flood that destroyed the wooden people.",
            ),
        },
        SagesTradition {
            civilization: Civilization::Greek,
            group_name: B("Seven Sages of Greece"),
            sage_names: vec![
                B("Thales"),
                B("Solon"),
                B("Chilon"),
                B("Bias"),
                B("Cleobulus"),
                B("Pittacus"),
                B("Periander"),
            ],
            source_texts: vec![
                B("Plato Timaeus"),
                B("Ovid Metamorphoses"),
                B("Diogenes Laertius"),
            ],
            associated_stars: B("Ursa Major (Arktos) and Pleiades"),
            catastrophe_narrative: B(
                "Deucalion and Pyrrha survived the Great Flood sent by Zeus; Plato's Timaeus describes the destruction of Atlantis and cyclical catastrophes.",
            ),
        },
        SagesTradition {
            civilization: Civilization::Chinese,
            group_name: B("Fuxi and Nuwa"),
            sage_names: vec![
                B("Fuxi"),
                B("Nuwa"),
                B("Shennong"),
                B("Huangdi"),
                B("Yao"),
                B("Shun"),
                B("Yu the Great"),
            ],
            source_texts: vec![
                B("Huainanzi"),
                B("Shanhaijing"),
                B("Shujing (Book of Documents)"),
            ],
            associated_stars: B("Beidou (Northern Dipper / Ursa Major)"),
            catastrophe_narrative: B(
                "Fuxi and Nuwa survived the Great Flood; Nuwa repaired the broken sky. Yu the Great later tamed the floodwaters and founded civilization.",
            ),
        },
    ]
}

// ---------------------------------------------------------------------------
// Seven Sages accessors
// ---------------------------------------------------------------------------

/// Return the Seven Sages tradition for a given civilization.
#[must_use]
pub fn sages_tradition(civ: Civilization) -> Option<SagesTradition> {
    seven_sages().into_iter().find(|s| s.civilization == civ)
}

/// Return all Seven Sages traditions.
#[must_use]
pub fn all_sages_traditions() -> Vec<SagesTradition> {
    seven_sages()
}

// ---------------------------------------------------------------------------
// Time conversion utilities
// ---------------------------------------------------------------------------

/// Convert years Before Present (BP, where Present = 1950 CE) to a JDN.
#[must_use]
#[inline]
pub fn bp_to_jdn(years_bp: f64) -> f64 {
    BP_REFERENCE_JDN - years_bp * JULIAN_YEAR_DAYS
}

/// Convert a JDN to years Before Present (BP).
#[must_use]
#[inline]
pub fn jdn_to_bp(jdn: f64) -> f64 {
    (BP_REFERENCE_JDN - jdn) / JULIAN_YEAR_DAYS
}

/// Convert a Julian calendar year to an approximate JDN (January 1 of that year).
///
/// Negative years represent BCE: year −3113 = 3114 BCE.
#[must_use]
#[inline]
pub fn julian_year_to_jdn(year: i64) -> f64 {
    // J2000.0 is Jan 1.5, 2000 CE. Approximate: shift by (year - 2000) * 365.25.
    J2000_JDN + (year as f64 - 2000.0) * JULIAN_YEAR_DAYS
}

/// Convert a JDN to an approximate Julian calendar year.
#[must_use]
#[inline]
pub fn jdn_to_julian_year(jdn: f64) -> f64 {
    2000.0 + (jdn - J2000_JDN) / JULIAN_YEAR_DAYS
}

// ---------------------------------------------------------------------------
// Precession calculations
// ---------------------------------------------------------------------------

/// Compute the ecliptic longitude of the vernal equinox point for a given JDN.
///
/// Models the slow westward drift of the vernal point at the canonical rate
/// of 1 degree per 72 years. Anchored so that at the Younger Dryas boundary,
/// the vernal point is at the start of Leo (150° ecliptic longitude in the
/// equal-sign scheme where Aries = 0°).
///
/// Returns degrees in the range 0.0 to 360.0.
#[must_use]
pub fn vernal_point_longitude(jdn: f64) -> f64 {
    let years_since_yd = (jdn - YOUNGER_DRYAS_JDN) / JULIAN_YEAR_DAYS;
    // At the Younger Dryas, vernal point is at Leo boundary = 150°.
    // Precession moves the vernal point westward (decreasing longitude).
    let longitude = 150.0 - years_since_yd * PRECESSION_RATE_DEG_PER_YEAR;
    ((longitude % 360.0) + 360.0) % 360.0
}

/// Compute the precessional age and position for a given Julian Day Number.
///
/// Uses the canonical ancient precession period of 25,920 years with the
/// Age of Leo beginning at the Younger Dryas boundary (~10,800 BCE).
#[must_use]
pub fn precessional_age(jdn: f64) -> AgePosition {
    let years_since_yd = (jdn - YOUNGER_DRYAS_JDN) / JULIAN_YEAR_DAYS;

    // Normalize to positive cycle position
    let cycle_years = ((years_since_yd % GREAT_YEAR_YEARS) + GREAT_YEAR_YEARS) % GREAT_YEAR_YEARS;
    let age_index = (cycle_years / PRECESSIONAL_AGE_YEARS) as usize;
    let age_index = if age_index >= 12 { 11 } else { age_index };
    let years_into_age = cycle_years - (age_index as f64) * PRECESSIONAL_AGE_YEARS;

    AgePosition {
        age: AGE_ORDER[age_index],
        years_into_age,
        fraction: years_into_age / PRECESSIONAL_AGE_YEARS,
        vernal_point_longitude: vernal_point_longitude(jdn),
    }
}

/// Return the Julian Day Number when a given precessional age begins.
///
/// Returns the start of the most recent occurrence of the age relative
/// to the Younger Dryas anchor.
#[must_use]
pub fn age_start_jdn(age: PrecessionalAge) -> f64 {
    let index = AGE_ORDER.iter().position(|&a| a == age).unwrap_or(0);
    YOUNGER_DRYAS_JDN + (index as f64) * PRECESSIONAL_AGE_DAYS
}

// ---------------------------------------------------------------------------
// Cycle infrastructure
// ---------------------------------------------------------------------------

/// Return the period in days for a named astronomical cycle.
#[must_use]
pub fn cycle_period(cycle: CycleName) -> f64 {
    match cycle {
        CycleName::Precession => GREAT_YEAR_DAYS,
        CycleName::Sothic => crate::egyptian::SOTHIC_CYCLE_DAYS as f64,
        CycleName::Saros => crate::babylonian::SAROS_DAYS,
        CycleName::VenusSynodic => crate::mayan::VENUS_SYNODIC_PERIOD,
        CycleName::CalendarRound => crate::mayan::CALENDAR_ROUND_DAYS as f64,
        CycleName::Metonic => 19.0 * JULIAN_YEAR_DAYS,
        CycleName::PrecessionalAge => PRECESSIONAL_AGE_DAYS,
    }
}

/// Compute the number of complete cycles elapsed since a reference epoch
/// for each named cycle.
///
/// Returns a vector of `(cycle_name, complete_cycles, remainder_days)`.
#[must_use]
pub fn cycles_elapsed(
    jdn: f64,
    reference_jdn: f64,
    cycles: &[CycleName],
) -> Vec<(CycleName, u64, f64)> {
    let elapsed_days = jdn - reference_jdn;
    cycles
        .iter()
        .map(|&cycle| {
            let period = cycle_period(cycle);
            if period <= 0.0 {
                return (cycle, 0, elapsed_days);
            }
            let complete = (elapsed_days / period).floor();
            let remainder = elapsed_days - complete * period;
            let complete_u64 = if complete < 0.0 { 0 } else { complete as u64 };
            (cycle, complete_u64, remainder)
        })
        .collect()
}

/// Find dates when multiple astronomical cycles align within a tolerance.
///
/// Searches from `start_jdn` to `end_jdn`, checking at each step of the
/// shortest cycle whether all cycles are within `tolerance_days` of an
/// integer multiple.
///
/// # Errors
///
/// Returns [`SankhyaError::InvalidDate`] if `start_jdn >= end_jdn`.
/// Returns [`SankhyaError::ComputationError`] if the search exceeds
/// 10,000,000 steps.
#[must_use = "returns the alignments or an error"]
pub fn find_cycle_alignments(
    cycles: &[CycleName],
    start_jdn: f64,
    end_jdn: f64,
    tolerance_days: f64,
) -> Result<Vec<CycleAlignment>> {
    if start_jdn >= end_jdn {
        return Err(SankhyaError::InvalidDate(
            "start_jdn must be before end_jdn".into(),
        ));
    }
    if cycles.len() < 2 {
        return Err(SankhyaError::ComputationError(
            "need at least 2 cycles for alignment search".into(),
        ));
    }

    let periods: Vec<f64> = cycles.iter().map(|&c| cycle_period(c)).collect();
    let step = periods
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min)
        .max(1.0);

    let max_steps = 10_000_000u64;
    let num_steps = ((end_jdn - start_jdn) / step).ceil() as u64;
    if num_steps > max_steps {
        return Err(SankhyaError::ComputationError(format!(
            "search space too large: {num_steps} steps (max {max_steps})"
        )));
    }

    let mut alignments = Vec::new();
    let mut jdn = start_jdn;

    while jdn < end_jdn {
        let mut max_residual = 0.0_f64;
        let mut all_aligned = true;

        for &period in &periods {
            let elapsed = jdn - start_jdn;
            let ratio = elapsed / period;
            let residual = (ratio - ratio.round()).abs() * period;
            if residual > tolerance_days {
                all_aligned = false;
                break;
            }
            max_residual = max_residual.max(residual);
        }

        if all_aligned {
            alignments.push(CycleAlignment {
                jdn,
                cycles: cycles.to_vec(),
                max_residual_days: max_residual,
            });
        }

        jdn += step;
    }

    Ok(alignments)
}

// ---------------------------------------------------------------------------
// Epoch correlation: the Rosetta Stone
// ---------------------------------------------------------------------------

/// Convert a Julian Day Number to a multi-calendar date.
///
/// Expresses the same moment in Mayan, Egyptian, and precessional frameworks
/// simultaneously. This is the primary cross-civilizational correlation
/// function — one date, every system sankhya knows.
///
/// # Errors
///
/// Returns [`SankhyaError::ComputationError`] if any sub-computation fails
/// (e.g., Mayan Long Count overflow for extreme dates).
#[must_use = "returns the multi-calendar date or an error"]
pub fn correlate(jdn: f64) -> Result<MultiCalendarDate> {
    // Mayan calendars (None if before the Mayan epoch)
    let mayan_epoch = crate::mayan::EPOCH_JDN as f64;
    let (mayan_lc, tzolkin, haab) = if jdn >= mayan_epoch {
        let jdn_u64 = jdn as u64;
        let days = jdn_u64 - crate::mayan::EPOCH_JDN;
        let lc = crate::mayan::LongCount::from_days(days)
            .map_err(|e| SankhyaError::ComputationError(format!("Mayan Long Count: {e}")))?;
        let tz = crate::mayan::Tzolkin::from_days(days);
        let hb = crate::mayan::Haab::from_days(days);
        (Some(lc), Some(tz), Some(hb))
    } else {
        (None, None, None)
    };

    // Egyptian Sothic position
    let sothic = crate::egyptian::sothic_position(jdn);

    // Precession
    let age = precessional_age(jdn);

    // Julian year
    let year = jdn_to_julian_year(jdn);

    // Historical context from itihas (when feature-gated)
    #[cfg(feature = "itihas")]
    let approx_year = year.round() as i32;

    Ok(MultiCalendarDate {
        jdn,
        mayan_long_count: mayan_lc,
        tzolkin,
        haab,
        sothic_position: sothic,
        precessional_age: age,
        julian_year: year,
        gregorian: crate::gregorian::jdn_to_gregorian(jdn),
        coptic: crate::coptic::jdn_to_coptic(jdn),
        persian: crate::persian::jdn_to_persian(jdn),
        hebrew: crate::hebrew::jdn_to_hebrew(jdn),
        chinese_sexagenary: crate::chinese::sexagenary_from_jdn(jdn),
        aztec_tonalpohualli: crate::aztec::tonalpohualli_from_jdn(jdn),
        aztec_xiuhpohualli: crate::aztec::xiuhpohualli_from_jdn(jdn),
        #[cfg(feature = "itihas")]
        eras: itihas::era::eras_containing(approx_year),
        #[cfg(feature = "itihas")]
        civilizations: itihas::civilization::active_at(approx_year),
        #[cfg(feature = "itihas")]
        events: itihas::event::at_year(approx_year),
        #[cfg(feature = "avatara")]
        saptarishi_profiles: avatara::incarnate::IncarnateSage::ALL
            .iter()
            .map(|sage| {
                use avatara::Archetype;
                (sage.name().to_string(), sage.profile())
            })
            .collect(),
    })
}

// ---------------------------------------------------------------------------
// Unified calendar conversion API
// ---------------------------------------------------------------------------

/// A date in any supported calendar system.
///
/// Used as input to [`convert()`] for unified any-to-any calendar conversion
/// via the JDN pivot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CalendarDate {
    /// Julian Day Number (direct).
    Jdn(f64),
    /// Proleptic Gregorian calendar.
    Gregorian(crate::gregorian::GregorianDate),
    /// Coptic (Alexandrian) calendar.
    Coptic(crate::coptic::CopticDate),
    /// Persian (Solar Hijri / Jalaali) calendar.
    Persian(crate::persian::PersianDate),
    /// Hebrew (Jewish) calendar.
    Hebrew(crate::hebrew::HebrewDate),
    /// Islamic (Hijri) tabular calendar.
    Hijri(crate::islamic::HijriDate),
    /// Mayan Long Count.
    MayanLongCount(crate::mayan::LongCount),
}

/// Convert any supported calendar date to its Julian Day Number.
///
/// This is the first half of the any-to-any conversion: input → JDN.
/// The second half (JDN → all systems) is handled by [`correlate()`].
///
/// # Errors
///
/// Returns [`SankhyaError::InvalidDate`] if the input date is invalid
/// (e.g., out-of-range day for the given month).
#[must_use = "returns the JDN or an error"]
pub fn calendar_to_jdn(date: &CalendarDate) -> Result<f64> {
    match date {
        CalendarDate::Jdn(jdn) => Ok(*jdn),
        CalendarDate::Gregorian(d) => crate::gregorian::gregorian_to_jdn(d),
        CalendarDate::Coptic(d) => crate::coptic::coptic_to_jdn(d),
        CalendarDate::Persian(d) => crate::persian::persian_to_jdn(d),
        CalendarDate::Hebrew(d) => crate::hebrew::hebrew_to_jdn(d),
        CalendarDate::Hijri(d) => crate::islamic::hijri_to_jdn(d),
        CalendarDate::MayanLongCount(lc) => Ok(lc.to_julian_day() as f64 - 0.5),
    }
}

/// Convert any supported calendar date to a [`MultiCalendarDate`] containing
/// the same moment expressed in every calendar system sankhya knows.
///
/// This is the unified any-to-any conversion: input → JDN → all systems.
///
/// # Errors
///
/// Returns [`SankhyaError::InvalidDate`] if the input date is invalid.
/// Returns [`SankhyaError::ComputationError`] if any sub-computation fails.
#[must_use = "returns the multi-calendar date or an error"]
pub fn convert(date: &CalendarDate) -> Result<MultiCalendarDate> {
    let jdn = calendar_to_jdn(date)?;
    correlate(jdn)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- Constants consistency --

    #[test]
    fn great_year_days_consistent() {
        assert!((GREAT_YEAR_DAYS - GREAT_YEAR_YEARS * JULIAN_YEAR_DAYS).abs() < 0.01);
    }

    #[test]
    fn twelve_ages_equal_great_year() {
        assert!((PRECESSIONAL_AGE_YEARS * 12.0 - GREAT_YEAR_YEARS).abs() < f64::EPSILON);
    }

    #[test]
    fn younger_dryas_jdn_value() {
        let expected = 2_451_545.0 - 12_800.0 * 365.25;
        assert!((YOUNGER_DRYAS_JDN - expected).abs() < 0.01);
    }

    // -- BP conversion --

    #[test]
    fn bp_roundtrip() {
        let bp = 12_800.0;
        let jdn = bp_to_jdn(bp);
        let back = jdn_to_bp(jdn);
        assert!((back - bp).abs() < 1e-10);
    }

    #[test]
    fn bp_zero_is_1950() {
        let jdn = bp_to_jdn(0.0);
        assert!((jdn - BP_REFERENCE_JDN).abs() < f64::EPSILON);
    }

    // -- Julian year conversion --

    #[test]
    fn julian_year_roundtrip() {
        for year in [-3113i64, -753, 0, 139, 2000, 2026] {
            let jdn = julian_year_to_jdn(year);
            let back = jdn_to_julian_year(jdn);
            assert!(
                (back - year as f64).abs() < 0.01,
                "roundtrip failed for {year}"
            );
        }
    }

    // -- Precessional ages --

    #[test]
    fn age_at_younger_dryas_is_leo() {
        let pos = precessional_age(YOUNGER_DRYAS_JDN);
        assert_eq!(pos.age, PrecessionalAge::Leo);
        assert!(pos.years_into_age < 1.0);
    }

    #[test]
    fn age_at_0_ce_is_pisces() {
        let jdn = julian_year_to_jdn(1); // ~1 CE
        let pos = precessional_age(jdn);
        assert_eq!(pos.age, PrecessionalAge::Pisces);
    }

    #[test]
    fn age_at_2500_bce_is_taurus() {
        let jdn = julian_year_to_jdn(-2499); // ~2500 BCE
        let pos = precessional_age(jdn);
        assert_eq!(pos.age, PrecessionalAge::Taurus);
    }

    #[test]
    fn age_fraction_bounds() {
        let pos = precessional_age(J2000_JDN);
        assert!(pos.fraction >= 0.0);
        assert!(pos.fraction <= 1.0);
    }

    #[test]
    fn vernal_longitude_wraps() {
        let lon = vernal_point_longitude(J2000_JDN);
        assert!(lon >= 0.0);
        assert!(lon < 360.0);
    }

    #[test]
    fn full_precession_cycle_returns_to_same_age() {
        let jdn = J2000_JDN;
        let age1 = precessional_age(jdn);
        let age2 = precessional_age(jdn + GREAT_YEAR_DAYS);
        assert_eq!(age1.age, age2.age);
    }

    // -- Seven Sages --

    #[test]
    fn all_sages_count() {
        assert_eq!(all_sages_traditions().len(), 6);
    }

    #[test]
    fn sages_vedic_saptarishi() {
        let vedic = sages_tradition(Civilization::Vedic).unwrap();
        assert_eq!(vedic.group_name, "Saptarishi");
        assert_eq!(vedic.sage_names.len(), 7);
    }

    #[test]
    fn sages_babylonian_apkallu() {
        let bab = sages_tradition(Civilization::Babylonian).unwrap();
        assert_eq!(bab.group_name, "Apkallu");
        assert_eq!(bab.sage_names.len(), 7);
    }

    #[test]
    fn sages_egyptian_shemsu_hor() {
        let egy = sages_tradition(Civilization::Egyptian).unwrap();
        assert_eq!(egy.group_name, "Shemsu Hor");
    }

    #[test]
    fn sages_all_have_nonempty_names() {
        for sage in all_sages_traditions() {
            assert!(!sage.group_name.is_empty());
            assert!(!sage.sage_names.is_empty());
            assert!(!sage.source_texts.is_empty());
        }
    }

    // -- Cycle infrastructure --

    #[test]
    fn cycle_period_saros_matches_babylonian() {
        assert!(
            (cycle_period(CycleName::Saros) - crate::babylonian::SAROS_DAYS).abs() < f64::EPSILON
        );
    }

    #[test]
    fn cycle_period_sothic_matches_egyptian() {
        assert!(
            (cycle_period(CycleName::Sothic) - crate::egyptian::SOTHIC_CYCLE_DAYS as f64).abs()
                < f64::EPSILON
        );
    }

    #[test]
    fn cycles_elapsed_basic() {
        let result = cycles_elapsed(
            J2000_JDN + 365.25 * 19.0, // exactly 1 Metonic cycle later
            J2000_JDN,
            &[CycleName::Metonic],
        );
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].1, 1); // 1 complete cycle
        assert!(result[0].2.abs() < 1.0); // near-zero remainder
    }

    // -- Correlate --

    #[test]
    fn correlate_at_mayan_creation() {
        let date = correlate(crate::mayan::EPOCH_JDN as f64).unwrap();
        let lc = date.mayan_long_count.unwrap();
        assert_eq!(lc.baktun, 0);
        assert_eq!(lc.katun, 0);
        assert_eq!(lc.kin, 0);
    }

    #[test]
    fn correlate_before_mayan_epoch() {
        let date = correlate(0.0).unwrap();
        assert!(date.mayan_long_count.is_none());
        assert!(date.tzolkin.is_none());
        assert!(date.haab.is_none());
    }

    #[test]
    fn correlate_at_younger_dryas() {
        let date = correlate(YOUNGER_DRYAS_JDN).unwrap();
        assert_eq!(date.precessional_age.age, PrecessionalAge::Leo);
        assert!(date.mayan_long_count.is_none()); // way before Mayan epoch
    }

    // -- Serde --

    #[test]
    fn serde_roundtrip_age_position() {
        let pos = precessional_age(J2000_JDN);
        let json = serde_json::to_string(&pos).unwrap();
        let back: AgePosition = serde_json::from_str(&json).unwrap();
        assert_eq!(pos.age, back.age);
        assert!((pos.years_into_age - back.years_into_age).abs() < 1e-10);
        assert!((pos.fraction - back.fraction).abs() < 1e-10);
    }

    #[test]
    fn serde_roundtrip_sages_tradition() {
        let vedic = sages_tradition(Civilization::Vedic).unwrap();
        let json = serde_json::to_string(&vedic).unwrap();
        let back: SagesTradition = serde_json::from_str(&json).unwrap();
        assert_eq!(vedic, back);
    }

    #[test]
    fn serde_roundtrip_multi_calendar_date() {
        let date = correlate(crate::mayan::EPOCH_JDN as f64).unwrap();
        let json = serde_json::to_string(&date).unwrap();
        let back: MultiCalendarDate = serde_json::from_str(&json).unwrap();
        assert_eq!(date, back);
    }

    // -- Unified convert() API --

    #[test]
    fn convert_gregorian_to_all() {
        let greg = crate::gregorian::GregorianDate {
            year: 2000,
            month: crate::gregorian::GregorianMonth::January,
            day: 1,
        };
        let result = convert(&CalendarDate::Gregorian(greg)).unwrap();
        assert_eq!(result.gregorian, greg);
        assert!((result.jdn - 2_451_544.5).abs() < f64::EPSILON);
    }

    #[test]
    fn convert_hijri_roundtrip() {
        // Convert a Hijri date to MultiCalendarDate, then verify the Gregorian
        let hijri = crate::islamic::HijriDate {
            year: 1,
            month: crate::islamic::HijriMonth::Muharram,
            day: 1,
        };
        let result = convert(&CalendarDate::Hijri(hijri)).unwrap();
        // 1 Muharram 1 AH = July 16, 622 CE Gregorian (approximately)
        assert_eq!(result.gregorian.year, 622);
    }

    #[test]
    fn convert_jdn_direct() {
        let result = convert(&CalendarDate::Jdn(2_451_544.5)).unwrap();
        assert_eq!(result.gregorian.year, 2000);
        assert_eq!(
            result.gregorian.month,
            crate::gregorian::GregorianMonth::January
        );
        assert_eq!(result.gregorian.day, 1);
    }

    #[test]
    fn convert_mayan_to_all() {
        let lc = crate::mayan::LongCount::from_days(1_872_000).unwrap();
        let result = convert(&CalendarDate::MayanLongCount(lc)).unwrap();
        // 13.0.0.0.0 = Dec 21, 2012
        assert_eq!(result.gregorian.year, 2012);
        assert_eq!(
            result.gregorian.month,
            crate::gregorian::GregorianMonth::December
        );
        assert_eq!(result.gregorian.day, 21);
    }

    #[test]
    fn convert_cross_calendar_consistency() {
        // Convert Gregorian Jan 1 2025 -> get Hebrew, Persian, Coptic, etc.
        // Then convert each of those back -> should get the same JDN.
        let greg = crate::gregorian::GregorianDate {
            year: 2025,
            month: crate::gregorian::GregorianMonth::January,
            day: 1,
        };
        let result = convert(&CalendarDate::Gregorian(greg)).unwrap();
        let jdn_orig = result.jdn;

        // Hebrew roundtrip
        let jdn_heb = calendar_to_jdn(&CalendarDate::Hebrew(result.hebrew)).unwrap();
        assert!(
            (jdn_heb - jdn_orig).abs() < f64::EPSILON,
            "Hebrew roundtrip: {jdn_heb} != {jdn_orig}"
        );

        // Coptic roundtrip
        let jdn_cop = calendar_to_jdn(&CalendarDate::Coptic(result.coptic)).unwrap();
        assert!(
            (jdn_cop - jdn_orig).abs() < f64::EPSILON,
            "Coptic roundtrip: {jdn_cop} != {jdn_orig}"
        );

        // Persian roundtrip
        let jdn_per = calendar_to_jdn(&CalendarDate::Persian(result.persian)).unwrap();
        assert!(
            (jdn_per - jdn_orig).abs() < f64::EPSILON,
            "Persian roundtrip: {jdn_per} != {jdn_orig}"
        );
    }

    #[test]
    fn convert_invalid_date_errors() {
        let bad = crate::gregorian::GregorianDate {
            year: 2025,
            month: crate::gregorian::GregorianMonth::February,
            day: 30,
        };
        assert!(convert(&CalendarDate::Gregorian(bad)).is_err());
    }

    #[test]
    fn calendar_date_serde_roundtrip() {
        let cd = CalendarDate::Gregorian(crate::gregorian::GregorianDate {
            year: 2025,
            month: crate::gregorian::GregorianMonth::April,
            day: 1,
        });
        let json = serde_json::to_string(&cd).unwrap();
        let back: CalendarDate = serde_json::from_str(&json).unwrap();
        assert_eq!(cd, back);
    }
}
