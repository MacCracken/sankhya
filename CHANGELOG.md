# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **epoch** — Cross-civilizational epoch correlation module:
  - Precession of the equinoxes (25,920-year Great Year, canonical ancient value)
  - Precessional ages (12 zodiacal ages, Leo anchored to Younger Dryas ~10,800 BCE)
  - Younger Dryas boundary epoch constant and BP conversion utilities
  - Seven Sages traditions for all 6 civilizations (Saptarishi, Apkallu, Shemsu Hor, Popol Vuh Creators, Seven Sages of Greece, Fuxi and Nuwa)
  - `correlate()` — multi-calendar "Rosetta Stone" function (Mayan + Egyptian + precessional from a single JDN)
  - Cycle alignment search across Sothic, Saros, Venus, Calendar Round, Metonic, and precession cycles
  - Julian year and BP ↔ JDN conversion utilities
- **egyptian** — Sothic cycle (Sopdet/Sirius):
  - `sopdet()` — reference to Sirius decan (#20)
  - `sothic_drift()`, `sothic_position()` — calendar drift and cycle position anchored to Censorinus epoch (139 CE)
  - `next_sopdet_rising()` — heliacal rising prediction with latitude adjustment
  - Constants: `SOTHIC_CYCLE_CIVIL_YEARS`, `SOTHIC_CYCLE_DAYS`, `CENSORINUS_EPOCH_JDN`
- **greek** — `isopsephy()` and `to_greek_numeral()` — Greek alphabetic numeral system via lipi (feature-gated)
- **babylonian** — `cuneiform_digit()` and `to_cuneiform()` — cuneiform sexagesimal display via lipi (feature-gated)
- **egyptian** — `to_hieroglyphic()` — Egyptian hieroglyphic numeral display via lipi (feature-gated)
- **vedic** — `katapayadi_encode_devanagari()` and `to_devanagari_digits()` — Devanagari script output via lipi (feature-gated)
- **chinese** — `to_unicode_rods()` — Unicode counting rod numeral display via lipi (feature-gated)
- `lipi` optional feature — gates all script-aware display functions via the lipi multilingual engine

### Changed

- **mayan** — `LongCount::from_days()` now returns `Result` with overflow check on baktun field (was silent truncation)
- **dependencies** — `hisab` now uses published crate (`version = "1"`) instead of local path

### Fixed

- **mayan** — `to_vigesimal()` doc corrected: returns `[0]` for zero, not empty vec
- **chinese** — formatting fix in `RodNumeral::Display` (clippy)
- **benchmarks** — migrated from deprecated `criterion::black_box` to `std::hint::black_box`

## [0.1.0] - 2026-03-26

### Added

- **mayan**: Vigesimal (base-20) number system, Long Count calendar with GMT correlation (JDN 584283), Tzolkin sacred 260-day cycle with all 20 day signs, Haab solar 365-day calendar with 19 months, Calendar Round (52-year / 18,980-day cycle), Venus synodic cycle (Dresden Codex model)
- **babylonian**: Sexagesimal (base-60) number system, Saros eclipse cycle (6585.32 days), reciprocal tables for regular numbers, Plimpton 322 Pythagorean triples (all 15 rows), Babylonian/Heron's square root method
- **egyptian**: Unit fraction decomposition (greedy algorithm), Rhind Papyrus doubling multiplication, Egyptian division with unit fraction remainders, 36 stellar decans with ecliptic longitudes
- **vedic**: Vedic Nikhilam multiplication (complement method), Sulba Sutra Pythagorean theorem (Baudhayana, 800 BCE), Baudhayana's sqrt(2) approximation (577/408), Katapayadi letter-to-number encoding, Meru Prastara (Pingala's triangle, 200 BCE)
- **chinese**: Rod numeral arithmetic, Chinese Remainder Theorem (Sun Tzu's algorithm), Lo Shu magic square (3x3), Siamese method for odd-order magic squares
- **greek**: Golden ratio (PHI), Fibonacci ratio convergence, Sieve of Eratosthenes, Euclidean GCD/LCM, Archimedes' pi (polygon exhaustion method), Antikythera mechanism gear ratios (7 astronomical cycles)
- **error**: SankhyaError with InvalidDate, InvalidBase, InvalidFraction, OverflowError, ComputationError variants
- Full serde support on all public types
- Integration test suite (40+ tests)
- Criterion benchmark suite (5 benchmarks)
- CI/CD workflows (check, test, coverage, release)
