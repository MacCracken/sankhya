# Development Roadmap

> **Status**: v3.1.2 released (Rust→Cyrius port complete at 3.0.0) | **Current**: 3.1.2 | **Compiler**: cyrius 6.7.18

Completed items are in [CHANGELOG.md](../../CHANGELOG.md).
`VERSION` is the single source of truth (`3.1.2`); `cyrius.cyml` derives it via `${file:VERSION}`.

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

**Recorded at 3.0.3 to 3.1.2 (re-checked 2026-11-18):**
- `cyrius audit` exits 1, as it has since at least 3.0.2, and CI does not run it. From 3.1.2 its fmt and lint
  steps pass across `src/` and `tests/`; what keeps it red is the docs step (795 undocumented public fns, the
  same at 3.1.1 and 3.1.2). Documenting them is the remaining work to make `audit` a usable gate. Not blocked.
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
| 3 | **epoch external-context deserialization** | Medium | `epoch`'s `MultiCalendarDate` serializes its itihas eras/civilizations/events and avatara Saptarishi context on the `_to_json` path, but `_from_json` cannot fully reconstruct those sibling records until `itihas` and `avatara` ship `from_json` constructors. Round-trip is complete for the sankhya-native fields; the external-context arrays reconstruct once the upstream `from_json` entry points land. **Still blocked at 3.1.2:** neither itihas 2.6.2 nor avatara 2.16.1 (their latest tags) defines a `*_from_json`. |

### Demand-gated (future)

- Display/formatting parity beyond `<type>_to_str` (the Rust `Display` impls were
  folded into the string accessors already used by the serial layer)
- Additional calendar systems / numeral scripts as consumers ask for them
- Wider varna script coverage once feature gating (backlog #1) lands

## Verification

| Gate | Command | Expected |
|------|---------|----------|
| Tests | `cyrius test tests/tcyr` | `32 passed, 0 failed` (1,353 assertions; `tests/foundation.tcyr` adds 22) |
| Benchmarks | `cyrius bench tests/sankhya.bcyr` | 18 benches; recorded via `scripts/bench-history.sh` |
| Smoke test | `cyrius build src/main.cyr build/sankhya` | prints `sankhya 3.1.2 (cyrius port)` |
| Lint / format | `cyrius lint <file>` / `cyrius fmt <file> --check` | 0 warnings and clean for every `src/*.cyr` (CI gates both) |
| Bundle | `cyrius distlib` | regenerates `dist/sankhya.cyr` |
