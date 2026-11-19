# sankhya — Current State

> Refreshed every release. CLAUDE.md is preferences/process/procedures
> (durable); this file is **state** (volatile).

## Version

**3.2.0** — cyrius pin 6.7.18, with the sibling deps at their latest tags (varna 2.6.0).
The Rust→Cyrius port completed at 3.0.0 (2026-07-21); 3.0.1 migrated to the cyrius 6.6.x
`Result` value form, 3.1.0 moved to the 6.7 language minor, 3.1.2 cleared the line-length
and fmt backlog, 3.1.3 was a P(-1) sweep: every public fn documented, input-domain
guards (ADR-007) and accuracy corrections to the Rust oracle (ADR-008), and 3.2.0 made
every `<type>_from_json` return a Result with serde's checks and messages (ADR-009). The original Rust
crate (v2.0.0, 9,857 lines across 17 files) is preserved at `rust-old/` as the frozen
parity oracle; ADR-007 and ADR-008 list where sankhya now differs from it.

## Toolchain

- **Cyrius pin**: `6.7.18` (in `cyrius.cyml [package].cyrius`)
- `VERSION` is the single source of truth; `cyrius.cyml` derives it via `${file:VERSION}`

## Source

- **Cyrius**: 11,917 lines across 37 `src/*.cyr` files; all 878 public fns in `src/` and
  `tests/` carry a doc comment (`cyrdoc --check`).
  - Foundation — `error`, `util`, `f64_util`, `logging`
  - 15 domain modules — mayan, babylonian, egyptian, vedic, chinese, greek, roman,
    islamic, gregorian, coptic, persian, hebrew, aztec, astro, epoch
  - Serialization — `serial_util.cyr` + one `serial_<module>.cyr` per module
  - `lib.cyr` (aggregator, drives `distlib`), `main.cyr` (smoke test)
- **Consumer bundle**: `dist/sankhya.cyr`
- **Rust reference**: `rust-old/` (frozen — do not edit)

## Tests

`cyrius test tests/tcyr` → **35 suites, 0 failed** (2,133 assertions, plus 22 in
`tests/foundation.tcyr`). Besides the per-module, serial round-trip, integration and
adversarial suites ported from Rust, three were added at 3.1.3: `serial_malformed`
(every decoder on hostile JSON), `lazy_init` (no init calls, logging off) and
`usage_examples` (every sample in `docs/guides/usage.md`, compiled). From 3.2.0 every
`serial_<module>` suite also checks the decoders' serde-parity errors.

**Fuzz**: `tests/sankhya.fcyr` runs 20,000 xorshift-driven rounds (malformed JSON, raw f64
JDNs with round-trip checks, Roman and Katapayadi strings) and exits 1 on a violation; CI
runs it from 3.1.3.

**Benchmarks**: 24 in `tests/sankhya.bcyr` (the 16 original criterion benches, coptic,
aztec, and from 3.1.3 the serial round trip, the two searches and three varna
renderers). Run with `cyrius bench tests/sankhya.bcyr`; history in `bench-history.csv`.

**Build health**: the only compiler warnings are the itihas/avatara `xalloc` duplicate
and two undefined-function warnings in the `serial_chinese` suite (it links no gregorian
module and never reaches them). Every `src/*.cyr` passes `cyrius fmt --check` and lints
with 0 warnings; CI gates both, line length included. `cyrius audit` exits 0 from 3.1.3
(fmt, lint and docs all pass). Its docs count had been inflated by a cyrius bug: it walks
`tests/tcyr/` three times (see the roadmap).

## Dependencies

Declared in `cyrius.cyml`:

- **stdlib** — string, fmt, alloc, vec, str, syscalls, io, args, assert, result, tagged,
  fnptr, hashmap, math, **ganita** (extended math), **bayan** (JSON), **sakshi** (logging),
  bench, net + http (transitive, for itihas's bundled hoosh client)
- **Sibling deps** (git-tagged, always-on — see [ADR-006](../adr/006-always-on-dependencies.md)):
  `varna` 2.6.0, `itihas` 2.6.2, `avatara` 2.16.1

## Consumers

- **hisab** — higher mathematics library (optional companion)

## Next

See [`roadmap.md`](roadmap.md). Top backlog item: re-introduce optional/feature-gated deps.
Cyrius has `optional` deps and `[features]` (since 6.3.1), but a feature only decides
whether a dep is resolved, not which code compiles, so the deps stay always-on for now.
