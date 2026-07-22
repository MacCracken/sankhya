# ADR-006: Always-on script and context dependencies (Cyrius port)

## Status

Accepted — v3.0.0 (2026-07-21). Supersedes [ADR-005](005-feature-gated-script-rendering.md).

## Context

The Rust crate gated three dependencies behind optional Cargo features, all disabled by
default:

| Dependency | Rust feature | What it powered |
|---|---|---|
| `varna` | `varna` | Unicode numeral rendering (cuneiform, hieroglyphic, Devanagari, Greek alphabetic, CJK rods) — see ADR-005 |
| `itihas` | `itihas` | `epoch` historical context — eras, civilizations, events |
| `avatara` | `avatara` | `epoch` Saptarishi archetype profiles |

The v3.0.0 port moves sankhya from Rust to Cyrius. **Cyrius has no equivalent of Cargo
features** — there is no per-consumer conditional dependency resolution, and `include` /
`[deps]` are unconditional at build time. The only conditional-compilation primitive is the
`#ifdef` preprocessor, which gates *source text*, not dependency resolution.

That leaves two options: drop the feature-gated capabilities, or compile them in always.

## Decision

**Compile all three dependencies in unconditionally.** `varna`, `itihas`, and `avatara` are
declared as ordinary (non-optional) git-tagged deps in `cyrius.cyml`, and every code path
they powered is ported and always present.

## Rationale

- **Parity first.** The port's correctness bar is "matches what the Rust did". Dropping the
  gated capabilities would have been a functional regression; making them always-on is not.
  Nothing the Rust crate could do was lost.
- The gating was a *packaging* optimisation (keep the default dependency footprint small),
  not a functional boundary. Losing the optimisation costs build size, not behaviour.
- A `#ifdef`-based emulation would gate source but still resolve and vendor every dep, so it
  would deliver the ergonomic cost of feature flags without the benefit.

## Consequences

- Every consumer of `dist/sankhya.cyr` links the varna, itihas, and avatara bundles whether
  or not they call into them. Unreached code is removed by `CYRIUS_DCE=1` builds, so the
  runtime cost is near zero; the build-time cost is real but bounded.
- `cyrius.cyml` must keep the sibling deps pinned by git tag (varna 2.1.0, itihas 2.4.0,
  avatara 2.9.0) so CI resolves without local checkouts.
- The stdlib list carries `net` + `http` transitively, because itihas's dist bundle ships a
  hoosh HTTP client we never call.
- **This is a deferral, not a permanent decision.** Re-introducing optional/feature-gated
  deps is the top item in [`../development/roadmap.md`](../development/roadmap.md), blocked
  on upstream Cyrius gaining a features equivalent. When it lands, revisit ADR-005's
  rationale — it still holds.

## Alternatives Considered

- **Drop the gated capabilities** — rejected: a functional regression, and the explicit
  instruction for the port was full parity.
- **`#ifdef` source gating** — rejected: gates source text but not dependency resolution, so
  deps are still fetched and vendored; adds build-matrix complexity for no packaging win.
- **Split into a separate `sankhya-scripts` bundle** — rejected for the same reason ADR-005
  rejected a separate crate: it fragments the API surface for what reads as one library.
