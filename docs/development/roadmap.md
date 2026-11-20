# Development Roadmap

> **Status**: v3.2.0 released (Rust→Cyrius port complete at 3.0.0) | **Current**: 3.2.0 | **Compiler**: cyrius 6.7.18

Completed items are in [CHANGELOG.md](../../CHANGELOG.md).
`VERSION` is the single source of truth (`3.2.0`); `cyrius.cyml` derives it via `${file:VERSION}`.

## Completed in 3.2.0 — Result-returning decoders

- [x] Every `<type>_from_json` (60, all 15 serial modules) returns `Ok(value)` or
      `Err(SKE_DESERIALIZE)` with serde's checks and wording (ADR-009); shared `sk_de_*` helpers
- [x] serde-parity error tests in every `serial_<module>` suite; `serial_malformed` checks the
      contract on every decoder; the fuzz harness checks every decode Err's code and message
- [x] Encoding output byte-identical to 3.1.3 (8,431 dump lines)

## Completed in 3.1.3 — P(-1) sweep

- [x] **All 878 public fns documented** (`cyrdoc --check` 0 undocumented in `src/` and `tests/`;
      the 795 the roadmap carried was a `cyrius audit` miscount, 533 in fact)
- [x] Internal review against `rust-old/` and primary sources: three parallel reviews, every finding
      reproduced before it was fixed
- [x] **Input-domain guards** (ADR-007): no crash, hang or garbage `Ok` from negative counts, bad
      months/days, `i64::MIN / -1`, non-finite f64, or malformed JSON
- [x] **Accuracy corrections** (ADR-008): Hebrew epoch (+1 day), Aztec correlation, Persian last break,
      Hijri pre-epoch years, reciprocal 75, Khayyam classifier, precession (Meeus 21.2), azimuth,
      heliacal side, solar mean anomaly, the Long Count of a calendar date
- [x] Exact `f64_rem` (Rust's `%`): obliquity, GMST and LST match the Rust formulas bit for bit
- [x] Logging opt-in, as in Rust; lazy module init (epoch, babylonian)
- [x] Perf: alloc-free Calendar Round search, cached Devanagari reverse map, Hebrew year estimate and
      month lengths, no eager log formatting
- [x] Tests: `serial_malformed`, `lazy_init`, `usage_examples` suites; a real fuzz harness (CI); 6 benches

## Completed in 3.1.2

- [x] **varna 2.5.2 → 2.6.0**, with `katapayadi_encode_devanagari` fixed for its bare-consonant
      Devanagari table (the lookup works against both the old and new key forms)
- [x] Backlog #2 — **line-length reflow** in `astro`/`epoch`/`mayan`: `cyrius lint` reports 0 warnings
      across `src/` (was 36), and CI's line-length exemption is gone
- [x] `src/gregorian.cyr` fmt-clean; every `src/*.cyr` passes `cyrius fmt --check`, now a CI gate
- [x] The 13 deprecated `f64_asin` / `f64_acos` / `f64_atan2` calls → `ganita_f64_*`
- [x] Output byte-identical to 3.1.1 (two dumps, 2,401 lines), so the 3.0.3 bit-for-bit Rust match holds

## Completed in 3.0.0 — Rust → Cyrius port

The full v2.0.0 Rust crate (9,857 lines, 17 files) is ported to Cyrius and preserved
at `rust-old/` as the parity oracle.

- [x] Port all **15 domain modules** — mayan, babylonian, egyptian, vedic, chinese,
      greek, roman, islamic, gregorian, coptic, persian, hebrew, aztec, astro, epoch
- [x] Foundation modules — `error` (SankhyaError codes + `SkErr{code,msg}` Result record),
      `util`, `f64_util`, `logging` (sakshi, replaces tracing)
- [x] Per-module **serial layer** — one `serial_*.cyr` per domain module with
      hand-written `<type>_to_json` / `<type>_from_json` / `<type>_eq` (replaces serde derive)
- [x] `epoch` capstone — unified `convert()` (any `CalendarDate` → JDN → `MultiCalendarDate`),
      `correlate()`, precessional ages, Seven Sages, cycle alignment
- [x] **10,733 lines across 37 `src/*.cyr` files**; `lib.cyr` aggregator + `main.cyr` smoke test
- [x] Cyrius idiom translation — offset-enum records (`store64`/`load64`), lazy-init
      float/table globals, `Ok`/`sk_fail` Result heap records, `rem_euclid`/`div_euclid`
      calendar modulo, `f64_from`/`f64_parse` floats, bare `#must_use` attributes
- [x] Dependency swap — serde/serde_json → `bayan` (full round-trip), tracing → `sakshi`,
      std/libm math → `math` + `ganita`
- [x] Sibling deps wired — `varna` 2.1.0, `itihas` 2.4.0, `avatara` 2.9.0 (always-on, full
      parity); `net`/`http` resolve itihas's bundled hoosh client transitively
- [x] **32 `.tcyr` suites**, ~1,346 assertions — per-module 729, serial round-trip 260,
      integration 198, adversarial/never-panic 159, foundation 22.
      Run `cyrius test tests/tcyr` → `32 passed, 0 failed`
- [x] **18 benchmarks** in `tests/sankhya.bcyr` — all 16 Rust criterion benches + new
      coptic/aztec. Run `cyrius bench tests/sankhya.bcyr`
- [x] Consumer bundle `dist/sankhya.cyr` via `cyrius distlib`
- [x] `VERSION` → `3.0.0`; `cyrius.cyml` toolchain pinned to `6.4.69`, version via `${file:VERSION}`

## Port Coverage

| Rust module | Cyrius | Serial | Status |
|-------------|--------|--------|--------|
| `mayan.rs` | `mayan.cyr` | `serial_mayan.cyr` | Complete |
| `babylonian.rs` | `babylonian.cyr` | `serial_babylonian.cyr` | Complete |
| `egyptian.rs` | `egyptian.cyr` | `serial_egyptian.cyr` | Complete |
| `vedic.rs` | `vedic.cyr` | `serial_vedic.cyr` | Complete |
| `chinese.rs` | `chinese.cyr` | `serial_chinese.cyr` | Complete |
| `greek.rs` | `greek.cyr` | `serial_greek.cyr` | Complete |
| `roman.rs` | `roman.cyr` | `serial_roman.cyr` | Complete |
| `islamic.rs` | `islamic.cyr` | `serial_islamic.cyr` | Complete |
| `gregorian.rs` | `gregorian.cyr` | `serial_gregorian.cyr` | Complete |
| `coptic.rs` | `coptic.cyr` | `serial_coptic.cyr` | Complete |
| `persian.rs` | `persian.cyr` | `serial_persian.cyr` | Complete |
| `hebrew.rs` | `hebrew.cyr` | `serial_hebrew.cyr` | Complete |
| `aztec.rs` | `aztec.cyr` | `serial_aztec.cyr` | Complete |
| `astro.rs` | `astro.cyr` | `serial_astro.cyr` | Complete |
| `epoch.rs` | `epoch.cyr` | `serial_epoch.cyr` | Complete |
| `error.rs` | `error.cyr` | — | Complete |
| `lib.rs` | `lib.cyr` + `main.cyr` | — | Complete |

## Deferred / Backlog

**Recorded at 3.1.3:**
- **Upstream, cyrius:** `cyrius audit`'s fmt, lint and docs walkers share one directory vec, and each
  appends the subdirectories it descends into, so the docs walk visits `tests/tcyr/` three times and the
  lint walk twice. The "795 undocumented" was 533 (533 + 2 × 131); the "16 lint warnings" in `tests/` were 8.
  Not yet filed with cyrius.
- ~~**API:** `<type>_from_json` returns a record, not a `Result`.~~ **Done in 3.2.0** (ADR-009).
  Decode errors lack serde_json's " at line L column C" suffix, because bayan's value tree keeps
  no positions; `&'static str` fields accept escaped strings, which serde_json's borrowed
  deserialization refuses.
- **Upstream accuracy, models left as the Rust has them** (ADR-008 lists what was corrected):
  - the arcus visionis heliacal model puts Sirius's first sighting about three weeks before the observed
    date;
  - `solar_longitude` uses a two-term equation of centre (Meeus 25.4 has three, with T terms);
  - `to_unicode_rods` uses the vertical (unit) glyphs in every place, where rod numerals alternate
    with the horizontal forms U+1D369–71.
- `MultiCalendarDate` writes 3 of the avatara profile fields for each Saptarishi entry (name, tradition,
  description), where serde wrote the whole profile. Full fidelity needs an avatara serializer, which is
  now in progress (table row #3).
- `sk_f64_display` (error messages) writes |x| >= 1e21 and |x| < 1e-6 in exponent form, where Rust's
  `Display` prints every digit.
- Persian dates outside jalaali-js's -61..3177 AP are extrapolated, as the Rust did; jalaali-js rejects them.

**Recorded at 3.0.3 to 3.1.2 (re-checked 2026-11-18):**
- `cyrius audit` was red through 3.1.2. At 3.1.3 it exits 0 (fmt, lint and docs). CI could now run it as a gate.
- bayan's `lib/bayan.cyr` keeps about 350 KB of Unicode tables that DCE cannot drop, so a DCE binary here is
  282 KB larger than at 3.1.0 (smoke 520,104 → 802,432 B). sankhya uses only bayan's JSON layer, but it cannot
  take the thin `dist/bayan-json.cyr` while itihas's sidecar and `# Requires` block name the monolith; both
  would define the JSON symbols. Once itihas moves, follow it. Still blocked at 3.1.2: itihas 2.6.2 is its
  latest tag.
- `xalloc` is defined by both the itihas and avatara bundles. Since cyrius 6.6.20 every call binds to avatara's,
  so itihas loses only its non-positive-size message; both exit 134. The fix is upstream (a prefixed name).
  Unchanged at 3.1.2 (itihas 2.6.2, avatara 2.16.1 are the latest tags).
- **Recorded at 3.1.2:** varna 2.6.0's `dist/varna.cyr` is 599,119 B of source (346,930 B at 2.5.2), and
  DCE keeps 54,976 B more of it in every binary. sankhya uses its numeral tables and one transliteration
  table. A thinner varna profile, if varna ships one, would win that back.


Post-port work, none of it blocking. Sequenced most-consequential first; each is
its own work-loop cycle behind the cleanliness + benchmark gates.

| # | Item | Effort | Details |
|---|------|--------|---------|
| 1 | **Re-introduce optional/feature-gated deps** | Large | Re-introduce optional/feature-gated deps (`varna`/`itihas`/`avatara`) — **currently always-on for full parity**. The Rust crate gated all three behind optional Cargo features (script rendering, historical context, Saptarishi profiles). Nothing is dropped; this is a packaging/optionality gap, not a functional one. **Re-checked at 3.1.2:** Cyrius has had `optional = true` deps and a `[features]` table since 6.3.1, so the earlier "no feature system" note was wrong. But a feature only decides whether a dep is resolved and vendored; it defines no `#ifdef` symbol, and features are read from the root manifest only (transitive `[features]` tables are not parsed). Gating sankhya's varna/itihas/avatara code would therefore need every consumer to pass a matching `--features` and `-D` by hand, or the bundle would call undefined fns. Still blocked upstream, on a feature → define link and on transitive feature resolution. See ADR-006. |
| 2 | ~~Line-length reflow in `astro`/`epoch`~~ | — | **Done in 3.1.2** (with `mayan`): 0 lint warnings across `src/`, output byte-identical. |
| 3 | **epoch external-context deserialization** | Medium | `epoch`'s `MultiCalendarDate` serializes its itihas eras/civilizations/events and avatara Saptarishi context on the `_to_json` path, but `_from_json` cannot fully reconstruct those sibling records until `itihas` and `avatara` ship `from_json` constructors. Round-trip is complete for the sankhya-native fields; the external-context arrays reconstruct once the upstream `from_json` entry points land. **⚠ Bug in 3.2.0:** the decoder fills those four vecs with raw bayan nodes. Re-encoding a decoded `MultiCalendarDate` whose context is not empty therefore segfaults (exit 139), because `era_to_json` reads a node as an itihas record. **In progress (2026-11-19), upstream first:** itihas gains `era_from_json` / `civ_from_json` / `event_from_json`, and avatara gains `profile_to_json` / `profile_from_json` in a new `dist/avatara-json.cyr` bundle (avatara ADR-011). Both are written but not yet tagged. The sankhya side is written and checked against the local checkouts. Once both are tagged it lands as follows: bump both pins, take `dist/avatara-json.cyr`, decode every element, and write each Saptarishi entry as serde's `[name, profile]` tuple. Every record must then round-trip byte-identically, and the only output change is the `saptarishi_profiles` array. |

### Demand-gated (future)

- Display/formatting parity beyond `<type>_to_str` (the Rust `Display` impls were
  folded into the string accessors already used by the serial layer)
- Additional calendar systems / numeral scripts as consumers ask for them
- Wider varna script coverage once feature gating (backlog #1) lands

## Verification

| Gate | Command | Expected |
|------|---------|----------|
| Tests | `cyrius test tests/tcyr` | `35 passed, 0 failed` (2,133 assertions; `tests/foundation.tcyr` adds 22) |
| Fuzz | `cyrius build tests/sankhya.fcyr build/f && ./build/f` | `fuzz: ok` (20,000 rounds) |
| Benchmarks | `cyrius bench tests/sankhya.bcyr` | 24 benches; recorded via `scripts/bench-history.sh` |
| Smoke test | `cyrius build src/main.cyr build/sankhya` | prints `sankhya 3.2.0 (cyrius port)` |
| Docs | `cyrdoc --check <file>` | 0 undocumented in every `src/` and `tests/` file |
| Lint / format | `cyrius lint <file>` / `cyrius fmt <file> --check` | 0 warnings and clean for every `src/*.cyr` (CI gates both) |
| Bundle | `cyrius distlib` | regenerates `dist/sankhya.cyr` |
