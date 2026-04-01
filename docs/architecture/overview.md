# Architecture Overview

## Module Map

```
sankhya (lib.rs)
  |-- error.rs      SankhyaError, Result alias
  |-- mayan.rs      Vigesimal, LongCount, Tzolkin, Haab, Venus, Calendar Round search
  |-- babylonian.rs Sexagesimal, Saros, reciprocals, Plimpton 322, sqrt, lunar calendar
  |-- egyptian.rs   Unit fractions, multiplication, division, decans, Sothic cycle
  |-- vedic.rs      Nikhilam, Sulba Sutra, Katapayadi, Meru Prastara
  |-- chinese.rs    Rod numerals, CRT, magic squares
  |-- greek.rs      PHI, sieve, GCD, Archimedes pi, Antikythera
  |-- roman.rs      Roman numeral conversion, validation, arithmetic
  |-- islamic.rs    Al-Khwarizmi algebra, Khayyam cubics, Hijri calendar
  |-- epoch.rs      Precession, precessional ages, Seven Sages, cycle alignment, correlate()
```

## Dependencies

- **hisab**: Higher math primitives (type compatibility, numerical methods)
- **varna** (optional): Script rendering (cuneiform, hieroglyphic, Devanagari, Greek, rod numerals)
- **serde**: Serialization for all public types
- **thiserror**: Error derive macros
- **tracing**: Structured logging (optional via `logging` feature)

## Data Flow

Sankhya is a pure computation library with no I/O. All functions are synchronous and deterministic. Types flow out to consumers via serde serialization.

## Architecture Decision Records

- [ADR-001](adr/001-unit-fraction-algorithm.md) — Egyptian fraction decomposition (greedy algorithm)
- [ADR-002](adr/002-gmt-correlation-constant.md) — GMT correlation constant (JDN 584,283)
- [ADR-003](adr/003-canonical-precession-period.md) — Canonical precession period (25,920 years)
- [ADR-004](adr/004-seven-sages-data-model.md) — Seven Sages data model (Cow<'static, str>)
- [ADR-005](adr/005-feature-gated-script-rendering.md) — Feature-gated script rendering via varna

## Consumers

- Any AGNOS component needing calendar conversions (Mayan, Egyptian, Hijri, Babylonian)
- Cross-civilizational epoch correlation (epoch module)
- Educational/demonstration tools
- Historical computation verification
