# Sankhya — Claude Code Instructions

## Project Identity

**Sankhya** (Sanskrit: सांख्य — enumeration/analysis) — Ancient mathematical systems,
historical calendars, and archaeoastronomy for AGNOS.

- **Type**: Cyrius library (ported from Rust v2.0.0 — original preserved at `rust-old/`)
- **License**: GPL-3.0-only
- **Compiler**: Cyrius 6.4.69 — pinned in `cyrius.cyml` `[package].cyrius`
- **Version**: SemVer 3.0.0 — `VERSION` at the project root is the source of truth
  (`cyrius.cyml` derives it via `${file:VERSION}`); do not inline the number elsewhere
- **Consumers**: hisab (higher mathematics, as an optional companion), and the wider AGNOS
  ecosystem needing historical/ancient computations, calendar systems, or archaeoastronomy

## Modules

| Module | Contents |
|--------|----------|
| `error` | `SankhyaError` codes + `SkErr {code, msg}` record (message-preserving), `sk_fail`/`sk_result_code` |
| `util` | Euclidean `rem_euclid`/`div_euclid`/`int_abs`, string case helpers, `vec_sort` |
| `f64_util` | `f64_tan`/`f64_fmod`/`f64_approx_eq`/NaN·Inf guards, shared `J2000_JDN`/`JULIAN_YEAR_DAYS` |
| `mayan` | Vigesimal numbers, Long Count, Tzolkin, Haab, Calendar Round, Venus tables |
| `babylonian` | Sexagesimal numbers, Saros cycle, reciprocals, Plimpton 322, Heron's sqrt |
| `egyptian` | Unit fractions, doubling multiplication, division, stellar decans, Sothic cycle |
| `vedic` | Nikhilam multiplication, Sulba Sutra, Katapayadi, Meru Prastara |
| `chinese` | Rod numerals, Chinese Remainder Theorem, magic squares, Sexagenary cycle |
| `greek` | Golden ratio, sieve, GCD/LCM, Archimedes' pi, Antikythera, isopsephy |
| `roman` | Roman numeral conversion, validation, arithmetic (I–MMMCMXCIX) |
| `islamic` | Al-Khwarizmi algebra, Khayyam cubics, completion of the square, Hijri calendar |
| `gregorian` | Proleptic Gregorian calendar, JDN conversion, leap years |
| `coptic` | 13-month Alexandrian calendar, Anno Martyrum |
| `persian` | Solar Hijri (Jalaali), 2820-year leap cycle, Nowruz |
| `hebrew` | Lunisolar, Metonic cycle, molad, dehiyyot, 6 year types |
| `aztec` | Tonalpohualli (260-day), Xiuhpohualli (365-day), Calendar Round |
| `astro` | Coordinate systems, star catalog, precession, heliacal rising, monument alignment |
| `epoch` | Precession, precessional ages, Seven Sages, cycle alignment, unified `convert()`/`correlate()` |
| `logging` | sakshi structured logging init (replaces Rust `tracing`) |
| `serial_*` | Hand-written round-trip JSON (`<type>_to_json` / `<type>_from_json`) via bayan — one per module |

`src/lib.cyr` aggregates all modules (dependency order); `cyrius distlib` bundles them into
`dist/sankhya.cyr` for consumers. `src/main.cyr` is a minimal smoke test only.

## Dependencies

- **bayan** (folded stdlib) — JSON serialize + parse, replacing serde/serde_json (round-trip).
- **sakshi** (folded stdlib) — structured logging, replacing tracing.
- **math** + **ganita** (stdlib) — f64 constants/parse and extended math (pow/hypot/asin/acos/atan2).
- **varna** (git dep, always-on) — Unicode numeral rendering (chinese/egyptian/greek/vedic/babylonian).
- **itihas** (git dep, always-on) — `epoch` historical context (eras/civilizations/events).
- **avatara** (git dep, always-on) — `epoch` Saptarishi archetype profiles.

> The Rust crate gated varna/itihas/avatara behind optional Cargo features. Cyrius has no
> feature-flag system yet, so they are **always-on** dependencies here — full functional parity,
> nothing dropped. Re-introducing true optional/feature gating is a backlog item (see
> `docs/development/roadmap.md`) pending Cyrius support.

## Key Principles

- **All math must be historically accurate** — real formulas, real correlations, cite primary sources.
- **Cross-check against `rust-old/`** — the correctness bar is "matches what the Rust did". The
  original tree is the parity oracle; do not modify it. Diverge only with an ADR.
- **Zero panics** — no `unwrap`; every fallible path returns a `Result` (`Ok`/`sk_fail`) or a
  sentinel; check every fallible stdlib call. The adversarial suite enforces "Err or finite, never NaN/Inf".
- **Euclidean modulo for calendars** — `rem_euclid`/`div_euclid` (not bare `%`/`/`) wherever an
  operand can be negative (pre-epoch dates). The single most common port bug.
- **f64 discipline** — no float literals; `f64_from(int)` / `f64_parse("x.y")`; guard NaN/Inf; keep
  1e-15 tolerances for algebraic paths, loosen to ~1e-12 for transcendentals on aarch64.
- **`#must_use`** on pure functions whose result is a bug to discard.
- **Cyrius attributes use bare `#attr`**, not Rust's `#[attr]` (which is a silent no-op comment).
  Recognized: `#must_use`, `#deprecated("reason")`, `#regalloc N`, `#derive(...)`. `#[non_exhaustive]`
  and `#[inline]` have no Cyrius equivalent — dropped in the port.
- **Self-contained modules** — no `include` lines in any `src/*.cyr` listed in `[lib].modules`
  (the distlib bundler strips them; stdlib/deps resolve from the consumer's `[deps]`). Only
  `lib.cyr` (last) and `.tcyr`/`.bcyr` test files carry includes.
- **Never skip benchmarks** — the CSV history is the proof.

## Toolchain

| Action | Command |
|--------|---------|
| Resolve sibling deps | `cyrius deps` |
| Vendor stdlib | `cyrius lib sync` |
| Build smoke test | `cyrius build src/main.cyr build/sankhya && ./build/sankhya` |
| Run all tests | `cyrius tests tests/tcyr` (each `.tcyr` prints "N passed, 0 failed") |
| Run benchmarks | `cyrius bench tests/sankhya.bcyr` |
| Build dist bundle | `cyrius distlib` → `dist/sankhya.cyr` |
| Lint / format | `cyrius lint <file>` / `cyrius fmt <file> [--check]` |
| Full gate | `cyrius audit` |

**Never run `cargo` / `clippy` / `rustc` / `cargo-audit` / `cargo-deny`** against the project —
those are stale pre-3.0 references. Rust survives only as the `rust-old/` parity oracle.

## Development Process

### P(-1): Scaffold Hardening (before new features)
0. Read roadmap, CHANGELOG, open issues. 1. Test + benchmark sweep. 2. Cleanliness check
(`cyrius build`, `cyrius tests`, `cyrius lint`). 3. Baseline benchmarks. 4. Internal deep review
(gaps, correctness vs `rust-old/`, logging/errors, docs). 5. External research. 6. Cleanliness
check. 7. Tests/benchmarks from findings. 8. Post-review benchmarks. 9. Repeat if heavy.

### Work Loop (continuous)
1. Work phase. 2. Cleanliness check (`cyrius build src/main.cyr build/sankhya && ./build/sankhya`,
`cyrius tests tests/tcyr`). 3. Test + benchmark additions. 4. Run benchmarks. 5. Internal review.
6. Cleanliness check. 7. Deeper tests. 8. Benchmarks again — prove the wins. 9. If review heavy →
step 5. 10. Documentation (CHANGELOG, roadmap, sources, ADRs). 11. Version check (VERSION ↔
`cyrius.cyml` via `${file:VERSION}`). 12. **Release benchmark gate (MANDATORY)** — full suite, record
CSV deltas, confirm no regressions. 13. Return to step 1.

## DO NOT

- **Do not commit or push** — the user handles all git operations.
- **NEVER use `gh` CLI** — use `curl` to the GitHub API only.
- **NEVER bump version** (VERSION / cyrius.cyml) unless the user explicitly says to.
- Do not modify `rust-old/` — it is the parity oracle.
- Do not add unnecessary dependencies.
- Do not use panics/`unwrap` equivalents in library code.
- Do not skip benchmarks before claiming performance improvements.
- Do not commit `build/` or `lib/` (vendored stdlib).
- Do not hardcode toolchain versions in CI YAML — `cyrius = "X.Y.Z"` in `cyrius.cyml` is the source of truth.
