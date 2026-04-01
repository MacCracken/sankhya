# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] — 2026-04-01

### Added

- **mayan** — Vigesimal (base-20) number system, Long Count calendar with GMT correlation (JDN 584,283), Tzolkin sacred 260-day cycle with all 20 day signs, Haab solar 365-day calendar with 19 months, Calendar Round (52-year / 18,980-day cycle), Venus synodic cycle (Dresden Codex model), Calendar Round date search (`find_calendar_round()`, `find_tzolkin()`)
- **babylonian** — Sexagesimal (base-60) number system, Saros eclipse cycle (6,585.32 days), reciprocal tables for regular numbers, Plimpton 322 Pythagorean triples (all 15 rows), Babylonian/Heron's square root method with NaN/Inf validation, lunar calendar (Seleucid Era epoch, 12 months, alternating 30/29 days, JDN conversions), synodic month computation, cuneiform display via varna (feature-gated)
- **egyptian** — Unit fraction decomposition (greedy algorithm with overflow checking), Rhind Papyrus doubling multiplication, Egyptian division with unit fraction remainders, 36 stellar decans with ecliptic longitudes, Sothic cycle (Sopdet/Sirius heliacal rising, calendar drift, Censorinus epoch), hieroglyphic numeral display via varna (feature-gated)
- **vedic** — Vedic Nikhilam multiplication (complement method), Sulba Sutra Pythagorean theorem (Baudhayana, 800 BCE), Baudhayana's sqrt(2) approximation (577/408), Katapayadi letter-to-number encoding with Devanagari output via varna (feature-gated), Meru Prastara (Pingala's triangle, 200 BCE)
- **chinese** — Rod numeral arithmetic, Chinese Remainder Theorem (Sun Tzu's algorithm), Lo Shu magic square (3x3), Siamese method for odd-order magic squares, Unicode counting rod display via varna (feature-gated)
- **greek** — Golden ratio (PHI), Fibonacci ratio convergence, Sieve of Eratosthenes, Euclidean GCD/LCM, Archimedes' pi (polygon exhaustion method), Antikythera mechanism gear ratios (7 astronomical cycles), isopsephy and Greek alphabetic numerals via varna (feature-gated)
- **islamic** — Al-Khwarizmi's six canonical quadratic equation forms with solver, Omar Khayyam's cubic equation classification (5 types) with Newton's method root-finding, geometric completion of the square, Hijri calendar (tabular 30-year cycle, leap year logic, JDN conversions, 12 months with Display)
- **roman** — Roman numeral system: decimal ↔ Roman conversion (1–3999), canonical subtractive notation with round-trip validation, arithmetic (add, subtract, multiply, divide with remainder), `RomanNumeral` type with serde support, `is_valid_roman()` validator
- **epoch** — Cross-civilizational epoch correlation: precession of the equinoxes (25,920-year Great Year, canonical ancient value), 12 precessional ages (Leo anchored to Younger Dryas ~10,800 BCE), Seven Sages traditions for all 6 civilizations, `correlate()` multi-calendar "Rosetta Stone" function, cycle alignment search across Sothic/Saros/Venus/Calendar Round/Metonic/precession, Julian year and BP ↔ JDN conversion utilities
- **error** — `SankhyaError` with `InvalidDate`, `InvalidBase`, `InvalidFraction`, `OverflowError`, `ComputationError` variants (`#[non_exhaustive]`, serde, thiserror)
- **varna** optional feature — gates all script-aware display functions (cuneiform, hieroglyphic, Devanagari, Greek alphabetic, CJK counting rods) via the varna multilingual engine
- Full serde (Serialize + Deserialize) support on all public types
- 275 tests: 155 unit, 64 adversarial (hostile input fuzzing), 55 integration, 1 doc test
- 9 modules covering 7 civilizations + cross-civilizational epoch correlation
- 11 Criterion benchmarks across all modules
- 9 per-module examples + 1 overview example
- 5 Architecture Decision Records (ADRs)
- CI/CD: check, clippy, security audit, cargo-deny, MSRV (1.89), cross-platform tests (Linux/macOS), coverage (85% gate), doc warnings, automated release pipeline
