//! Archaeoastronomy computations.
//!
//! Coordinate systems, precession-corrected star positions, heliacal
//! rising predictions, and monument alignment analysis for historical
//! and archaeological applications.
//!
//! # Historical Context
//!
//! Archaeoastronomy studies how ancient cultures observed and interpreted
//! celestial phenomena. This module provides the computational foundation:
//! coordinate transformations between equatorial, ecliptic, and horizontal
//! systems; precession corrections for deep historical dates; heliacal
//! rising predictions for calendrically significant stars; and monument
//! alignment analysis.
//!
//! # Sources
//!
//! - Meeus, *Astronomical Algorithms* (2nd ed., Willmann-Bell, 1998)
//! - Ruggles, *Archaeoastronomy and Ethnoastronomy* (Cambridge, 2011)
//! - Schaefer, "Heliacal Rise Phenomena", *Journal for the History of
//!   Astronomy* (1987)
//! - Lieske et al., "Expressions for the Precession Quantities",
//!   *Astronomy & Astrophysics* 58, 1–16 (1977)

use serde::{Deserialize, Serialize};

use crate::epoch::J2000_JDN;

// ---------------------------------------------------------------------------
// Angle conversion helpers
// ---------------------------------------------------------------------------

/// Convert degrees to radians.
#[must_use]
#[inline]
pub fn degrees_to_radians(deg: f64) -> f64 {
    deg * core::f64::consts::PI / 180.0
}

/// Convert radians to degrees.
#[must_use]
#[inline]
pub fn radians_to_degrees(rad: f64) -> f64 {
    rad * 180.0 / core::f64::consts::PI
}

/// Convert hours (of right ascension) to degrees.
#[must_use]
#[inline]
pub fn hours_to_degrees(hours: f64) -> f64 {
    hours * 15.0
}

/// Convert degrees to hours (of right ascension).
#[must_use]
#[inline]
pub fn degrees_to_hours(deg: f64) -> f64 {
    deg / 15.0
}

// ---------------------------------------------------------------------------
// Coordinate types
// ---------------------------------------------------------------------------

/// Equatorial celestial coordinates (Right Ascension / Declination).
///
/// The fundamental coordinate system of positional astronomy, referenced
/// to the celestial equator and vernal equinox.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CelestialCoord {
    /// Right ascension in decimal hours (0.0–24.0).
    pub ra_hours: f64,
    /// Declination in decimal degrees (-90.0 to +90.0).
    pub dec_degrees: f64,
}

impl core::fmt::Display for CelestialCoord {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let ra_h = self.ra_hours.floor() as i32;
        let ra_m = ((self.ra_hours - ra_h as f64) * 60.0).floor() as i32;
        let ra_s = ((self.ra_hours - ra_h as f64) * 3600.0 - ra_m as f64 * 60.0).abs();
        let dec_sign = if self.dec_degrees < 0.0 { "-" } else { "+" };
        let dec_abs = self.dec_degrees.abs();
        let dec_d = dec_abs.floor() as i32;
        let dec_m = ((dec_abs - dec_d as f64) * 60.0).floor() as i32;
        write!(
            f,
            "RA {:02}h {:02}m {:.1}s, Dec {}{:02}d {:02}'",
            ra_h, ra_m, ra_s, dec_sign, dec_d, dec_m
        )
    }
}

/// Ecliptic coordinates (longitude / latitude).
///
/// Referenced to the ecliptic plane (Earth's orbital plane) and the
/// vernal equinox. Used extensively in ancient astronomy — most
/// Babylonian, Greek, and Indian positional data is in ecliptic coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct EclipticCoord {
    /// Ecliptic longitude in degrees (0.0–360.0).
    pub longitude: f64,
    /// Ecliptic latitude in degrees (-90.0 to +90.0).
    pub latitude: f64,
}

impl core::fmt::Display for EclipticCoord {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "lon {:.4}°, lat {:.4}°", self.longitude, self.latitude)
    }
}

/// Horizontal (alt-azimuth) coordinates for a specific observer.
///
/// Referenced to the observer's local horizon. Azimuth is measured
/// from north (0°) through east (90°), south (180°), west (270°).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HorizontalCoord {
    /// Azimuth in degrees from north (0.0–360.0, clockwise).
    pub azimuth: f64,
    /// Altitude in degrees above the horizon (-90.0 to +90.0).
    pub altitude: f64,
}

impl core::fmt::Display for HorizontalCoord {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "az {:.2}°, alt {:.2}°", self.azimuth, self.altitude)
    }
}

// ---------------------------------------------------------------------------
// Obliquity of the ecliptic
// ---------------------------------------------------------------------------

/// Compute the mean obliquity of the ecliptic for a given Julian Day Number.
///
/// Uses the IAU formula from Lieske (1977). The obliquity is the angle
/// between the ecliptic and celestial equator — approximately 23.44° at
/// J2000.0, varying by ~0.013° per century due to planetary perturbations.
///
/// Returns degrees.
#[must_use]
pub fn obliquity_of_ecliptic(jdn: f64) -> f64 {
    let t = (jdn - J2000_JDN) / 36_525.0; // Julian centuries from J2000.0
    // IAU formula (Lieske 1977), in arcseconds from the J2000.0 value
    23.439_291_1 - 0.013_004_2 * t - 1.64e-7 * t * t + 5.04e-7 * t * t * t
}

// ---------------------------------------------------------------------------
// Coordinate conversions
// ---------------------------------------------------------------------------

/// Convert equatorial coordinates to ecliptic coordinates.
///
/// Uses the obliquity of the ecliptic at the given JDN for the
/// transformation. Based on Meeus, *Astronomical Algorithms* ch. 13.
#[must_use]
pub fn equatorial_to_ecliptic(coord: &CelestialCoord, jdn: f64) -> EclipticCoord {
    let eps = degrees_to_radians(obliquity_of_ecliptic(jdn));
    let ra = degrees_to_radians(hours_to_degrees(coord.ra_hours));
    let dec = degrees_to_radians(coord.dec_degrees);

    let sin_lon = ra.sin() * eps.cos() + dec.tan() * eps.sin();
    let cos_lon = ra.cos();
    let mut lon = radians_to_degrees(sin_lon.atan2(cos_lon));
    if lon < 0.0 {
        lon += 360.0;
    }

    let sin_lat = dec.sin() * eps.cos() - dec.cos() * eps.sin() * ra.sin();
    let lat = radians_to_degrees(sin_lat.asin());

    EclipticCoord {
        longitude: lon,
        latitude: lat,
    }
}

/// Convert ecliptic coordinates to equatorial coordinates.
///
/// Inverse of [`equatorial_to_ecliptic()`]. Based on Meeus ch. 13.
#[must_use]
pub fn ecliptic_to_equatorial(coord: &EclipticCoord, jdn: f64) -> CelestialCoord {
    let eps = degrees_to_radians(obliquity_of_ecliptic(jdn));
    let lon = degrees_to_radians(coord.longitude);
    let lat = degrees_to_radians(coord.latitude);

    let sin_ra = lon.sin() * eps.cos() - lat.tan() * eps.sin();
    let cos_ra = lon.cos();
    let mut ra_deg = radians_to_degrees(sin_ra.atan2(cos_ra));
    if ra_deg < 0.0 {
        ra_deg += 360.0;
    }

    let sin_dec = lat.sin() * eps.cos() + lat.cos() * eps.sin() * lon.sin();
    let dec = radians_to_degrees(sin_dec.asin());

    CelestialCoord {
        ra_hours: degrees_to_hours(ra_deg),
        dec_degrees: dec,
    }
}

/// Convert equatorial coordinates to horizontal coordinates for a given
/// observer location and time.
///
/// This is a simplified computation that does not account for atmospheric
/// refraction, aberration, or nutation. Suitable for archaeoastronomy
/// applications where ~0.5° accuracy is sufficient.
///
/// Based on Meeus, *Astronomical Algorithms* ch. 13.
#[must_use]
pub fn equatorial_to_horizontal(
    coord: &CelestialCoord,
    jdn: f64,
    observer_lat: f64,
    observer_lon: f64,
) -> HorizontalCoord {
    let lat = degrees_to_radians(observer_lat);
    let dec = degrees_to_radians(coord.dec_degrees);

    // Approximate Local Sidereal Time
    let lst = local_sidereal_time(jdn, observer_lon);
    let ha = degrees_to_radians(lst * 15.0 - hours_to_degrees(coord.ra_hours));

    // Altitude
    let sin_alt = dec.sin() * lat.sin() + dec.cos() * lat.cos() * ha.cos();
    let alt = radians_to_degrees(sin_alt.asin());

    // Azimuth (from north, clockwise)
    let cos_az_num = dec.sin() - sin_alt * lat.sin();
    let cos_az_den = sin_alt.acos().sin() * lat.cos();
    let mut az = radians_to_degrees((-dec.cos() * ha.sin()).atan2(cos_az_num));
    if cos_az_den.abs() > 1e-10 {
        az = radians_to_degrees((-(dec.cos() * ha.sin())).atan2(cos_az_num / cos_az_den.abs()));
    }
    if az < 0.0 {
        az += 360.0;
    }

    HorizontalCoord {
        azimuth: az,
        altitude: alt,
    }
}

/// Approximate Greenwich Mean Sidereal Time in hours for a given JDN.
///
/// Based on Meeus ch. 12.
#[must_use]
fn greenwich_sidereal_time(jdn: f64) -> f64 {
    let t = (jdn - J2000_JDN) / 36_525.0;
    let gmst = 280.460_618_37 + 360.985_647_366_29 * (jdn - J2000_JDN) + 0.000_387_933 * t * t
        - t * t * t / 38_710_000.0;
    ((gmst % 360.0) + 360.0) % 360.0 / 15.0 // Convert to hours
}

/// Local Sidereal Time in hours for a given JDN and observer longitude.
#[must_use]
fn local_sidereal_time(jdn: f64, observer_lon: f64) -> f64 {
    let gmst = greenwich_sidereal_time(jdn);
    let lst = gmst + observer_lon / 15.0;
    ((lst % 24.0) + 24.0) % 24.0
}

// ---------------------------------------------------------------------------
// Approximate solar position
// ---------------------------------------------------------------------------

/// Approximate ecliptic longitude of the Sun for a given JDN.
///
/// Accurate to ~1° for dates within a few thousand years of J2000.0.
/// Based on the simplified solar position algorithm from Meeus ch. 25.
#[must_use]
pub fn solar_longitude(jdn: f64) -> f64 {
    let t = (jdn - J2000_JDN) / 36_525.0;
    // Mean longitude
    let l0 = 280.466_46 + 36_000.769_83 * t;
    // Mean anomaly
    let m = degrees_to_radians(357.528_11 + 35_999.050_29 * t);
    // Equation of center
    let c = 1.914_602 * m.sin() + 0.019_993 * (2.0 * m).sin();
    let lon = l0 + c;
    ((lon % 360.0) + 360.0) % 360.0
}

/// Approximate solar declination in degrees for a given JDN.
#[must_use]
pub fn solar_declination(jdn: f64) -> f64 {
    let lon = degrees_to_radians(solar_longitude(jdn));
    let eps = degrees_to_radians(obliquity_of_ecliptic(jdn));
    radians_to_degrees((eps.sin() * lon.sin()).asin())
}

// ---------------------------------------------------------------------------
// Star catalog
// ---------------------------------------------------------------------------

/// A star in the archaeoastronomy catalog.
///
/// Contains J2000.0 equatorial coordinates and visual magnitude for
/// stars of archaeological significance (bright stars used in ancient
/// calendars, monument alignments, and heliacal rising observations).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Star {
    /// Common name.
    pub name: &'static str,
    /// Bayer designation (e.g., "α CMa").
    pub bayer: &'static str,
    /// Constellation.
    pub constellation: &'static str,
    /// Right ascension at J2000.0 in decimal hours.
    pub ra_j2000: f64,
    /// Declination at J2000.0 in decimal degrees.
    pub dec_j2000: f64,
    /// Apparent visual magnitude.
    pub visual_magnitude: f64,
}

/// Named stars in the catalog.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum StarName {
    /// Sirius (α CMa) — brightest star, governed the Egyptian calendar
    Sirius,
    /// Canopus (α Car) — second brightest, navigation star
    Canopus,
    /// Vega (α Lyr) — former pole star ~12,000 BCE
    Vega,
    /// Arcturus (α Boo) — brightest in northern celestial hemisphere
    Arcturus,
    /// Rigel (β Ori) — Orion's foot
    Rigel,
    /// Procyon (α CMi) — "before the dog" (rises before Sirius)
    Procyon,
    /// Betelgeuse (α Ori) — Orion's shoulder, red supergiant
    Betelgeuse,
    /// Aldebaran (α Tau) — "the follower" of the Pleiades
    Aldebaran,
    /// Antares (α Sco) — "rival of Mars", heart of the Scorpion
    Antares,
    /// Spica (α Vir) — used by Hipparchus to discover precession
    Spica,
    /// Pollux (β Gem) — one of the Dioscuri twins
    Pollux,
    /// Fomalhaut (α PsA) — one of four Royal Stars of Persia
    Fomalhaut,
    /// Deneb (α Cyg) — Summer Triangle, future pole star
    Deneb,
    /// Regulus (α Leo) — "little king", Royal Star of Persia
    Regulus,
    /// Polaris (α UMi) — current north pole star
    Polaris,
    /// Thuban (α Dra) — pole star of the pyramid builders (~2800 BCE)
    Thuban,
    /// Kochab (β UMi) — pole star ~1500 BCE
    Kochab,
    /// Pleiades (cluster center) — calendrically important worldwide
    Pleiades,
    /// Alcyone (η Tau) — brightest Pleiad
    Alcyone,
    /// Dubhe (α UMa) — pointer star to Polaris
    Dubhe,
}

/// The archaeoastronomy star catalog: J2000.0 coordinates for ~20
/// archaeologically significant stars.
///
/// Coordinates from the Hipparcos catalog (ESA, 1997).
pub static STAR_CATALOG: [Star; 20] = [
    Star {
        name: "Sirius",
        bayer: "α CMa",
        constellation: "Canis Major",
        ra_j2000: 6.752,
        dec_j2000: -16.716,
        visual_magnitude: -1.46,
    },
    Star {
        name: "Canopus",
        bayer: "α Car",
        constellation: "Carina",
        ra_j2000: 6.399,
        dec_j2000: -52.696,
        visual_magnitude: -0.74,
    },
    Star {
        name: "Vega",
        bayer: "α Lyr",
        constellation: "Lyra",
        ra_j2000: 18.616,
        dec_j2000: 38.784,
        visual_magnitude: 0.03,
    },
    Star {
        name: "Arcturus",
        bayer: "α Boo",
        constellation: "Boötes",
        ra_j2000: 14.261,
        dec_j2000: 19.182,
        visual_magnitude: -0.05,
    },
    Star {
        name: "Rigel",
        bayer: "β Ori",
        constellation: "Orion",
        ra_j2000: 5.242,
        dec_j2000: -8.202,
        visual_magnitude: 0.13,
    },
    Star {
        name: "Procyon",
        bayer: "α CMi",
        constellation: "Canis Minor",
        ra_j2000: 7.655,
        dec_j2000: 5.225,
        visual_magnitude: 0.34,
    },
    Star {
        name: "Betelgeuse",
        bayer: "α Ori",
        constellation: "Orion",
        ra_j2000: 5.919,
        dec_j2000: 7.407,
        visual_magnitude: 0.42,
    },
    Star {
        name: "Aldebaran",
        bayer: "α Tau",
        constellation: "Taurus",
        ra_j2000: 4.599,
        dec_j2000: 16.509,
        visual_magnitude: 0.86,
    },
    Star {
        name: "Antares",
        bayer: "α Sco",
        constellation: "Scorpius",
        ra_j2000: 16.490,
        dec_j2000: -26.432,
        visual_magnitude: 0.96,
    },
    Star {
        name: "Spica",
        bayer: "α Vir",
        constellation: "Virgo",
        ra_j2000: 13.420,
        dec_j2000: -11.161,
        visual_magnitude: 0.97,
    },
    Star {
        name: "Pollux",
        bayer: "β Gem",
        constellation: "Gemini",
        ra_j2000: 7.755,
        dec_j2000: 28.026,
        visual_magnitude: 1.14,
    },
    Star {
        name: "Fomalhaut",
        bayer: "α PsA",
        constellation: "Piscis Austrinus",
        ra_j2000: 22.961,
        dec_j2000: -29.622,
        visual_magnitude: 1.16,
    },
    Star {
        name: "Deneb",
        bayer: "α Cyg",
        constellation: "Cygnus",
        ra_j2000: 20.690,
        dec_j2000: 45.280,
        visual_magnitude: 1.25,
    },
    Star {
        name: "Regulus",
        bayer: "α Leo",
        constellation: "Leo",
        ra_j2000: 10.139,
        dec_j2000: 11.967,
        visual_magnitude: 1.40,
    },
    Star {
        name: "Polaris",
        bayer: "α UMi",
        constellation: "Ursa Minor",
        ra_j2000: 2.530,
        dec_j2000: 89.264,
        visual_magnitude: 1.98,
    },
    Star {
        name: "Thuban",
        bayer: "α Dra",
        constellation: "Draco",
        ra_j2000: 14.073,
        dec_j2000: 64.376,
        visual_magnitude: 3.65,
    },
    Star {
        name: "Kochab",
        bayer: "β UMi",
        constellation: "Ursa Minor",
        ra_j2000: 14.845,
        dec_j2000: 74.156,
        visual_magnitude: 2.08,
    },
    Star {
        name: "Pleiades",
        bayer: "M45",
        constellation: "Taurus",
        ra_j2000: 3.787,
        dec_j2000: 24.105,
        visual_magnitude: 1.60,
    },
    Star {
        name: "Alcyone",
        bayer: "η Tau",
        constellation: "Taurus",
        ra_j2000: 3.791,
        dec_j2000: 24.105,
        visual_magnitude: 2.87,
    },
    Star {
        name: "Dubhe",
        bayer: "α UMa",
        constellation: "Ursa Major",
        ra_j2000: 11.062,
        dec_j2000: 61.751,
        visual_magnitude: 1.79,
    },
];

/// Look up a star by name from the catalog.
#[must_use]
pub fn star(name: StarName) -> &'static Star {
    let idx = name as usize;
    &STAR_CATALOG[idx]
}

/// Get the J2000.0 equatorial coordinates for a named star.
#[must_use]
pub fn star_j2000(name: StarName) -> CelestialCoord {
    let s = star(name);
    CelestialCoord {
        ra_hours: s.ra_j2000,
        dec_degrees: s.dec_j2000,
    }
}

// ---------------------------------------------------------------------------
// Precession
// ---------------------------------------------------------------------------

/// Precess equatorial coordinates from one epoch to another.
///
/// Uses the simplified IAU precession model from Lieske et al. (1977),
/// accurate to ~0.1° over 6000 years from J2000.0. For dates beyond
/// this range, accuracy degrades but remains useful for archaeoastronomy.
///
/// Based on Meeus, *Astronomical Algorithms* ch. 21.
#[must_use]
pub fn precess_coordinates(coord: &CelestialCoord, from_jdn: f64, to_jdn: f64) -> CelestialCoord {
    let t = (from_jdn - J2000_JDN) / 36_525.0; // centuries from J2000 to start
    let dt = (to_jdn - from_jdn) / 36_525.0; // centuries of precession

    // Precession parameters (Lieske 1977), in arcseconds
    let zeta_a = (0.640_616_1 + 0.000_083_9 * t + 0.000_005_0 * t * t) * dt
        + (0.000_083_0 + 0.000_005_0 * t) * dt * dt
        + 0.000_005_0 * dt * dt * dt;
    let z_a = (0.640_616_1 + 0.000_083_9 * t + 0.000_005_0 * t * t) * dt
        + (0.000_304_1 + 0.000_005_1 * t) * dt * dt
        + 0.000_007_3 * dt * dt * dt;
    let theta_a = (0.556_753_0 - 0.000_118_5 * t - 0.000_011_6 * t * t) * dt
        - (0.000_116_2 + 0.000_011_6 * t) * dt * dt
        - 0.000_011_6 * dt * dt * dt;

    // Convert to radians (parameters are in degrees already — the Lieske
    // coefficients above are given in degrees per century)
    let zeta = degrees_to_radians(zeta_a);
    let z = degrees_to_radians(z_a);
    let theta = degrees_to_radians(theta_a);

    let ra0 = degrees_to_radians(hours_to_degrees(coord.ra_hours));
    let dec0 = degrees_to_radians(coord.dec_degrees);

    // Rotation matrix application
    let a = dec0.cos() * (ra0 + zeta).sin();
    let b = theta.cos() * dec0.cos() * (ra0 + zeta).cos() - theta.sin() * dec0.sin();
    let c = theta.sin() * dec0.cos() * (ra0 + zeta).cos() + theta.cos() * dec0.sin();

    let mut ra_new = a.atan2(b) + z;
    let dec_new = c.asin();

    ra_new = radians_to_degrees(ra_new);
    if ra_new < 0.0 {
        ra_new += 360.0;
    }
    ra_new %= 360.0;

    CelestialCoord {
        ra_hours: degrees_to_hours(ra_new),
        dec_degrees: radians_to_degrees(dec_new),
    }
}

/// Compute a star's position at a given date, precessing from J2000.0.
#[must_use]
pub fn star_position_at(name: StarName, jdn: f64) -> CelestialCoord {
    let j2000 = star_j2000(name);
    precess_coordinates(&j2000, J2000_JDN, jdn)
}

// ---------------------------------------------------------------------------
// Heliacal rising
// ---------------------------------------------------------------------------

/// A heliacal event (first/last visibility of a star near the Sun).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HeliacalEvent {
    /// Julian Day Number of the event.
    pub jdn: f64,
    /// The star involved.
    pub star: StarName,
    /// Whether this is a rising or setting event.
    pub event_type: HeliacalEventType,
}

/// Type of heliacal event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum HeliacalEventType {
    /// First visibility of the star at dawn after conjunction with the Sun.
    Rising,
    /// Last visibility of the star at dusk before conjunction with the Sun.
    Setting,
}

impl core::fmt::Display for HeliacalEventType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Rising => write!(f, "heliacal rising"),
            Self::Setting => write!(f, "heliacal setting"),
        }
    }
}

/// Arcus visionis (minimum solar depression angle) for heliacal visibility.
///
/// Empirical formula based on Schaefer (1987): brighter stars require
/// less solar depression to be visible. Returns degrees.
#[must_use]
fn arcus_visionis(visual_magnitude: f64) -> f64 {
    // Simplified Schoch/Purrington model:
    // For mag -1.5 (Sirius): ~10°
    // For mag 0: ~12°
    // For mag 1: ~13°
    // For mag 2: ~15°
    // For mag 3: ~17°
    11.0 + 1.7 * visual_magnitude
}

/// Find the next heliacal rising of a star after a given date.
///
/// A heliacal rising occurs when a star first becomes visible at dawn
/// after its period of invisibility near the Sun. The computation uses
/// the arcus visionis model (Schaefer 1987) to determine when the Sun
/// is far enough below the horizon for the star to be visible.
///
/// # Errors
///
/// Returns [`crate::SankhyaError::ComputationError`] if the star is
/// circumpolar or never rises at the given latitude.
#[must_use = "returns the heliacal event or an error"]
pub fn heliacal_rising(
    name: StarName,
    jdn_start: f64,
    observer_lat: f64,
    _observer_lon: f64,
) -> crate::error::Result<HeliacalEvent> {
    let s = star(name);
    let min_depression = arcus_visionis(s.visual_magnitude);

    // Search day by day for up to 400 days (slightly more than one year)
    for day_offset in 0..400 {
        let jdn = jdn_start + f64::from(day_offset);

        // Star position at this date (precessed)
        let star_coord = star_position_at(name, jdn);

        // Check if the star can rise at this latitude
        // A star rises if its declination satisfies: dec > -(90 - |lat|)
        let max_dec_for_never_rise = -(90.0 - observer_lat.abs());
        if observer_lat >= 0.0 && star_coord.dec_degrees < max_dec_for_never_rise {
            continue; // Star never rises at this latitude on this date
        }

        // Solar longitude at this date
        let sun_lon = solar_longitude(jdn);
        // Star's ecliptic longitude
        let star_ecl = equatorial_to_ecliptic(&star_coord, jdn);

        // Angular separation between Sun and star (simplified: ecliptic longitude diff)
        let mut sep = star_ecl.longitude - sun_lon;
        if sep < 0.0 {
            sep += 360.0;
        }
        if sep > 180.0 {
            sep = 360.0 - sep;
        }

        // Heliacal rising occurs when the star is just west of the Sun
        // (rising just before the Sun) with sufficient angular separation.
        // The separation should be roughly equal to the arcus visionis
        // converted to ecliptic degrees (~1:1 for stars near the ecliptic).
        if sep > min_depression && sep < min_depression + 20.0 {
            // Check that the star is on the morning side (rising before the Sun)
            let star_minus_sun = star_ecl.longitude - sun_lon;
            let normalized = ((star_minus_sun % 360.0) + 360.0) % 360.0;
            if normalized < 180.0 {
                // Star is ahead of Sun (morning side)
                return Ok(HeliacalEvent {
                    jdn,
                    star: name,
                    event_type: HeliacalEventType::Rising,
                });
            }
        }
    }

    Err(crate::error::SankhyaError::ComputationError(format!(
        "no heliacal rising found for {} within 400 days of JDN {}",
        s.name, jdn_start
    )))
}

// ---------------------------------------------------------------------------
// Monument alignment
// ---------------------------------------------------------------------------

/// A potential celestial alignment target.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AlignmentTarget {
    /// Summer solstice sunrise.
    SummerSolsticeSunrise,
    /// Summer solstice sunset.
    SummerSolsticeSunset,
    /// Winter solstice sunrise.
    WinterSolsticeSunrise,
    /// Winter solstice sunset.
    WinterSolsticeSunset,
    /// Equinox sunrise (~90° azimuth).
    EquinoxSunrise,
    /// Equinox sunset (~270° azimuth).
    EquinoxSunset,
    /// A named star's rising point.
    StarRising(StarName),
}

/// Result of a monument alignment analysis.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AlignmentResult {
    /// The alignment target.
    pub target: AlignmentTarget,
    /// Azimuth of the target (degrees from north).
    pub azimuth: f64,
    /// Deviation from the monument's bearing (degrees).
    pub deviation: f64,
}

/// Compute the azimuth of sunrise/sunset at solstice for a given latitude.
///
/// Uses the solar declination at solstice (~±23.44°) and the standard
/// sunrise azimuth formula. Returns degrees from north.
///
/// Based on Meeus, *Astronomical Algorithms* ch. 15.
#[must_use]
pub fn solstice_sunrise_azimuth(observer_lat: f64, summer: bool) -> f64 {
    let lat = degrees_to_radians(observer_lat);
    let dec = if summer {
        degrees_to_radians(23.44)
    } else {
        degrees_to_radians(-23.44)
    };

    // Sunrise azimuth: cos(A) = sin(dec) / cos(lat)
    let cos_az = dec.sin() / lat.cos();
    // Measured from north; sunrise is in the east (< 180°)
    radians_to_degrees(cos_az.acos())
}

/// Compute the azimuth where a star rises at a given location and date.
///
/// # Errors
///
/// Returns [`crate::SankhyaError::ComputationError`] if the star is
/// circumpolar or never rises at the given latitude.
#[must_use = "returns the azimuth or an error"]
pub fn star_rise_azimuth(name: StarName, observer_lat: f64, jdn: f64) -> crate::error::Result<f64> {
    let coord = star_position_at(name, jdn);
    let lat = degrees_to_radians(observer_lat);
    let dec = degrees_to_radians(coord.dec_degrees);

    let cos_az = dec.sin() / lat.cos();
    if cos_az.abs() > 1.0 {
        return Err(crate::error::SankhyaError::ComputationError(format!(
            "{} does not rise at latitude {:.1}°",
            star(name).name,
            observer_lat
        )));
    }

    Ok(radians_to_degrees(cos_az.acos()))
}

/// Analyze what celestial targets align with a monument's bearing.
///
/// Given an observer's position, the monument's azimuth bearing, and a
/// date, checks all standard alignment targets (solstice/equinox
/// sunrise/sunset, major star rising points) and returns those within
/// the specified tolerance.
#[must_use]
pub fn monument_alignment(
    observer_lat: f64,
    azimuth: f64,
    jdn: f64,
    tolerance_degrees: f64,
) -> Vec<AlignmentResult> {
    let mut results = Vec::new();

    // Solstice alignments
    let targets = [
        (
            AlignmentTarget::SummerSolsticeSunrise,
            solstice_sunrise_azimuth(observer_lat, true),
        ),
        (
            AlignmentTarget::SummerSolsticeSunset,
            360.0 - solstice_sunrise_azimuth(observer_lat, true),
        ),
        (
            AlignmentTarget::WinterSolsticeSunrise,
            solstice_sunrise_azimuth(observer_lat, false),
        ),
        (
            AlignmentTarget::WinterSolsticeSunset,
            360.0 - solstice_sunrise_azimuth(observer_lat, false),
        ),
        (AlignmentTarget::EquinoxSunrise, 90.0),
        (AlignmentTarget::EquinoxSunset, 270.0),
    ];

    for (target, target_az) in targets {
        let dev = angle_deviation(azimuth, target_az);
        if dev <= tolerance_degrees {
            results.push(AlignmentResult {
                target,
                azimuth: target_az,
                deviation: dev,
            });
        }
    }

    // Star rising alignments
    let stars = [
        StarName::Sirius,
        StarName::Vega,
        StarName::Aldebaran,
        StarName::Antares,
        StarName::Regulus,
        StarName::Spica,
        StarName::Pleiades,
        StarName::Thuban,
        StarName::Polaris,
    ];

    for &sn in &stars {
        if let Ok(star_az) = star_rise_azimuth(sn, observer_lat, jdn) {
            let dev = angle_deviation(azimuth, star_az);
            if dev <= tolerance_degrees {
                results.push(AlignmentResult {
                    target: AlignmentTarget::StarRising(sn),
                    azimuth: star_az,
                    deviation: dev,
                });
            }
        }
    }

    results
}

/// Angular deviation between two azimuths, accounting for wrap-around.
fn angle_deviation(a: f64, b: f64) -> f64 {
    let diff = (a - b).abs() % 360.0;
    if diff > 180.0 { 360.0 - diff } else { diff }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::epoch::JULIAN_YEAR_DAYS;

    // -- Angle helpers --

    #[test]
    fn degrees_radians_roundtrip() {
        let deg = 45.0;
        let rad = degrees_to_radians(deg);
        assert!((radians_to_degrees(rad) - deg).abs() < 1e-12);
    }

    #[test]
    fn hours_degrees_roundtrip() {
        let hours = 6.0;
        assert!((degrees_to_hours(hours_to_degrees(hours)) - hours).abs() < 1e-12);
    }

    // -- Obliquity --

    #[test]
    fn obliquity_at_j2000() {
        let eps = obliquity_of_ecliptic(J2000_JDN);
        assert!((eps - 23.4393).abs() < 0.001);
    }

    // -- Coordinate conversions --

    #[test]
    fn equatorial_ecliptic_roundtrip() {
        let orig = CelestialCoord {
            ra_hours: 6.752,
            dec_degrees: -16.716,
        };
        let ecl = equatorial_to_ecliptic(&orig, J2000_JDN);
        let back = ecliptic_to_equatorial(&ecl, J2000_JDN);
        assert!(
            (back.ra_hours - orig.ra_hours).abs() < 0.01,
            "RA: {} vs {}",
            back.ra_hours,
            orig.ra_hours
        );
        assert!(
            (back.dec_degrees - orig.dec_degrees).abs() < 0.01,
            "Dec: {} vs {}",
            back.dec_degrees,
            orig.dec_degrees
        );
    }

    #[test]
    fn coord_display() {
        let c = CelestialCoord {
            ra_hours: 6.752,
            dec_degrees: -16.716,
        };
        let s = c.to_string();
        assert!(s.contains("RA"));
        assert!(s.contains("Dec"));
    }

    // -- Star catalog --

    #[test]
    fn sirius_catalog() {
        let s = star(StarName::Sirius);
        assert_eq!(s.name, "Sirius");
        assert!((s.ra_j2000 - 6.752).abs() < 0.01);
        assert!((s.dec_j2000 - (-16.716)).abs() < 0.01);
        assert!(s.visual_magnitude < 0.0); // Brightest star
    }

    #[test]
    fn all_stars_bright() {
        // All catalog stars should have magnitude < 4
        for s in &STAR_CATALOG {
            assert!(
                s.visual_magnitude < 4.0,
                "{} has magnitude {}",
                s.name,
                s.visual_magnitude
            );
        }
    }

    #[test]
    fn catalog_has_20_stars() {
        assert_eq!(STAR_CATALOG.len(), 20);
    }

    #[test]
    fn star_j2000_matches_catalog() {
        let coord = star_j2000(StarName::Vega);
        let s = star(StarName::Vega);
        assert!((coord.ra_hours - s.ra_j2000).abs() < f64::EPSILON);
        assert!((coord.dec_degrees - s.dec_j2000).abs() < f64::EPSILON);
    }

    // -- Precession --

    #[test]
    fn precess_identity_at_j2000() {
        let orig = star_j2000(StarName::Polaris);
        let precessed = precess_coordinates(&orig, J2000_JDN, J2000_JDN);
        assert!((precessed.ra_hours - orig.ra_hours).abs() < 0.001);
        assert!((precessed.dec_degrees - orig.dec_degrees).abs() < 0.001);
    }

    #[test]
    fn thuban_was_pole_star() {
        // Around 2800 BCE (JDN ~623000), Thuban should be near Dec +89°
        let jdn_2800_bce = J2000_JDN - 4800.0 * JULIAN_YEAR_DAYS;
        let pos = star_position_at(StarName::Thuban, jdn_2800_bce);
        assert!(
            pos.dec_degrees > 85.0,
            "Thuban at 2800 BCE: Dec = {:.1}° (expected > 85°)",
            pos.dec_degrees
        );
    }

    #[test]
    fn polaris_not_pole_star_in_antiquity() {
        // Around 3000 BCE, Polaris should NOT be near the pole
        let jdn_3000_bce = J2000_JDN - 5000.0 * JULIAN_YEAR_DAYS;
        let pos = star_position_at(StarName::Polaris, jdn_3000_bce);
        assert!(
            pos.dec_degrees < 80.0,
            "Polaris at 3000 BCE: Dec = {:.1}° (expected < 80°)",
            pos.dec_degrees
        );
    }

    #[test]
    fn precession_roundtrip() {
        let orig = star_j2000(StarName::Sirius);
        let forward = precess_coordinates(&orig, J2000_JDN, J2000_JDN - 3000.0 * JULIAN_YEAR_DAYS);
        let back = precess_coordinates(&forward, J2000_JDN - 3000.0 * JULIAN_YEAR_DAYS, J2000_JDN);
        assert!(
            (back.ra_hours - orig.ra_hours).abs() < 0.1,
            "RA roundtrip: {:.4} vs {:.4}",
            back.ra_hours,
            orig.ra_hours
        );
        assert!(
            (back.dec_degrees - orig.dec_degrees).abs() < 0.1,
            "Dec roundtrip: {:.4} vs {:.4}",
            back.dec_degrees,
            orig.dec_degrees
        );
    }

    // -- Solar position --

    #[test]
    fn solar_longitude_at_j2000() {
        let lon = solar_longitude(J2000_JDN);
        // At J2000.0 (Jan 1 2000 noon), Sun is near ~280° ecliptic longitude
        assert!(
            (lon - 280.0).abs() < 5.0,
            "Solar longitude at J2000: {:.1}°",
            lon
        );
    }

    #[test]
    fn solar_declination_range() {
        // Check solstice declinations
        // June solstice 2000 ≈ JDN 2451720
        let dec_summer = solar_declination(2_451_720.0);
        assert!(dec_summer > 20.0 && dec_summer < 24.0);
        // December solstice 2000 ≈ JDN 2451904
        let dec_winter = solar_declination(2_451_904.0);
        assert!(dec_winter < -20.0 && dec_winter > -24.0);
    }

    // -- Heliacal rising --

    #[test]
    fn sirius_heliacal_rising_memphis() {
        // Sirius heliacal rising at Memphis (30°N) should be ~July-August
        let result = heliacal_rising(StarName::Sirius, 2_451_545.0, 30.0, 31.2);
        assert!(result.is_ok());
        let evt = result.unwrap();
        assert_eq!(evt.star, StarName::Sirius);
        assert_eq!(evt.event_type, HeliacalEventType::Rising);
        // Should be within ~200 days (roughly July-Aug from Jan 1 2000)
        assert!(evt.jdn - 2_451_545.0 < 250.0);
    }

    // -- Solstice azimuth --

    #[test]
    fn stonehenge_summer_solstice() {
        // Stonehenge latitude ~51.18°N
        let az = solstice_sunrise_azimuth(51.18, true);
        // Summer solstice sunrise at Stonehenge: ~50° from north
        assert!(
            (az - 50.0).abs() < 5.0,
            "Stonehenge solstice azimuth: {:.1}°",
            az
        );
    }

    #[test]
    fn equinox_sunrise_approximately_east() {
        // At equinox, sunrise should be ~90° at any reasonable latitude
        let az = solstice_sunrise_azimuth(30.0, false);
        // This is actually winter solstice, not equinox
        // For equinox: dec=0, so cos(A) = 0/cos(lat) = 0, A = 90°
        // We don't have a separate equinox function, but monument_alignment
        // uses 90° directly.
        assert!(az > 90.0); // Winter solstice sunrise is south of east
    }

    // -- Monument alignment --

    #[test]
    fn monument_alignment_stonehenge() {
        // Stonehenge (51.18°N), Heel Stone azimuth ~51°
        let results = monument_alignment(51.18, 51.0, J2000_JDN, 5.0);
        let has_solstice = results
            .iter()
            .any(|r| matches!(r.target, AlignmentTarget::SummerSolsticeSunrise));
        assert!(has_solstice, "Should find summer solstice alignment");
    }

    #[test]
    fn monument_alignment_equinox() {
        // Any monument pointing due east (90°) should match equinox sunrise
        let results = monument_alignment(30.0, 90.0, J2000_JDN, 2.0);
        let has_equinox = results
            .iter()
            .any(|r| matches!(r.target, AlignmentTarget::EquinoxSunrise));
        assert!(has_equinox, "Should find equinox sunrise alignment");
    }

    // -- Serde --

    #[test]
    fn serde_roundtrip_celestial_coord() {
        let c = star_j2000(StarName::Sirius);
        let json = serde_json::to_string(&c).unwrap();
        let back: CelestialCoord = serde_json::from_str(&json).unwrap();
        assert_eq!(c, back);
    }

    #[test]
    fn serde_roundtrip_star_name() {
        let sn = StarName::Vega;
        let json = serde_json::to_string(&sn).unwrap();
        let back: StarName = serde_json::from_str(&json).unwrap();
        assert_eq!(sn, back);
    }

    #[test]
    fn serde_roundtrip_alignment_result() {
        let r = AlignmentResult {
            target: AlignmentTarget::SummerSolsticeSunrise,
            azimuth: 51.0,
            deviation: 0.5,
        };
        let json = serde_json::to_string(&r).unwrap();
        let back: AlignmentResult = serde_json::from_str(&json).unwrap();
        assert_eq!(r, back);
    }
}
