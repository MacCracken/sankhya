# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.2.0] — 2026-04-01

### Added

- **gregorian** — Proleptic Gregorian calendar with JDN conversions. March-based computational algorithm (Dershowitz & Reingold) with exact roundtrip across 6000+ years. Leap year rules, year-day computation, Display with CE/BCE
- **coptic** — Coptic (Alexandrian) calendar. 13 months (12×30 + Nasie 5/6), Anno Martyrum epoch (284 CE), leap at year mod 4 == 3
- **persian** — Persian Solar Hijri (Jalaali) calendar. Arithmetic leap year algorithm via jalaali-js breaks array (2820-year grand cycle, 683 leaps). Gregorian bridge for JDN conversion. Nowruz-anchored
- **hebrew** — Hebrew (Jewish) lunisolar calendar. Metonic 19-year cycle, molad (mean new moon) computation in chalakim, all four dehiyyot postponement rules (Lo ADU, Molad Zaken, GaTRaD, BeTUTeKPaT). Six year types (353/354/355/383/384/385). 14-variant month enum with Adar/AdarI/AdarII validation
- **aztec** — Aztec (Mexica) calendar systems. Tonalpohualli (260-day sacred cycle, 13×20 with 20 Nahuatl day signs) and Xiuhpohualli (365-day solar cycle, 18×20 + Nemontemi). Caso correlation. Calendar Round (18,980-day / 52-year cycle)
- **chinese** — Sexagenary (60-year) cycle. 10 Heavenly Stems (Tiangan) with CJK characters + 12 Earthly Branches (Dizhi) with zodiac animals. `sexagenary_from_year()` and `sexagenary_from_jdn()`. Anchored to 4 CE = Jiǎ-Zǐ
- **epoch** — Unified `convert()` API: `CalendarDate` enum wrapping all supported calendar types, `calendar_to_jdn()` dispatch, `convert()` for any-to-any conversion through the JDN pivot. Cross-calendar roundtrip verified
- **epoch** — `MultiCalendarDate` now includes Gregorian, Coptic, Persian, Hebrew, Chinese Sexagenary, Aztec Tonalpohualli, and Aztec Xiuhpohualli fields populated by `correlate()`
- **astro** — Archaeoastronomy module. Coordinate types (`CelestialCoord`, `EclipticCoord`, `HorizontalCoord`) with equatorial↔ecliptic↔horizontal conversions. Obliquity of ecliptic (IAU/Lieske 1977)
- **astro** — Star catalog: 20 archaeologically significant stars (Sirius, Canopus, Vega, Thuban, Polaris, Pleiades, etc.) with J2000.0 RA/Dec/magnitude from Hipparcos
- **astro** — Precession-corrected star positions using simplified IAU model (Lieske 1977). Validated: Thuban near pole at 2800 BCE, Polaris not pole star in antiquity. ~0.1° accuracy over 6000 years
- **astro** — Heliacal rising prediction using arcus visionis model (Schaefer 1987). Sirius rising at Memphis validated ~July-August
- **astro** — Solar position (simplified Meeus ch. 25): ecliptic longitude and declination, ~1° accuracy
- **astro** — Monument alignment analysis: `monument_alignment()` checks solstice/equinox sunrise/sunset and star rising points against a bearing. `solstice_sunrise_azimuth()` validated at Stonehenge (~50° NE). `star_rise_azimuth()` for individual stars

## [1.1.0] — 2026-04-01

### Added

- **avatara** — Optional feature-gated dependency on avatara for Saptarishi archetype profiles. When enabled, `MultiCalendarDate` includes `saptarishi_profiles` with personality data for all seven sages
- **itihas** — Optional feature-gated dependency on itihas for historical context. When enabled, `MultiCalendarDate` includes `eras`, `civilizations`, and `events` fields populated by `correlate()`

### Fixed

- **all modules** — Added `#[must_use]` to all 41 `Result`-returning public functions that were missing it, achieving full compliance with the `#[must_use]` on all pure functions principle

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
