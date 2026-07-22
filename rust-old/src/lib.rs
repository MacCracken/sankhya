//! # sankhya
//!
//! **sankhya** (Sanskrit: सांख्य — enumeration/analysis) is a library of
//! ancient mathematical systems, historical calendars, and archaeoastronomy
//! for the AGNOS project.
//!
//! Faithful implementations of computational traditions from eight
//! civilizations, 10+ calendar systems, and archaeoastronomy tools —
//! with cross-civilizational epoch correlation. Every algorithm cites
//! its primary source.
//!
//! ## Modules
//!
//! | Module | Domain | Highlights |
//! |--------|--------|------------|
//! | [`mayan`] | Maya | Vigesimal numbers, Long Count, Tzolkin, Haab, Venus tables |
//! | [`babylonian`] | Babylon | Sexagesimal numbers, Saros cycle, Plimpton 322, Heron's sqrt |
//! | [`egyptian`] | Egypt | Unit fractions, doubling multiplication, stellar decans, Sothic cycle |
//! | [`vedic`] | India | Nikhilam multiplication, Sulba Sutra, Katapayadi, Meru Prastara |
//! | [`chinese`] | China | Rod numerals, CRT, magic squares, Sexagenary 60-year cycle |
//! | [`greek`] | Greece | Golden ratio, sieve, GCD, Archimedes' pi, Antikythera, isopsephy |
//! | [`roman`] | Rome | Roman numeral conversion, validation, arithmetic (I–MMMCMXCIX) |
//! | [`islamic`] | Islam | Al-Khwarizmi algebra, Khayyam cubics, Hijri calendar |
//! | [`gregorian`] | Global | Proleptic Gregorian calendar, JDN conversion |
//! | [`coptic`] | Egypt | 13-month Alexandrian calendar, Anno Martyrum |
//! | [`persian`] | Iran | Solar Hijri (Jalaali), 2820-year leap cycle |
//! | [`hebrew`] | Israel | Lunisolar, Metonic cycle, molad, dehiyyot |
//! | [`aztec`] | Mesoamerica | Tonalpohualli (260-day), Xiuhpohualli (365-day) |
//! | [`epoch`] | Cross-civilizational | Precession, Seven Sages, unified `convert()` API |
//! | [`astro`] | Archaeoastronomy | Coordinates, star catalog, precession, heliacal rising, alignment |
//!
//! ## Quick Start
//!
//! ```rust
//! use sankhya::mayan::LongCount;
//! use sankhya::babylonian;
//! use sankhya::greek;
//!
//! // Convert days to Mayan Long Count
//! let lc = LongCount::from_days(1_872_000).unwrap(); // 13.0.0.0.0 (Dec 21, 2012)
//! assert_eq!(lc.baktun, 13);
//!
//! // Babylonian square root (Heron's method, from YBC 7289 tablet)
//! let sqrt2 = babylonian::babylonian_sqrt(2.0, 10).unwrap();
//! assert!((sqrt2 - std::f64::consts::SQRT_2).abs() < 1e-15);
//!
//! // Sieve of Eratosthenes
//! let primes = greek::sieve(100);
//! assert_eq!(primes.len(), 25); // 25 primes below 100
//! ```

pub mod astro;
pub mod aztec;
pub mod babylonian;
pub mod chinese;
pub mod coptic;
pub mod egyptian;
pub mod epoch;
pub mod error;
pub mod greek;
pub mod gregorian;
pub mod hebrew;
pub mod islamic;
pub mod mayan;
pub mod persian;
pub mod roman;
pub mod vedic;

// Re-export key types at the crate root for convenience.
pub use error::{Result, SankhyaError};
