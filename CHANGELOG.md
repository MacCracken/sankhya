# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
