# Benchmarks: Rust v2.0.0 vs Cyrius v3.0.0

Rust source: **v2.0.0** (preserved at `rust-old/`, criterion 0.5, rustc 1.89)
Cyrius port: **2026-07-21** (cyrius 6.4.69, `cyrius.cyml` derives VERSION `3.0.0` via `${file:VERSION}`)

## Port Summary

| Metric | Rust v2.0.0 | Cyrius v3.0.0 |
|--------|-------------|---------------|
| Source lines | 9,857 | 10,733 |
| Source files | 17 (`src/*.rs`) | 37 (`src/*.cyr`) |
| Domain modules | 15 | 15 |
| Compiler / toolchain | rustc 1.89 | cyrius 6.4.69 |
| Distribution | library crate | `dist/sankhya.cyr` (368 KB, via `cyrius distlib`) |
| Serialization | serde derive | bayan (hand-written `*_to_json` / `*_from_json`, full round-trip) |
| Error type | thiserror enum | `code`+`message` record, `Ok` / `sk_fail` |
| Logging | tracing | sakshi (`sankhya_log_init`) |
| Tests | 418 `#[test]` | 32 `.tcyr` suites / ~1,346 assertions |
| Optional deps (varna/itihas/avatara) | Cargo `[features]` gated | always-on git deps (feature gating deferred) |
| Parity | baseline | full parity minus optional-feature gating |

The Cyrius port reaches **full functional parity** with the Rust v2.0.0 crate.
The one deliberate difference is that the three sibling deps the Rust crate hid
behind optional Cargo features are **always compiled in** here — nothing is
dropped, and re-introducing true optional gating is a backlog item pending
Cyrius feature-flag support (see [Parity notes](#parity-notes)).

## Source Lines

| | Rust v2.0.0 | Cyrius v3.0.0 |
|---|-------------|---------------|
| `src/*` lines | 9,857 | 10,733 |
| Files | 17 | 37 |
| Layout | 15 domain modules + `error` + `lib.rs` | 15 domain modules + foundation (`error`, `util`, `f64_util`, `logging`) + per-module `serial_*.cyr` JSON layer + `lib.cyr` aggregator + `main.cyr` smoke test |

Domain modules (identical set, both ports): `mayan`, `babylonian`, `egyptian`,
`vedic`, `chinese`, `greek`, `roman`, `islamic`, `gregorian`, `coptic`,
`persian`, `hebrew`, `aztec`, `astro`, `epoch`.

The Cyrius tree carries more files and slightly more lines because serde's
derive macro is replaced by an explicit, hand-written JSON layer — one
`serial_<module>.cyr` per module — rather than a compiler-generated impl.

## Tests

| | Rust v2.0.0 | Cyrius v3.0.0 |
|---|-------------|---------------|
| Unit of measure | `#[test]` functions | `.tcyr` suites / assertions |
| Count | 418 `#[test]` | 32 suites, ~1,346 assertions |
| Runner | `cargo test` | `cyrius tests tests/tcyr` → `32 passed, 0 failed` |

Cyrius assertion breakdown (`tests/tcyr/` plus `tests/foundation.tcyr`):

| Group | Assertions |
|-------|-----------|
| Per-module correctness | 729 |
| Serial (JSON) round-trip | 260 |
| Integration (cross-module) | 198 |
| Adversarial / never-panic | 159 |
| Foundation (`tests/foundation.tcyr`) | 22 |
| **Total** | **~1,346** |

Rust counts one `#[test]` per case; Cyrius counts individual assertions inside
suites, so the two columns measure different granularities and are not directly
proportional. Coverage is equivalent: every Rust behavior has a Cyrius suite,
plus the serial round-trip and adversarial never-panic groups the JSON layer
adds.

## Dependency Mapping

| Rust dependency | Cyrius replacement | Notes |
|-----------------|--------------------|-------|
| `serde` (derive) | **bayan** | No derive macros — hand-written `*_to_json` / `*_from_json` per type |
| `serde_json` | **bayan** | Parse + serialize; full round-trip verified (260 assertions) |
| `tracing` | **sakshi** | Structured logging; `sankhya_log_init`, `sakshi_set_level(SK_ERROR)` |
| `tracing-subscriber` (`logging` feature) | **sakshi** | Folded into the sakshi stdlib module |
| `thiserror` | `code`+`message` record | `Result` via `Ok` / `sk_fail`; propagation with `?` on tagged results |
| `criterion` (dev) | **lib/bench** | `.bcyr` harness — `bench_run_batch` / `bench_report_all` (see `tests/sankhya.bcyr`) |
| `varna` (`varna` feature, optional) | **varna 2.1.0** (git, tag-pinned) | Numeral modules; **always-on** |
| `itihas` (`itihas` feature, optional) | **itihas 2.4.0** (git, tag-pinned) | epoch historical context (eras/civilizations/events); **always-on** |
| `avatara` (`avatara` feature, optional) | **avatara 2.9.0** (git, tag-pinned) | epoch Saptarishi archetype profiles; **always-on** |
| `hisab` (dep) | consumer | hisab is a downstream consumer, not a build dep of the port |
| (stdlib f64/int math) | **math** + **ganita** | `math`: f64 constants + `f64_parse`/`min`/`max`/`clamp`, `gcd`/`lcm`; `ganita`: `f64_pow`/`hypot`/`asin`/`acos`/`atan2`/`sinh`/`cosh` (astro) |
| — | **net** / **http** (transitive) | Satisfy the hoosh HTTP client bundled in itihas's dist; not called by sankhya |

Folded stdlib modules resolve at compile time from `cyrius.cyml`'s `[deps]`
(`string`, `fmt`, `alloc`, `vec`, `str`, `syscalls`, `io`, `args`, `assert`,
`result`, `tagged`, `fnptr`, `hashmap`, `math`, `ganita`, `bayan`, `sakshi`,
`bench`, `net`, `http`). The three sibling git deps (`varna`, `itihas`,
`avatara`) are tag-pinned and locked in `cyrius.lock`.

## Binary / Distribution Size

| | Rust v2.0.0 | Cyrius v3.0.0 |
|---|-------------|---------------|
| Consumer artifact | library crate (`.rlib`, no standalone binary) | `dist/sankhya.cyr` — **374,390 bytes (368 KB)** |
| Build tool | `cargo build` | `cyrius distlib` |
| Smoke-test binary | — | `build/sankhya` (from `src/main.cyr`; CLI-only, excludes the library) |

`cyrius distlib` bundles the foundation, 15 domain modules, the per-module
`serial_*` JSON layer, and `lib.cyr` into the single self-contained
`dist/sankhya.cyr` that consumers (hisab, …) `include`. The Rust crate ships no
standalone binary, so there is no like-for-like binary comparison — the dist
bundle is the closest analog.

## Benchmarks

18 Cyrius benchmarks in `tests/sankhya.bcyr` — all 16 Rust criterion
`bench_function` groups plus two new closes-the-gap benches (`coptic`, `aztec`,
marked †). Run:

```sh
cyrius bench tests/sankhya.bcyr
# or capture current numbers directly:
cyrius build tests/sankhya.bcyr build/_cmp && ./build/_cmp && rm build/_cmp
```

Light ops run 1,000,000 iterations (`BATCH 10000 × ROUNDS 100`); heavy
allocators (`greek/sieve`, `epoch/correlate`, `epoch/convert_all`) run 5,000
(`HBATCH 1000 × HROUNDS 5`) so cumulative bump-heap use stays bounded. Times
below are from the current toolchain (cyrius 6.4.69); the Rust column is a
placeholder — re-run `cargo bench` in `rust-old/` (criterion 0.5, HTML reports)
to populate it.

| Module | Benchmark | Cyrius avg | (min / max) | Iters | Rust (criterion) |
|--------|-----------|-----------:|-------------|------:|-----------------:|
| mayan | `long_count_conversion_1000` | 74 ns | 69 / 94 ns | 1,000,000 | _TBD_ |
| mayan | `vigesimal_roundtrip_1000` | 193 ns | 185 / 232 ns | 1,000,000 | _TBD_ |
| greek | `sieve_10000` | 86.416 µs | 85.439 / 87.680 µs | 5,000 | _TBD_ |
| greek | `archimedes_pi_20` | 343 ns | 338 / 360 ns | 1,000,000 | _TBD_ |
| chinese | `chinese_remainder_100` | 374 ns | 364 / 412 ns | 1,000,000 | _TBD_ |
| egyptian | `decompose_1000` | 195 ns | 190 / 210 ns | 1,000,000 | _TBD_ |
| babylonian | `sqrt_convergence` | 295 ns | 288 / 309 ns | 1,000,000 | _TBD_ |
| vedic | `nikhilam_multiply_100` | 59 ns | 56 / 68 ns | 1,000,000 | _TBD_ |
| islamic | `cubic_newton` | 2.318 µs | 2.277 / 2.510 µs | 1,000,000 | _TBD_ |
| epoch | `correlate` | 4.551 µs | 4.529 / 4.583 µs | 5,000 | _TBD_ |
| roman | `roundtrip_3999` | 813 ns | 804 / 840 ns | 1,000,000 | _TBD_ |
| gregorian | `jdn_roundtrip_1000` | 116 ns | 112 / 156 ns | 1,000,000 | _TBD_ |
| hebrew | `jdn_roundtrip_100` | 2.498 µs | 2.462 / 2.581 µs | 1,000,000 | _TBD_ |
| persian | `jdn_roundtrip_1000` | 613 ns | 601 / 645 ns | 1,000,000 | _TBD_ |
| astro | `precession_1000` | 597 ns | 592 / 614 ns | 1,000,000 | _TBD_ |
| epoch | `convert_all_calendars` | 4.779 µs | 4.763 / 4.799 µs | 5,000 | _TBD_ |
| coptic † | `jdn_roundtrip` | 118 ns | 115 / 129 ns | 1,000,000 | — (new) |
| aztec † | `calendar_round` | 60 ns | 58 / 70 ns | 1,000,000 | — (new) |

† Not present in the Rust criterion suite — added in the Cyrius port to close
the per-module benchmark-coverage gap.

Notes on the harness:

- Each Rust criterion bench wraps an internal `0..N` loop; here each `_b_<name>`
  performs one representative operation and the `BATCH × ROUNDS` harness supplies
  the repetition, so the per-op numbers are comparable in intent.
- `sakshi_set_level(SK_ERROR)` silences the `INFO`/`DEBUG` emission from
  `convert()` / `correlate()` so logging I/O does not skew timings.
- Module globals are warmed (`gregorian_init`, `epoch_init`, `astro_init`, …)
  and the itihas/avatara lazy caches touched by `correlate`/`convert` are
  pre-built before the first timed call.
- Calendar modulo uses `rem_euclid` / `div_euclid`; f64 values come from
  `f64_from` / `f64_parse`; pure functions carry `#must_use` (bare `#attr`, not
  Rust's `#[attr]`).

## Parity notes

The port is **complete** — every Rust v2.0.0 module, type, function, and
behavior has a Cyrius equivalent, and the JSON layer, integration tests, and
adversarial never-panic suites go beyond the original coverage.

| Rust feature | Cyrius status | Plan |
|--------------|---------------|------|
| `varna` / `itihas` / `avatara` optional Cargo features | Always-on (full parity, nothing dropped) | **Backlog**: re-introduce true optional/feature gating once Cyrius grows a feature-flag system |

Because Cyrius has no feature-flag mechanism yet, the three sibling deps that
the Rust crate gated behind `[features]` compile in unconditionally. This is a
strict superset of the Rust default build (which shipped none of them) and
matches the Rust `--all-features` build functionally. No capability is lost; the
only deferred work is restoring the ability to *exclude* them at build time.

---

Generated during the Cyrius port, 2026-07-21 (cyrius 6.4.69).
Cyrius benchmark numbers captured from `tests/sankhya.bcyr` on the current
toolchain. Rust source and criterion harness preserved at `rust-old/`.
