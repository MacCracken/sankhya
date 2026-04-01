# ADR-005: Feature-Gated Script Rendering via Varna

## Status

Accepted

## Context

Sankhya implements numeral display for multiple ancient scripts (cuneiform, hieroglyphic, Devanagari, Greek alphabetic, Chinese rod numerals). These require the varna crate for Unicode character data and transliteration tables. Not all consumers need script rendering.

## Decision

Gate all script-rendering functions behind the `varna` Cargo feature. The feature is optional and disabled by default.

## Rationale

- Keeps the default dependency footprint minimal — consumers who only need mathematical computations (number conversions, calendar calculations) don't pull in varna.
- Follows the Rust ecosystem convention of feature-gating optional capabilities.
- All feature-gated functions are clearly documented with `Requires the \`varna\` feature.` in their doc comments.
- Feature-gated tests use `#[cfg(feature = "varna")]` test modules within each source file.

## Affected Functions

| Module | Function | Script |
|--------|----------|--------|
| babylonian | `cuneiform_digit()`, `to_cuneiform()` | Babylonian cuneiform |
| egyptian | `to_hieroglyphic()` | Egyptian hieroglyphic |
| vedic | `katapayadi_encode_devanagari()`, `to_devanagari_digits()` | Devanagari |
| greek | `isopsephy()`, `to_greek_numeral()` | Greek alphabetic |
| chinese | `to_unicode_rods()` | CJK counting rods |

## Alternatives Considered

- **Always include varna**: Simpler code but adds ~100KB to every consumer's build, even if they never use script rendering.
- **Separate crate (sankhya-scripts)**: Maximum decoupling but fragments the API surface. Users would need two crates for what feels like one feature set.
- **Runtime feature detection**: Not idiomatic in Rust; compile-time feature gating is the standard approach.

## Consequences

- CI must test with `--all-features` to exercise script-rendering code paths.
- The `varna` crate must be published to crates.io before sankhya can be published (varna 1.0.0 is now available).
- Future script-rendering functions should follow the same pattern: `#[cfg(feature = "varna")]` + doc comment noting the requirement.
