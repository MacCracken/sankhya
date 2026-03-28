# Architecture Overview

## Module Map

```
sankhya (lib.rs)
  |-- error.rs      SankhyaError, Result alias
  |-- mayan.rs      Vigesimal, LongCount, Tzolkin, Haab, Venus
  |-- babylonian.rs Sexagesimal, Saros, reciprocals, Plimpton 322, sqrt
  |-- egyptian.rs   Unit fractions, multiplication, division, decans
  |-- vedic.rs      Nikhilam, Sulba Sutra, Katapayadi, Meru Prastara
  |-- chinese.rs    Rod numerals, CRT, magic squares
  |-- greek.rs      PHI, sieve, GCD, Archimedes pi, Antikythera
```

## Dependencies

- **hisab**: Higher math primitives (currently used for type compatibility; can be expanded for numerical methods)
- **serde**: Serialization for all public types
- **thiserror**: Error derive macros
- **tracing**: Structured logging (optional via `logging` feature)

## Data Flow

Sankhya is a pure computation library with no I/O. All functions are synchronous and deterministic. Types flow out to consumers via serde serialization.

## Consumers

- Any AGNOS component needing calendar conversions (Mayan, Egyptian decan)
- Educational/demonstration tools
- Historical computation verification
