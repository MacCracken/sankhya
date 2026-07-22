# Contributing to Sankhya

Thank you for your interest in contributing to Sankhya.

Sankhya is a Cyrius library (toolchain `6.4.69`), ported from the Rust v2.0.0
crate. The original Rust source is preserved read-only at `rust-old/` and serves
as the reference oracle for every ported module (see
[Cross-checking against Rust](#cross-checking-against-rust)).

## Development Workflow

1. Fork and clone the repository
2. Create a feature branch from `main` (e.g. `port/haab`, `fix/hebrew-molad`)
3. Make your changes in `src/*.cyr` and `tests/tcyr/*.tcyr`
4. Run `cyrius tests tests/tcyr` to validate (and `cyrius bench tests/sankhya.bcyr`
   if performance is touched)
5. Push the branch and open a pull request against `main`

Keep PRs focused on one module or one logical change. A PR that touches a domain
module should carry its cross-check against the `rust-old/` reference in the
description.

## Prerequisites

- Cyrius 6.4.69 (`cyriusly install 6.4.69`) — pinned in `cyrius.cyml`

## Commands

| Command | Description |
|---------|-------------|
| `cyrius build src/main.cyr build/sankhya` | Build the smoke-test binary |
| `CYRIUS_DCE=1 cyrius build src/main.cyr build/sankhya` | Build (dead-code-elimination optimized) |
| `./build/sankhya` | Run the CLI smoke test |
| `cyrius tests tests/tcyr` | Run the full suite (32 `.tcyr` suites, ~1,346 assertions) — expect `32 passed, 0 failed` |
| `cyrius bench tests/sankhya.bcyr` | Run the 18 benchmarks |
| `cyrius lint src/*.cyr` | Lint |
| `cyrius fmt src/*.cyr` | Format |
| `cyrius distlib` | Build the consumer bundle `dist/sankhya.cyr` |
| `./scripts/bench-history.sh` | Run benchmarks + record history to `bench-history.csv` |

## Layout

- `src/*.cyr` — 37 source files: 15 domain modules (`mayan`, `babylonian`,
  `egyptian`, `vedic`, `chinese`, `greek`, `roman`, `islamic`, `gregorian`,
  `coptic`, `persian`, `hebrew`, `aztec`, `astro`, `epoch`), the foundation
  (`error`, `util`, `f64_util`, `logging`), a per-module `serial_*.cyr` JSON
  layer, `lib.cyr` (the include-only aggregator), and `main.cyr` (smoke test)
- `tests/tcyr/*.tcyr` — 32 test suites (per-module, serial round-trip,
  integration, adversarial/never-panic); plus `tests/foundation.tcyr`
- `tests/sankhya.bcyr` — 18 benchmarks
- `dist/sankhya.cyr` — the single-file bundle consumers include (built by
  `cyrius distlib`)
- `rust-old/` — the frozen Rust v2.0.0 reference (do not modify)

## Adding a Module

1. Create `src/module_name.cyr`. It must be include-free — the `distlib` bundler
   strips includes and resolves stdlib/deps from the consumer's `[deps]`.
2. Create `src/serial_module_name.cyr` with hand-written
   `<type>_to_json` / `<type>_from_json` functions (Cyrius has no serde derive).
3. Add both files to the `[lib].modules` list in `cyrius.cyml`, in dependency
   order, before `src/lib.cyr` (which is always last).
4. Add `include "module_name.cyr"` (and the serial file) to `src/lib.cyr`.
5. Add a test suite `tests/tcyr/module_name.tcyr` covering the happy path,
   serial round-trips, and adversarial/never-panic inputs.
6. Cross-check the module's outputs against its `rust-old/` counterpart.
7. Update the README module table.
8. Verify: `cyrius build src/main.cyr build/sankhya && cyrius tests tests/tcyr`.

## Cross-checking against Rust

`rust-old/` holds the original Rust v2.0.0 crate (9,857 lines, 17 files) as a
frozen reference oracle. It is not built, not tested, and not maintained.

- **Do not modify `rust-old/`.** Use it only to verify Cyrius behavior.
- Every ported module must produce the same results as its Rust counterpart for
  the same inputs — calendar round-trips, numeral conversions, reciprocals,
  epoch correlations, coordinate transforms.
- A change whose output diverges from the Rust reference without an explicit,
  documented justification is a bug. Note any intentional divergence (rounding,
  ordering) in the PR description.

The Rust crate gated `varna` / `itihas` / `avatara` behind optional Cargo
features. Cyrius has no feature-flag system yet, so those siblings are
**always-on** here — full functional parity, nothing dropped. Re-introducing
true optional gating is a backlog item pending Cyrius support.

## Code Style

- No `unwrap`/`panic` equivalent — all errors via `Result` (`Ok` / `sk_fail`
  with a `{ code, message }` record)
- No serde derive — hand-written `<type>_to_json` / `<type>_from_json` (bayan)
- `rem_euclid` / `div_euclid` for calendar modulo (never bare `%` across zero)
- f64 via `f64_from` / `f64_parse`; validate NaN/Infinity on public f64 inputs
- `#must_use` on pure functions (bare `#attr` form, not Rust `#[attr]`)
- `(0 - N)` for negative year literals (no negative literals in Cyrius)
- Enum values for struct field offsets; accessor functions for struct fields
- Structured logging via `sakshi` (`sankhya_log_init`), never `println` for logs
- Comments with `#`

## Testing

- Full suite in `tests/tcyr/` (run via `cyrius tests tests/tcyr`) — per-module,
  serial round-trip, integration, and adversarial/never-panic coverage
- Foundation checks in `tests/foundation.tcyr`
- Smoke test in `src/main.cyr` (run via `./build/sankhya`)
- Benchmarks in `tests/sankhya.bcyr` (run via `cyrius bench tests/sankhya.bcyr`
  or `./scripts/bench-history.sh`)
- Target: `32 passed, 0 failed`, all modules cross-checked against `rust-old/`

## Commits

- One logical change per commit
- Descriptive messages

## License

By contributing, you agree that your contributions will be licensed under GPL-3.0.
