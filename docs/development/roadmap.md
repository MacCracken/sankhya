# Development Roadmap

> **Status**: v3.0.0 released (Rust→Cyrius port complete) | **Current**: 3.0.0 | **Compiler**: cyrius 6.4.69

Completed items are in [CHANGELOG.md](../../CHANGELOG.md).
`VERSION` is the single source of truth (`3.0.0`); `cyrius.cyml` derives it via `${file:VERSION}`.

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
      Run `cyrius tests tests/tcyr` → `32 passed, 0 failed`
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

Post-port work, none of it blocking. Sequenced most-consequential first; each is
its own work-loop cycle behind the cleanliness + benchmark gates.

| # | Item | Effort | Details |
|---|------|--------|---------|
| 1 | **Re-introduce optional/feature-gated deps** | Large | Re-introduce optional/feature-gated deps (`varna`/`itihas`/`avatara`) once Cyrius gains a Cargo-features equivalent — **currently always-on for full parity**. The Rust crate gated all three behind optional Cargo features (`logging`, script rendering, historical context, Saptarishi profiles); Cyrius has no feature-flag system yet, so they are compiled in unconditionally. Nothing is dropped — this is a packaging/optionality gap, not a functional one. Blocked on upstream Cyrius feature support. |
| 2 | **Line-length reflow in `astro`/`epoch`** | Low | The two densest modules (heavy `f64_*` call chains and long correlate/convert dispatch) carry a handful of over-long lines that survived the mechanical port. Reflow for readability; behavior-preserving, no API change. |
| 3 | **epoch external-context deserialization** | Medium | `epoch`'s `MultiCalendarDate` serializes its itihas eras/civilizations/events and avatara Saptarishi context on the `_to_json` path, but `_from_json` cannot fully reconstruct those sibling records until `itihas` and `avatara` ship `from_json` constructors. Round-trip is complete for the sankhya-native fields; the external-context arrays reconstruct once the upstream `from_json` entry points land. |

### Demand-gated (future)

- Display/formatting parity beyond `<type>_to_str` (the Rust `Display` impls were
  folded into the string accessors already used by the serial layer)
- Additional calendar systems / numeral scripts as consumers ask for them
- Wider varna script coverage once feature gating (backlog #1) lands

## Verification

| Gate | Command | Expected |
|------|---------|----------|
| Tests | `cyrius tests tests/tcyr` | `32 passed, 0 failed` (~1,346 assertions) |
| Benchmarks | `cyrius bench tests/sankhya.bcyr` | 18 benches; recorded via `scripts/bench-history.sh` |
| Smoke test | `cyrius build src/main.cyr build/sankhya` | prints `sankhya 3.0.0 (cyrius port)` |
| Bundle | `cyrius distlib` | regenerates `dist/sankhya.cyr` |
