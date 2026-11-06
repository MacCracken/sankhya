# sankhya — Current State

> Refreshed every release. CLAUDE.md is preferences/process/procedures
> (durable); this file is **state** (volatile).

## Version

**3.1.0** — cyrius pin 6.7.9, the first on the 6.7 language minor, with the sibling deps
at the tags that pin it. The Rust→Cyrius port completed at 3.0.0 (2026-07-21); 3.0.1
migrated to the cyrius 6.6.x `Result` value form. The original Rust crate (v2.0.0, 9,857
lines across 17 files) is preserved at `rust-old/` as the frozen parity oracle.

## Toolchain

- **Cyrius pin**: `6.7.9` (in `cyrius.cyml [package].cyrius`)
- `VERSION` is the single source of truth; `cyrius.cyml` derives it via `${file:VERSION}`

## Source

- **Cyrius**: 10,733 lines across 37 `src/*.cyr` files.
  - Foundation — `error`, `util`, `f64_util`, `logging`
  - 15 domain modules — mayan, babylonian, egyptian, vedic, chinese, greek, roman,
    islamic, gregorian, coptic, persian, hebrew, aztec, astro, epoch
  - Serialization — `serial_util.cyr` + one `serial_<module>.cyr` per module
  - `lib.cyr` (aggregator, drives `distlib`), `main.cyr` (smoke test)
- **Consumer bundle**: `dist/sankhya.cyr`
- **Rust reference**: `rust-old/` (frozen — do not edit)

## Tests

`cyrius test tests/tcyr` → **32 suites, 0 failed** (~1,346 assertions):

| Suite group | Assertions |
|---|---|
| Per-module (15 suites) | 729 |
| Serialization round-trip (15 suites) | ~260 |
| `integration.tcyr` (all 68 Rust integration tests) | 198 |
| `adversarial.tcyr` (all 87 Rust never-panic/NaN tests) | 159 |
| `tests/foundation.tcyr` | 22 |

**Benchmarks**: 18 in `tests/sankhya.bcyr` (all 16 original criterion benches + coptic +
aztec). Run with `cyrius bench tests/sankhya.bcyr`; history in `bench-history.csv`.

**Build health**: the only compiler warnings are the itihas/avatara `xalloc` duplicate
and two undefined-function warnings in the `serial_chinese` suite (it links no gregorian
module and never reaches them). `cyrfmt --check` passes except `src/gregorian.cyr`.
Lint warnings in `src/` are line-length only (astro 29, epoch 6, mayan 1) from deeply
nested f64 expressions; CI exempts line-length. `cyrius audit` exits 1 on fmt, lint and
undocumented fns, as it did at 3.0.2 (see the roadmap); CI does not run it.

## Dependencies

Declared in `cyrius.cyml`:

- **stdlib** — string, fmt, alloc, vec, str, syscalls, io, args, assert, result, tagged,
  fnptr, hashmap, math, **ganita** (extended math), **bayan** (JSON), **sakshi** (logging),
  bench, net + http (transitive, for itihas's bundled hoosh client)
- **Sibling deps** (git-tagged, always-on — see [ADR-006](../adr/006-always-on-dependencies.md)):
  `varna` 2.5.0, `itihas` 2.6.0, `avatara` 2.16.0

## Consumers

- **hisab** — higher mathematics library (optional companion)

## Next

See [`roadmap.md`](roadmap.md). Top backlog item: re-introduce optional/feature-gated deps
once Cyrius gains a Cargo-features equivalent (currently always-on for full parity).
