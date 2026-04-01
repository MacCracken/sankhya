//! # sankhya
//!
//! **sankhya** (Sanskrit: सांख्य — enumeration/analysis) is a library of
//! ancient mathematical systems for the AGNOS project.
//!
//! It provides faithful implementations of computational traditions from
//! six civilizations: Mayan, Babylonian, Egyptian, Vedic/Indian, Chinese,
//! and Greek.
//!
//! ## Modules
//!
//! | Module | Civilization | Highlights |
//! |--------|-------------|------------|
//! | [`mayan`] | Maya | Vigesimal numbers, Long Count calendar, Tzolkin, Haab, Venus tables |
//! | [`babylonian`] | Babylon | Sexagesimal numbers, Saros cycle, Plimpton 322 triples, Heron's sqrt |
//! | [`egyptian`] | Egypt | Unit fractions, doubling multiplication, stellar decans, Sothic cycle |
//! | [`vedic`] | India | Nikhilam multiplication, Sulba Sutra geometry, Katapayadi, Meru Prastara |
//! | [`chinese`] | China | Rod numerals, Chinese Remainder Theorem, magic squares |
//! | [`greek`] | Greece | Golden ratio, Sieve of Eratosthenes, Euclidean GCD, Archimedes' pi, Antikythera |
//! | [`islamic`] | Islam | Al-Khwarizmi algebra, Khayyam cubics, completion of the square, Hijri calendar |
//! | [`roman`] | Rome | Roman numeral conversion, validation, arithmetic (I–MMMCMXCIX) |
//! | [`epoch`] | Cross-civilizational | Precession, precessional ages, Younger Dryas, Seven Sages, cycle alignment |
//!
//! ## Quick Start
//!
//! ```rust
//! use sankhya::mayan::{LongCount, Tzolkin};
//! use sankhya::babylonian;
//! use sankhya::greek;
//!
//! // Convert days to Mayan Long Count
//! let lc = LongCount::from_days(1_872_000).unwrap(); // 13.0.0.0.0 (Dec 21, 2012)
//! assert_eq!(lc.baktun, 13);
//!
//! // Babylonian square root
//! let sqrt2 = babylonian::babylonian_sqrt(2.0, 10).unwrap();
//! assert!((sqrt2 - std::f64::consts::SQRT_2).abs() < 1e-15);
//!
//! // Sieve of Eratosthenes
//! let primes = greek::sieve(100);
//! assert_eq!(primes.len(), 25); // 25 primes below 100
//! ```

pub mod babylonian;
pub mod chinese;
pub mod egyptian;
pub mod epoch;
pub mod error;
pub mod greek;
pub mod islamic;
pub mod mayan;
pub mod roman;
pub mod vedic;

// Re-export key types at the crate root for convenience.
pub use error::{Result, SankhyaError};
