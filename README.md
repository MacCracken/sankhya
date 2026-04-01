# sankhya

**sankhya** (Sanskrit: सांख्य — enumeration/analysis) is an ancient mathematical systems, historical calendar, and archaeoastronomy library for the [AGNOS](https://github.com/MacCracken/agnosticos) project.

Faithful implementations of computational traditions from eight civilizations, 10+ calendar systems, and archaeoastronomy tools — with cross-civilizational epoch correlation. Every algorithm cites its primary source. No other library in any language covers this scope.

## Modules

| Module | Civilization | Era | Highlights |
|--------|-------------|-----|------------|
| `mayan` | Maya | 2000 BCE+ | Vigesimal (base-20), Long Count calendar, Tzolkin, Haab, Venus tables, Calendar Round search |
| `babylonian` | Babylon | 2000–300 BCE | Sexagesimal (base-60), Saros cycle, Plimpton 322 triples, Heron's sqrt, lunar calendar, cuneiform display |
| `egyptian` | Egypt | 2000–1000 BCE | Unit fractions (Rhind Papyrus), doubling multiplication, stellar decans, Sothic cycle, hieroglyphic display |
| `vedic` | India | 800 BCE+ | Nikhilam multiplication, Sulba Sutra geometry, Katapayadi, Meru Prastara, Devanagari display |
| `chinese` | China | 500 BCE+ | Rod numerals, Chinese Remainder Theorem, magic squares, Sexagenary 60-year cycle, Unicode rod display |
| `greek` | Greece | 600 BCE–300 CE | Golden ratio, Sieve of Eratosthenes, Euclidean GCD, Archimedes' pi, Antikythera, isopsephy |
| `roman` | Rome | 500 BCE+ | Roman numeral conversion (I–MMMCMXCIX), canonical validation, arithmetic |
| `islamic` | Islamic Golden Age | 750–1258 CE | Al-Khwarizmi algebra, Khayyam cubics, completion of the square, Hijri calendar |
| `gregorian` | Global | 1582 CE+ | Proleptic Gregorian calendar, JDN conversion, leap years |
| `coptic` | Egypt/Ethiopia | 284 CE+ | 13-month Alexandrian calendar, Anno Martyrum |
| `persian` | Iran | 622 CE+ | Solar Hijri (Jalaali), 2820-year leap cycle, Nowruz |
| `hebrew` | Israel | 3761 BCE+ | Lunisolar, Metonic cycle, molad, dehiyyot, 6 year types |
| `aztec` | Mesoamerica | 1200–1521 CE | Tonalpohualli (260-day), Xiuhpohualli (365-day), Calendar Round |
| `epoch` | Cross-civilizational | — | Precession, precessional ages, Seven Sages, cycle alignment, unified `convert()` API |
| `astro` | Archaeoastronomy | — | Coordinate systems, star catalog, precession, heliacal rising, monument alignment |

## Quick Start

```rust
use sankhya::mayan::LongCount;
use sankhya::babylonian;
use sankhya::greek;
use sankhya::roman;

// Convert days to Mayan Long Count
let lc = LongCount::from_days(1_872_000).unwrap(); // 13.0.0.0.0 (Dec 21, 2012)
assert_eq!(lc.baktun, 13);

// Babylonian square root (Heron's method, from YBC 7289 tablet)
let sqrt2 = babylonian::babylonian_sqrt(2.0, 10).unwrap();
assert!((sqrt2 - std::f64::consts::SQRT_2).abs() < 1e-15);

// Sieve of Eratosthenes
let primes = greek::sieve(100);
assert_eq!(primes.len(), 25);

// Roman numerals
assert_eq!(roman::to_roman_str(1776).unwrap(), "MDCCLXXVI");
assert_eq!(roman::from_roman("MCMXCIX").unwrap(), 1999);
```

### Calendar Conversion

```rust
use sankhya::epoch::{CalendarDate, convert};
use sankhya::gregorian::{GregorianDate, GregorianMonth};

// Convert any date to every calendar system sankhya knows
let date = GregorianDate { year: 2025, month: GregorianMonth::April, day: 1 };
let all = convert(&CalendarDate::Gregorian(date)).unwrap();

println!("Hebrew: {}", all.hebrew);       // 3 Nisan 5785 AM
println!("Persian: {}", all.persian);     // 12 Farvardin 1404 AP
println!("Coptic: {}", all.coptic);       // 23 Paremhat 1741 AM
println!("Aztec: {}", all.aztec_tonalpohualli); // Tonalpohualli date
```

### Archaeoastronomy

```rust
use sankhya::astro;

// Was Stonehenge aligned to the summer solstice?
let results = astro::monument_alignment(51.18, 51.0, 2_451_545.0, 5.0);
// Returns: SummerSolsticeSunrise match at ~50° azimuth

// Where was Thuban (the pyramid builders' pole star) in 2800 BCE?
let pos = astro::star_position_at(astro::StarName::Thuban, 2_451_545.0 - 4800.0 * 365.25);
// Returns: Dec > 85° (near the celestial pole)
```

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| `avatara` | off | Saptarishi archetype profiles on `MultiCalendarDate` via avatara divine engine |
| `itihas` | off | Historical context: eras, civilizations, and events on `MultiCalendarDate` |
| `varna` | off | Script rendering: cuneiform, hieroglyphic, Devanagari, Greek alphabetic, CJK rod numerals |
| `logging` | off | Structured tracing via `tracing-subscriber` |

## Building

```bash
cargo build
cargo test --all-features
make check   # fmt + clippy + test + audit
make bench   # criterion benchmarks with history tracking
```

## Requirements

- Rust 1.89+ (MSRV)
- Dependencies: hisab, serde, thiserror, tracing
- Optional: avatara (archetype profiles), itihas (historical context), varna (script rendering)

## License

GPL-3.0-only. See [LICENSE](LICENSE).
