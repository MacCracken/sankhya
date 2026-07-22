# sankhya — Current State

> Refreshed every release. CLAUDE.md is preferences/process/procedures
> (durable); this file is **state** (volatile).

## Version

**0.1.0** — ported from Rust (2026-07-22) via `cyrius port`. 9857 lines of Rust preserved at `rust-old/` for parity reference.

## Toolchain

- **Cyrius pin**: `6.4.69` (in `cyrius.cyml [package].cyrius`)

## Source

- Rust reference: 9857 lines at `rust-old/` (frozen, do not edit).
- Cyrius port: scaffold only — `src/main.cyr` stub.

## Tests

_Replace with parity test status once tests land._

## Dependencies

Direct (declared in `cyrius.cyml`):

- stdlib — string, fmt, alloc, vec, str, syscalls, io, args, assert

## Consumers

_None yet._

## Next

See [`roadmap.md`](roadmap.md). The first milestone is typically Rust→Cyrius surface parity for the 9857-line subset.
