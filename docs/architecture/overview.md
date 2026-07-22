# Architecture Overview

> **Sankhya** (Sanskrit: enumeration/analysis) — ancient mathematical systems,
> historical calendars, and archaeoastronomy for AGNOS. Ported from Rust
> v2.0.0 to a Cyrius library (toolchain `6.4.69`).

## Module Map

```
sankhya/
├── src/                         — 10,733 lines across 37 .cyr files
│   ├── main.cyr                 — binary entry point / smoke test (does NOT include the library)
│   ├── lib.cyr                  — library aggregator (the ONLY src file with `include` lines; drives distlib)
│   │
│   │  ── foundation ──
│   ├── error.cyr                — SankhyaError codes + SkErr{code,msg} Result record (was thiserror enum)
│   ├── util.cyr                 — shared string/print/number helpers
│   ├── f64_util.cyr             — f64 construction/formatting helpers over math/ganita
│   ├── logging.cyr              — sakshi logging init (sankhya_log_init) — replaces tracing
│   │
│   │  ── 15 domain modules ──
│   ├── mayan.cyr                — Vigesimal, Long Count, Tzolkin, Haab, Calendar Round search, Venus tables
│   ├── babylonian.cyr           — Sexagesimal, Saros cycle, reciprocals, Plimpton 322, Heron's sqrt
│   ├── egyptian.cyr             — Unit fractions, doubling multiplication, division, decans, Sothic cycle
│   ├── vedic.cyr                — Nikhilam multiplication, Sulba Sutra, Katapayadi, Meru Prastara
│   ├── chinese.cyr              — Rod numerals, Chinese Remainder Theorem, magic squares, Sexagenary cycle
│   ├── greek.cyr                — Golden ratio, sieve, GCD/LCM, Archimedes' pi, Antikythera, isopsephy
│   ├── roman.cyr                — Roman numeral conversion, validation, arithmetic (I–MMMCMXCIX)
│   ├── islamic.cyr              — Al-Khwarizmi algebra, Khayyam cubics, completion of the square, Hijri calendar
│   ├── gregorian.cyr            — Proleptic Gregorian calendar, JDN conversion, leap years
│   ├── coptic.cyr               — 13-month Alexandrian calendar, Anno Martyrum
│   ├── persian.cyr              — Solar Hijri (Jalaali), 2820-year leap cycle, Nowruz
│   ├── hebrew.cyr               — Lunisolar, Metonic cycle, molad, dehiyyot, 6 year types
│   ├── aztec.cyr                — Tonalpohualli (260-day), Xiuhpohualli (365-day), Calendar Round
│   ├── astro.cyr                — Coordinate systems, star catalog, precession, heliacal rising, alignment
│   └── epoch.cyr                — Precession, precessional ages, Seven Sages, cycle alignment, unified convert()
│   │
│   │  ── serial layer (one JSON module per domain module) ──
│   ├── serial_util.cyr          — sk_json_* builders + bayan parse entry (sk_json_parse)
│   ├── serial_gregorian.cyr     serial_roman.cyr     serial_greek.cyr    serial_vedic.cyr
│   ├── serial_mayan.cyr         serial_aztec.cyr     serial_coptic.cyr   serial_egyptian.cyr
│   ├── serial_babylonian.cyr    serial_hebrew.cyr    serial_islamic.cyr  serial_astro.cyr
│   └── serial_chinese.cyr       serial_persian.cyr   serial_epoch.cyr
│
├── tests/
│   ├── tcyr/                    — 32 .tcyr suites, ~1,346 assertions (cyrius tests tests/tcyr)
│   ├── foundation.tcyr          — 22 foundation assertions
│   └── sankhya.bcyr             — 18-benchmark suite (cyrius bench tests/sankhya.bcyr)
├── dist/
│   └── sankhya.cyr              — single-file consumer bundle (cyrius distlib)
├── build/                       — compiled artifacts (gitignored, regenerated)
└── rust-old/                    — preserved Rust v2.0.0 source (9,857 lines, 17 files — frozen reference oracle)
```

The `[lib].modules` list in `cyrius.cyml` fixes the dependency order for the
bundle: foundation → domain modules → serial layer → `lib.cyr` (include-only entry).
`lib.cyr` is the only source file that carries `include` lines; `cyrius distlib`
strips them so the bundle is a flat, self-resolving concatenation.

## Data Flow

Sankhya is a pure computation library — no I/O, all functions synchronous and
deterministic. A caller feeds a date, day count, or integer in one representation
and reads back the same value in another. The `epoch` module is the fan-in point:
its `convert()` takes any supported `CalendarDate`, routes it through a Julian Day
Number, and correlates it across every system at once.

```
Integer / day count / JDN / CalendarDate
  │
  ├─→ mayan        long_count_from_julian_day(jdn), tzolkin_from_days(d), haab_from_days(d), venus_phase(d)
  ├─→ babylonian   to_sexagesimal(n), babylonian_sqrt(n, iters), generate_plimpton_triples(), saros_cycle(jdn)
  ├─→ egyptian     decompose(num, den), egyptian_multiply(a, b), sothic helpers
  ├─→ vedic        vedic_multiply_nikhilam(a, b), sulba_sqrt2(), katapayadi_*
  ├─→ chinese      chinese_remainder(pairs), rod numerals, sexagenary cycle
  ├─→ greek        sieve(limit), gcd(a,b), lcm(a,b), archimedes_pi(iters), antikythera_gear_ratios()
  ├─→ roman        to_roman_str(n), from_roman(s), roman_add/subtract/multiply/divide, is_valid_roman(s)
  ├─→ islamic      al-Khwarizmi/Khayyam solvers, hijri_to_jdn(date)
  ├─→ gregorian    gregorian_date_new(y,m,d), gregorian_to_jdn(date), gregorian_is_leap(y)
  ├─→ coptic       coptic_to_jdn(date), Anno Martyrum helpers
  ├─→ persian      persian_to_jdn(date), Jalaali leap cycle
  ├─→ hebrew       hebrew_to_jdn(date), molad / dehiyyot
  ├─→ aztec        tonalpohualli / xiuhpohualli from day counts
  ├─→ astro        coordinate transforms, star catalog, heliacal rising, monument alignment
  └─→ epoch        calendar_to_jdn(date) ─→ correlate(jdn) ─→ convert(date)  ← unified any-to-any
```

Every domain type has a matching `serial_*` module providing
`<type>_to_json(p)` → `Str` and `<type>_from_json(v)` → pointer, plus a
`<type>_eq(a, b)` used by the round-trip suites. Serialization is the only
"output" path; it is entirely in-memory (build a JSON string, parse it back).

## Module Capabilities

| Group | Modules | Representative API |
|-------|---------|--------------------|
| Ancient numerals & math | mayan, babylonian, egyptian, vedic, chinese, greek, roman, islamic | vigesimal/sexagesimal conversion, Heron's sqrt, unit-fraction decomposition, Nikhilam, CRT, sieve, Roman arithmetic, cubic solvers |
| Calendar systems | gregorian, coptic, persian, hebrew, mayan (Long Count), aztec, islamic (Hijri) | `*_to_jdn` / `*_from_jdn`, leap-year rules, cycle math |
| Archaeoastronomy | astro, epoch | precession, heliacal rising, monument alignment, Seven Sages, cycle alignment |
| Cross-calendar bridge | epoch | `convert()` — any supported date → JDN → all systems (`MultiCalendarDate`) |

## Idioms (Rust → Cyrius port)

Three patterns recur across every module and are the load-bearing translation of
the Rust type system into Cyrius:

- **Offset-enum records.** Rust structs become heap records allocated with
  `alloc(SIZE)` and addressed by `store64(p + OFFSET, …)` / `load64(p + OFFSET)`.
  Field offsets are `enum` constants, e.g.
  `enum RomanOff { RN_VALUE = 0; RN_TEXT = 8; RN_SIZE = 16; }`, with thin accessor
  functions (`fn romannum_value(p) { return load64(p + RN_VALUE); }`). Enum
  members also carry plain integer constants with no global-init cost
  (`enum MayanConst { EPOCH_JDN = 584283; CALENDAR_ROUND_DAYS = 18980; }`).
- **Lazy-init globals.** Float constants and parsed tables can't be `enum`
  members, so they are computed once on first access and cached behind a
  `_done` flag — e.g. `greek_phi()` parses `"1.618033988749895"` via `f64_parse`
  on the first call, then returns the cached global. Data tables
  (reciprocal table, star catalog) follow the same lazy-init-and-cache shape.
- **Result-as-heap-record.** Rust's `Result<T, SankhyaError>` (where every
  variant carried a `format!` message) becomes `Ok(value)` / `sk_fail(code, msg)`.
  `sk_fail` wraps a 16-byte `SkErr{code, msg}` record in `Err`; callers that only
  care about the variant read `sk_result_code(r)`, callers that want the text read
  `sk_err_msg(...)`. Both the discriminant **and** the interpolated message survive
  the port.

Other Cyrius specifics carried over from the port:

- No serde derive — every type has a hand-written `<type>_to_json` /
  `<type>_from_json` in its `serial_*` module (backed by `bayan`).
- Calendar modulo uses `rem_euclid` / `div_euclid` (truncated `%` / `/` only
  where Rust operands are provably non-negative `u64`).
- Floats flow through `f64_from` / `f64_parse` and the `math` / `ganita` helpers,
  never native float literals.
- Attributes are bare `#attr` (e.g. `#must_use`), **not** Rust `#[attr]`.

## Dependency Stack

Folded stdlib (auto-included at compile time via `[deps].stdlib`):

| Dep | Role | Replaces (Rust) |
|-----|------|-----------------|
| `bayan` | JSON serialize + parse — full round-trip | serde / serde_json |
| `sakshi` | structured logging (`sankhya_log_init`) | tracing |
| `math` | f64 constants + `f64_parse/min/max/clamp`, gcd/lcm | std / core math |
| `ganita` | extended math: `f64_pow/hypot`, `asin/acos/atan2/sinh/cosh` (astro) | libm-style routines |
| `string`, `fmt`, `alloc`, `vec`, `str`, `syscalls`, `io`, `args` | core runtime | — |
| `assert` | test harness | — |
| `result`, `tagged` | `Result<T,E>` + `?` propagation | Rust `Result` |
| `fnptr` | function pointers (vec_sort comparators) | — |
| `hashmap` | hash map (required by varna + avatara bundle internals) | std collections |
| `bench` | benchmark harness | criterion |
| `net`, `http` | transitive — satisfy itihas's bundled hoosh HTTP client symbols (never called here) | — |

First-party sibling deps (git-tagged, **always-on** — see below):

| Dep | Tag | Purpose |
|-----|-----|---------|
| `varna` | 2.1.0 | script rendering (cuneiform, hieroglyphic, Devanagari, Greek, rod numerals) — wave-1/2 numeral modules |
| `itihas` | 2.4.0 | historical context (eras, civilizations, events) on the epoch capstone |
| `avatara` | 2.9.0 | Saptarishi archetype profiles on the epoch capstone |

> **Always-on note.** The Rust crate gated `varna` / `itihas` / `avatara` behind
> optional Cargo features. Cyrius has no feature-flag system yet, so these three
> siblings are **always compiled in** — full functional parity, nothing dropped.
> Re-introducing true optional/feature gating is a backlog item pending Cyrius
> support (see [roadmap](../development/roadmap.md)).

## Downstream Consumers

```
sankhya
  └─→ hisab      — higher mathematics (uses sankhya as an optional companion for
                   historical/ancient computations, calendar systems, number-system
                   conversions)
```

Consumers include a single file — `dist/sankhya.cyr`, produced by `cyrius distlib`
— and resolve stdlib/sibling deps from their own `[deps]`.

## Architecture Decision Records

The Rust-era ADRs remain valid and compiler-agnostic (they describe algorithm and
constant choices, not the Rust surface):

- [ADR-001](../adr/001-unit-fraction-algorithm.md) — Egyptian fraction decomposition (greedy algorithm)
- [ADR-002](../adr/002-gmt-correlation-constant.md) — GMT correlation constant (JDN 584,283)
- [ADR-003](../adr/003-canonical-precession-period.md) — Canonical precession period (25,920 years)
- [ADR-004](../adr/004-seven-sages-data-model.md) — Seven Sages data model
- [ADR-005](../adr/005-feature-gated-script-rendering.md) — Feature-gated script rendering via varna (**superseded by [ADR-006](../adr/006-always-on-dependencies.md)**)

> ADR-005's *feature gating* is the design the port cannot yet honor 1:1 — varna
> is always-on under Cyrius. The algorithmic decision it records (script rendering
> delegated to varna) is unchanged.

## Design Principles

- **Historically accurate** — real formulas, real correlations; the Rust source in
  `rust-old/` is the parity oracle.
- **Pure & deterministic** — no I/O, no global mutation beyond lazy-init caches.
- **Every type round-trips** — a `serial_*` module for each domain module, verified
  by the 260-assertion round-trip suite.
- **Zero unwrap/panic** — all fallible paths return `Result` via `Ok` / `sk_fail`;
  the adversarial suite (159 assertions) proves the never-panic contract.
- **Composable** — each domain module is independent; `epoch` is the only fan-in.
- **Single-file distribution** — `dist/sankhya.cyr` is the whole public surface.
