# ADR-003: Canonical Precession Period (25,920 Years)

## Status

Accepted

## Context

The precession of the equinoxes — the slow wobble of Earth's rotational axis — has both a modern astronomical value (~25,772 years) and a canonical ancient value (25,920 years). The epoch module must choose which to use.

## Decision

Use the **canonical ancient value of 25,920 years** for the Great Year.

## Rationale

- Sankhya models what the ancients encoded, not modern observational astronomy. The purpose is to faithfully implement ancient computational systems.
- 25,920 has clean factorization that appears across civilizations:
  - Babylonian: 72 years per degree (72 × 360 = 25,920)
  - Egyptian: 36 decans × 720 years = 25,920
  - Hindu: 12 precessional ages of 2,160 years
- This value produces the "1 degree per 72 years" rate used in ancient astronomical texts.
- Using the modern value would obscure the mathematical relationships that the ancients encoded.

## Alternatives Considered

- **25,772 years (modern astronomical)**: More accurate for present-day astronomy, but misrepresents what ancient civilizations computed and encoded.
- **Configurable**: Allow the user to choose. Rejected as over-engineering for the initial release — a future version could add a `modern_precession()` variant.

## Consequences

- Precessional age boundaries differ slightly from modern astronomical calculations.
- The Age of Aquarius "starts" at a slightly different date than modern astronomical precession would indicate.
- The Younger Dryas anchor at the start of Leo (~10,800 BCE) aligns with the 25,920-year cycle by design.
- All cross-civilizational correlations use a consistent ancient framework.
