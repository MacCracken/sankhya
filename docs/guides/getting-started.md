# Getting started with sankhya

## Build and test

```sh
cyrius deps                                    # resolve sibling deps (varna/itihas/avatara)
cyrius lib sync                                # vendor the folded stdlib into lib/
cyrius build src/main.cyr build/sankhya        # compile the smoke test
./build/sankhya                                # -> sankhya 3.0.0 (cyrius port)
cyrius tests tests/tcyr                        # run all 32 suites -> "32 passed, 0 failed"
cyrius bench tests/sankhya.bcyr                # run the 18 benchmarks
cyrius distlib                                 # rebuild dist/sankhya.cyr for consumers
```

## Layout

- `src/*.cyr` — the library. Foundation (`error`, `util`, `f64_util`, `logging`), 15 domain
  modules (mayan … epoch), and a `serial_<module>.cyr` JSON layer per module.
- `src/lib.cyr` — the aggregator. The **only** src file carrying `include` lines; its order
  is the dependency order and drives `cyrius distlib`.
- `src/main.cyr` — minimal smoke test only. It does *not* include the library.
- `tests/tcyr/*.tcyr` — test suites (one per module, plus `integration` and `adversarial`).
- `tests/sankhya.bcyr` — benchmarks.
- `dist/sankhya.cyr` — the bundle consumers include.
- `rust-old/` — the original Rust source, preserved as the parity oracle. **Do not modify.**

## Using sankhya as a consumer

```sh
# in your cyrius.cyml
# [deps.sankhya]
# git = "https://github.com/MacCracken/sankhya.git"
# tag = "3.0.0"
# modules = ["dist/sankhya.cyr"]
```

Then call the library directly — see [`usage.md`](usage.md) for worked examples.

## Adding a feature

1. Add or edit the relevant `src/<module>.cyr`. Keep it **self-contained** — no `include`
   lines (the distlib bundler strips them); stdlib and deps resolve from `[deps]`.
2. Register it in `src/lib.cyr` (dependency order) and in `cyrius.cyml` `[lib].modules`.
3. Cross-check parity against `rust-old/` — that is the correctness bar.
4. Add assertions to `tests/tcyr/<module>.tcyr` (and `serial_<module>.tcyr` if the type is
   serializable), then run `cyrius tests tests/tcyr`.
5. Add a benchmark to `tests/sankhya.bcyr` for any new hot path and run
   `./scripts/bench-history.sh`.
6. Add a CHANGELOG entry. **Do not bump `VERSION`** unless explicitly asked.

## Gotchas worth knowing early

- Everything is `i64` — no `u8`/`u32`/`u64`. `f64` values are bit patterns; use
  `f64_from(int)` / `f64_parse("x.y")`, never a float literal.
- Calendar arithmetic on pre-epoch dates **must** use `rem_euclid` / `div_euclid` from
  `util.cyr` — bare `%` is truncated and silently corrupts months/days.
- `f64_to` does **not** saturate the way Rust's `as` cast does — bounds-check indices
  derived from floats.
- No negative literals: write `(0 - 3114)`.
- `alloc_init()` must be the first line of any `main()`.

See [`../adr/template.md`](../adr/template.md) when a non-trivial design choice deserves an
ADR, and [`../adr/`](../adr/) for the existing decision records.
