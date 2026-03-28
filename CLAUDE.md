# Sankhya -- Claude Code Instructions

## Project Identity

**Sankhya** (Sanskrit: enumeration/analysis) -- Ancient mathematical systems for AGNOS

- **Type**: Flat library crate
- **License**: GPL-3.0
- **MSRV**: 1.89
- **Version**: SemVer 0.1.0

## Consumers

AGNOS ecosystem crates that need historical/ancient mathematical computations, calendar systems, or number system conversions. Primary consumer: hisab (as an optional companion).

## Modules

| Module | Contents |
|--------|----------|
| `error` | SankhyaError enum (thiserror, serde, non_exhaustive) |
| `mayan` | Vigesimal numbers, Long Count, Tzolkin, Haab, Calendar Round, Venus tables |
| `babylonian` | Sexagesimal numbers, Saros cycle, reciprocals, Plimpton 322, Heron's sqrt |
| `egyptian` | Unit fractions, doubling multiplication, division, stellar decans |
| `vedic` | Nikhilam multiplication, Sulba Sutra, Katapayadi, Meru Prastara |
| `chinese` | Rod numerals, Chinese Remainder Theorem, magic squares |
| `greek` | Golden ratio, sieve, GCD/LCM, Archimedes' pi, Antikythera |

## Key Principles

- All math must be historically accurate -- real formulas, real correlations
- `#[non_exhaustive]` on ALL public enums
- `#[must_use]` on all pure functions
- Every type must be Serialize + Deserialize (serde)
- Zero unwrap/panic in library code
- All errors via Result<T, SankhyaError>

## Development Process

Follow the Work Loop from the AGNOS CLAUDE.md:
1. Work phase
2. Cleanliness check: `cargo fmt --check`, `cargo clippy --all-features --all-targets -- -D warnings`, `cargo audit`, `cargo deny check`, `RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps`
3. Test + benchmark additions
4. Run benchmarks (`./scripts/bench-history.sh`)
5. Internal review
6. Cleanliness check again
7. Documentation updates

## DO NOT

- **Do not commit or push** -- the user handles all git operations
- **NEVER use `gh` CLI**
- Do not add unnecessary dependencies
- Do not use unwrap/panic in library code
- Do not skip benchmarks before claiming performance improvements
