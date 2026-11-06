# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [3.0.3] — 2026-11-05

**cyrius 6.6.11 → 6.6.20, with the sibling deps at the tags that pin it. Sidereal time and the
obliquity of the ecliptic now match Rust bit for bit.** 32 suites and `tests/foundation.tcyr`:
1,368 assertions, 0 failed.

### Changed

- **Toolchain pin `cyrius` 6.6.11 → 6.6.20.** Nothing in 6.6.12–6.6.20 turned into a compile
  error here: no declared `sizeof` / `mulh64` / `fncallN` / `f64_le` / `f64_ge` / `f64_trunc`, and
  no function-like `#define`. The source changes below remove warnings.
- **Sibling deps move to the tags that pin 6.6.20:** varna 2.4.1 → 2.4.3, itihas 2.5.0 → 2.5.2,
  avatara 2.14.8 → 2.15.1. These are not the newest tags; varna 2.5.x, itihas 2.6.x and avatara
  2.16.x pin the 6.7 line. Each `lib/*.cyr` is byte-equal to its tag's `dist/` bundle.
- **`cyrius.lock` regenerated from an empty `lib/`** (42 deps, 3 commit pins, a `cyrius 6.6.20`
  row). A clean checkout leaves it byte-identical, and `cyrius deps --verify` reports 42 verified.
- **`dist/sankhya.cyr` opens with a compile-verified `# Requires` block, and `dist/sankhya.deps`
  shrinks from 20 stdlib leaves to 2 (`math`, `ganita`).** Both are cyrius 6.6.18+ `distlib`
  output. The sidecar now names only what the bundle needs beyond its named deps; the rest arrive
  through the varna, itihas and avatara sidecars.
  ⚠ **Consumer note:** a project that calls a stdlib family itself and got it only through
  sankhya's old sidecar must now declare it in its own `[deps] stdlib`. The CLI prints a `hint:`
  naming the leaf. A scratch consumer with a bare `[deps.sankhya]` and no stdlib list resolves,
  compiles and runs.
- **`bayan_json_v_obj_get` → `bayan_json_v_obj_get_by_cstr`** at all 142 call sites (14 serial
  modules and `tests/tcyr/integration.tcyr`). bayan deprecated the old name, which added 140
  warnings to every build. Every key is a literal and the old name is a one-line wrapper around
  the new one, so behaviour is unchanged.
- **`f64_tan` is gone from `f64_util`; ganita's is used.** ganita 1.2.11 began exporting `f64_tan`,
  and from cyrius 6.6.20 a redefined fn binds every call to its last definition, so a consumer's
  include order would have chosen which tan ran. The module's rule is that only what neither the
  stdlib nor ganita provides lives there. `f64_tan` still resolves for consumers, because `ganita`
  is in the sidecar. ganita's is an fdlibm port and closer to Rust's libm-backed `f64::tan`: over
  17,999 angles in (−90°, 90°) it is exact against libm 96.03 % of the time with a worst case of
  1 ulp, against 67.79 % and 2 ulp for `sin / cos`.
- **CI, the release workflow and the docs run `cyrius lib sync` before `cyrius deps`**, the order
  the cyrius migration guide asks for from 6.6.17. Here both orders produce a byte-identical lock.
- **The documented dep versions are re-derived from `cyrius.lock`.** `README.md`,
  `docs/development/state.md` and `docs/architecture/overview.md` still named varna 2.1.0, itihas
  2.4.0 and avatara 2.9.0. ADR-006 now points at the manifest instead of naming tags. The roadmap's
  two "Moving the cyrius pin" sections described moves already made and are removed.

### Fixed

- **`obliquity_of_ecliptic` and `greenwich_sidereal_time` match Rust bit for bit.** Their constants
  come from `f64_parse`, which is correctly rounded since cyrius 6.6.13; before that some parsed one
  ulp off. Recomputing the `rust-old/` formulas in IEEE doubles at 40 dates spaced 18,251 days apart,
  including Rust's exact `%`, matches `obliquity_of_ecliptic` at 40 of 40 (0 of 40 before) and
  `greenwich_sidereal_time` at 40 of 40 (3 of 40 before). The sidereal-time error reached
  3.97e-9 hours, about 6e-8°, because `GMST_B` is multiplied by the day count. `greek_phi()` moves
  one ulp to `1.618033988749895`, the value the Rust literal parses to.
  ⚠ **Golden numbers:** values derived from these change too. `equatorial_to_horizontal` azimuth and
  altitude moved by up to about 1.2e-7° in the dates tested, and ecliptic and equatorial conversions
  by a few ulp (more near zero). Those paths use trig, so they were not checked bit for bit against
  Rust. Calendar and numeral output did not change: a bit-level dump under both pins is identical
  for `correlate` at 42 JDNs (full JSON, including itihas and avatara context) and for every
  cuneiform, hieroglyphic and Greek numeral sampled. JSON float text changes only where the value did.

### Performance

- **Flat, with one row faster.** Interleaved A/B over 9 rounds against a 6.6.11 control built from
  3.0.2, comparing medians: suite total −1.4 %, and every row within its ±3–5 % round-to-round
  spread except `epoch/correlate` at −13.8 %. That gain is not attributed.
- **`bench-history.csv` has its first rows:** the 6.6.11 control (`6.6.11@e8df3fa`) and this release.
  This release's 18 rows carry `e8df3fa` because they were recorded before the 3.0.3 commit.
  `benchmarks.md` is generated from them. Compare releases with the A/B above, not these
  single-sample rows.

## [3.0.2] — 2026-09-30

**cyrius 6.6.2 → 6.6.11; the CI bench is built with `CYRIUS_DCE=1` again.** No source change.

### Changed

- **Toolchain pin `cyrius` 6.6.2 → 6.6.11.** `cyrius.lock` re-locked (42 deps, the three
  sibling deps still commit-pinned at varna 2.4.1, itihas 2.5.0 and avatara 2.14.8). The
  re-vendor also picks up the aarch64 syscall peer's `SYS_UNLINKAT` 35 → 263 correction; a
  peer vendored before it made `sys_unlink` issue nanosleep on aarch64. Nothing in the
  6.6.3–6.6.11 releases turned into a compile error here: the tests, bench and `cyrius
  distlib` all build unchanged.
- `dist/sankhya.cyr` regenerated (version header only); `dist/sankhya.deps` unchanged.
- **The version and pin strings name 3.0.2 and 6.6.11.** `src/main.cyr` printed `sankhya 3.0.0`
  through 3.0.1, and `CLAUDE.md`, `README.md`, `CONTRIBUTING.md`, `docs/development/state.md`,
  `docs/development/roadmap.md` and `docs/guides/getting-started.md` still named 3.0.0 and the
  6.4.69 pin. The *Smoke test* CI step now requires the binary to print exactly
  `sankhya <VERSION> (cyrius port)`, so the literal cannot fall behind `VERSION` again.

### Fixed

- **The *Benchmarks* CI step builds with `CYRIUS_DCE=1` again**, and the workaround comment is
  gone. 3.0.1 dropped DCE there because cycc 6.6.2 emitted a PT_LOAD the code was never
  patched against: DCE shrank the code after the data segment's address was fixed, so global
  initialisation stored into unmapped memory and the DCE bench died with rc 139 and no
  output. cyrius 6.6.3 fixed it, and at the 6.6.11 pin the DCE bench runs to completion with
  rc 0 and the same output shape as the non-DCE build.

## [3.0.1] — 2026-09-11

**Migrated to the cyrius 6.6.x value form.** 34/34 test files, 0 failures.

### Changed — cyrius pin 6.4.69 → **6.6.2**

cyrius 6.6.0 flipped `Result` / `Option` / `Either` declared `: stack` to a value form — a payload
variant returns a `(tag, payload)` REGISTER PAIR and allocates nothing; `payload()` is gone. 6.6.2
is the repair release.

⚠ **Almost the entire migration surface lived in the test tree, not the build.** `src/main.cyr`
includes none of sankhya's 37 source files — the library is only ever compiled *through* its tests.
So `cyrius build` reported **zero errors** while 33 of 34 test files would not compile. Anyone
checking the build alone would have called this repo already migrated.

- `src/` — 9 declarations, **7 of them the propagation trap**: `if (is_err_result(r) == 1)
  { return r; }` returns the payload alone under the value form, so `Err` arrives as
  `is_err_result == 0` — an error that reads as SUCCESS. Highest trap density of the sweep.
  Affected `astro`, `babylonian`, `egyptian`, `epoch`, `persian`, `roman`.
- `tests/` — **209** declarations across 36 files.
- `tests/sankhya.bcyr` — the warm block reassigned `warm = correlate(...)` with no `var`. There is
  no `t, v = f();` reassignment form, so each pair-returning warm call now takes its own bind.

### ⚠ BREAKING — Result-taking functions take both halves

A `Result` passed as a *parameter* is two registers now:

```
- fn sk_result_code(r)              + fn sk_result_code(r_tag, r)
```

21 in-repo callers updated; **zero external consumers**, so nothing downstream moves. Three test
helpers changed the same way — `err_or_finite`, `roots_all_finite`, `khroots_all_finite`.

### Fixed — `char_value` collided with varna

sankhya's `char_value(ch)` (Roman numeral → value) and varna's
`char_value(script_code, system, ch)` (gematria) are different functions sharing a name. Both were
in scope; before 6.6.2 that was a silent "last definition wins", so calls to whichever lost
mis-bound their arguments. It surfaced when `varna` moved 2.1.0 → 2.4.1, because 6.6.2 makes a
same-name different-arity duplicate a hard error. sankhya's side moved to `roman_char_value` —
2 callers here, none elsewhere in the ecosystem.

### Changed — dependency pins

| dep | from | to |
|---|---|---|
| `varna` | 2.1.0 | **2.4.1** |
| `itihas` | 2.4.0 | **2.5.0** |
| `avatara` | 2.9.0 | **2.14.8** |

### Worked around — `CYRIUS_DCE=1` segfaults the benchmark binary

`CYRIUS_DCE=1 cyrius build tests/sankhya.bcyr` produces a binary that segfaults immediately
(`rc=139`, zero output). The identical source built without DCE runs clean; DCE removes ~74 % of
that binary. sankhya's **main** binary is built with `CYRIUS_DCE=1` and is unaffected — this is
specific to the bench translation unit.

This is a **cyrius defect, not a sankhya one**, filed upstream as
`docs/development/issues/sankhya-dce-bench-segfault.md`.

**Workaround applied:** the CI Benchmarks step drops `CYRIUS_DCE=1` and builds the bench plainly.
The step is green again and the benchmark numbers are unaffected — DCE is a size optimisation, not
a timing one. The flag is to be **restored** once the upstream fix lands; the workaround carries an
inline comment saying so, and cyrius's roadmap backlog carries the matching follow-up.

⚠ Ruled out during diagnosis, recorded so it is not re-chased: the `itihas` 2.4.0 → 2.5.0 bump
introduced a genuine new `xalloc` duplicate against `avatara`'s (same arity, "last definition
wins"). Pinning `itihas` back removes the warning entirely and the DCE build **still** segfaults,
so the collision is not the cause.

## [3.0.0] — 2026-07-21

Full language port — the entire crate is rewritten from Rust v2.0.0 to Cyrius
(toolchain 6.4.69). 9,857 lines of Rust across 17 files → 10,733 lines of
Cyrius across 37 `src/*.cyr` files. The original Rust source is preserved
verbatim in `rust-old/` for reference. All 15 domain modules retain their full
mathematical and calendrical behavior; `cyrius tests tests/tcyr` reports
**32 passed, 0 failed** (~1,346 assertions).

### Added

- **Cyrius toolchain** — `cyrius.cyml` manifest pinned to Cyrius `6.4.69`, with `VERSION` (`3.0.0`) as the single source of truth, derived into the manifest via `${file:VERSION}`. `math` + `ganita` (folded stdlib) supply the f64/extended-math primitives the Rust build got from `std`/`libm`
- **dist/sankhya.cyr** — single-file distribution bundle generated by `cyrius distlib`, letting consumers (hisab, …) depend on sankhya without vendoring its source tree
- **serial layer** — hand-written, round-trip JSON serialization across a per-module `serial_*.cyr` set. Cyrius has no serde derive, so every public type gets an explicit `<type>_to_json()` / `<type>_from_json()` pair supporting both serialize **and** deserialize. 260 round-trip assertions verify parity
- **tests/sankhya.bcyr** — 18-benchmark suite (`cyrius bench tests/sankhya.bcyr`): all 16 Rust criterion benchmarks ported plus new `coptic` and `aztec` benchmarks
- **adversarial never-panic suite** — 159 assertions of hostile / malformed input across all modules, verifying every public function returns a `Result` error (`sk_fail`) rather than panicking or aborting
- **32 `.tcyr` test suites** under `tests/tcyr/` (plus `tests/foundation.tcyr`), ~1,346 assertions total: 729 per-module, 260 serial round-trip, 198 integration, 159 adversarial/never-panic, 22 foundation. Run with `cyrius tests tests/tcyr`
- **src/lib.cyr** — library aggregator (public API surface) driving the `[lib]` distlib module list; **src/main.cyr** — lean CLI smoke test

### Changed

- **All 15 modules ported to Cyrius idioms** — `mayan`, `babylonian`, `egyptian`, `vedic`, `chinese`, `greek`, `roman`, `islamic`, `gregorian`, `coptic`, `persian`, `hebrew`, `aztec`, `astro`, `epoch` rewritten around i64 arithmetic, offset-enum heap records with accessor functions, `Result` via `Ok` / `sk_fail`, `rem_euclid` / `div_euclid` for calendar modulo, f64 via `f64_from` / `f64_parse`, and the real `#must_use` attribute (Cyrius bare `#attr`, not Rust's no-op `#[attr]`). Foundation modules (`error`, `util`, `f64_util`, `logging`) underpin all of them
- **serde → bayan** — JSON serialization/deserialization now runs through `bayan` (folded stdlib), replacing `serde` / `serde_json`. Full round-trip is retained via the hand-written serial layer
- **tracing → sakshi** — structured logging now runs through `sakshi` (folded stdlib), replacing `tracing`
- **thiserror → code+message error record** — `SankhyaError` is now a `code` + `message` record raised via `sk_fail`, replacing the `thiserror` derive; all fallible functions still return `Result<T>`
- **varna / itihas / avatara now always-on deps** — these git-tagged sibling dependencies (`varna 2.1.0`, `itihas 2.4.0`, `avatara 2.9.0`) were **optional Cargo features** in the Rust crate. Cyrius has no feature-flag system yet, so they are compiled in unconditionally, at **full functional parity** — nothing that was feature-gated is dropped. `net` + `http` come in transitively to satisfy itihas's bundled hoosh HTTP client. Re-introducing true optional/feature gating is a **backlog** item pending Cyrius support

### Breaking

The crate is no longer consumed as a Rust library. Downstream code must migrate
to the Cyrius distribution bundle and its C-style API.

- **Rust crate → Cyrius dist bundle.** Consumers no longer add a `[dependencies] sankhya` crate; they include the single-file bundle `dist/sankhya.cyr` (produced by `cyrius distlib`) and resolve `bayan` / `sakshi` / `math` / `ganita` (and transitively `net` / `http`) from their own `[deps].stdlib`. The Rust toolchain files (`Cargo.toml`, `deny.toml`, etc.) live under `rust-old/`
- **API is C-style free functions, not methods.** Types are Cyrius heap records read through accessor functions rather than Rust structs with methods; `foo.to_long_count()` becomes a free call such as `long_count_from(foo)`. There are no `Display` / `Ord` / `Serialize` / `Deserialize` trait impls — use the module `*_to_json()` / `*_from_json()` functions for JSON and explicit comparison helpers for ordering
- **Errors are code+message records.** `Result<T, SankhyaError>` is now Cyrius `Result<T>` carrying a `code` + `message` record; match on the integer error `code` rather than on `thiserror` variants
- **Optional features are always-on.** The Rust `varna` / `itihas` / `avatara` feature flags no longer exist — their functionality is always present and cannot be opted out of. Builds that previously compiled without these features get the full feature set (and their transitive `net` / `http` deps)

**Migration guide**

1. Drop the `sankhya` crate dependency; `include "dist/sankhya.cyr"` and declare `bayan`, `sakshi`, `math`, `ganita` under `[deps].stdlib` (with `net` + `http` for the hoosh path).
2. Rewrite method calls as free-function calls and replace field access with the generated accessor functions.
3. Replace `serde` (de)serialization with the per-type `*_to_json()` / `*_from_json()` functions.
4. Replace `SankhyaError` variant matching with checks against the returned error `code`; treat any non-`Ok` result as the failure path.
5. Remove any Cargo `features = [...]` selection for `varna` / `itihas` / `avatara` — those integrations are now unconditional.

## [2.0.0] — 2026-04-01

### Breaking

- **epoch** — `MultiCalendarDate` struct gained 7 new fields: `gregorian`, `coptic`, `persian`, `hebrew`, `chinese_sexagenary`, `aztec_tonalpohualli`, `aztec_xiuhpohualli`. Code destructuring this struct must be updated

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
