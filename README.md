# sankhya

**sankhya** (Sanskrit: सांख्य -- enumeration/analysis) is an ancient mathematical systems library for the [AGNOS](https://github.com/MacCracken/agnosticos) project.

Faithful implementations of computational traditions from six civilizations.

## Modules

| Module | Civilization | Era | Highlights |
|--------|-------------|-----|------------|
| `mayan` | Maya | 2000 BCE+ | Vigesimal (base-20), Long Count calendar, Tzolkin, Haab, Venus tables |
| `babylonian` | Babylon | 2000-300 BCE | Sexagesimal (base-60), Saros cycle, Plimpton 322 triples, Heron's sqrt |
| `egyptian` | Egypt | 2000-1000 BCE | Unit fractions (Rhind Papyrus), doubling multiplication, stellar decans |
| `vedic` | India | 800 BCE+ | Nikhilam multiplication, Sulba Sutra geometry, Katapayadi, Meru Prastara |
| `chinese` | China | 500 BCE+ | Rod numerals, Chinese Remainder Theorem (Sun Tzu), magic squares |
| `greek` | Greece | 600 BCE-300 CE | Golden ratio, Sieve of Eratosthenes, Euclidean GCD, Archimedes' pi, Antikythera |

## Quick Start

```rust
use sankhya::mayan::LongCount;
use sankhya::babylonian;
use sankhya::greek;

// Convert days to Mayan Long Count
let lc = LongCount::from_days(1_872_000); // 13.0.0.0.0 (Dec 21, 2012)
assert_eq!(lc.baktun, 13);

// Babylonian square root (Heron's method, from YBC 7289 tablet)
let sqrt2 = babylonian::babylonian_sqrt(2.0, 10).unwrap();

// Sieve of Eratosthenes
let primes = greek::sieve(100);
assert_eq!(primes.len(), 25);
```

## Building

```bash
cargo build
cargo test
make check   # fmt + clippy + test + audit
make bench   # criterion benchmarks with history tracking
```

## Requirements

- Rust 1.89+ (MSRV)
- Dependencies: hisab, serde, thiserror, tracing

## License

GPL-3.0-only. See [LICENSE](LICENSE).
