# Architecture Decision Records

Decisions about sankhya — what we chose, the context, and the consequences we accept. Use these when a future reader would reasonably ask *"why did we do it this way?"*

## Conventions

- **Filename**: `NNN-kebab-case-title.md`, zero-padded to three digits (the established
  scheme here — `001`…`006`). Never renumber.
- **One decision per ADR.** If a decision supersedes a prior one, add a new ADR and set the old one's status to `Superseded by NNN`.
- **Status lifecycle**: `Proposed` → `Accepted` → (optionally) `Superseded` or `Deprecated`.
- Use [`template.md`](template.md) as the starting point.

## ADR vs. architecture note vs. guide

| Kind | Lives in | Answers |
|---|---|---|
| ADR | `docs/adr/` | *Why did we choose X over Y?* |
| Architecture note | `docs/architecture/` | *What non-obvious constraint is true about the code?* |
| Guide | `docs/guides/` | *How do I do X?* |

## Index

| ADR | Decision | Status |
|---|---|---|
| [001](001-unit-fraction-algorithm.md) | Greedy (Fibonacci–Sylvester) unit-fraction decomposition for Egyptian fractions | Accepted |
| [002](002-gmt-correlation-constant.md) | GMT-584283 as the Maya Long Count ↔ Julian Day correlation | Accepted |
| [003](003-canonical-precession-period.md) | Canonical 25,920-year Great Year (not the modern ~25,772) | Accepted |
| [004](004-seven-sages-data-model.md) | Static function + struct for the Seven Sages traditions | Accepted |
| [005](005-feature-gated-script-rendering.md) | Feature-gate script rendering behind the `varna` Cargo feature | **Superseded by 006** |
| [006](006-always-on-dependencies.md) | Compile `varna` / `itihas` / `avatara` in unconditionally (Cyrius port) | Accepted (v3.0.0) |

ADRs 001–005 were written for the Rust crate (v1.x–2.0.0). They remain accurate about the
*domain* decisions (algorithms, correlation constants, data models); where the Rust
*implementation* detail no longer holds after the v3.0.0 Cyrius port, the ADR carries an
inline **Cyrius port note**.
