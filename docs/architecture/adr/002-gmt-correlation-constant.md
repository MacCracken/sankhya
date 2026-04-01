# ADR-002: GMT Correlation Constant for the Mayan Calendar

## Status

Accepted

## Context

The Mayan Long Count calendar must be correlated with the Julian/Gregorian calendar via a "correlation constant" — the Julian Day Number of the Mayan creation date (0.0.0.0.0). Multiple constants have been proposed, with scholarly consensus shifting over the decades.

## Decision

Use the **Goodman-Martinez-Thompson (GMT) correlation constant: JDN 584,283**, placing the Mayan creation date at August 11, 3114 BCE (Julian).

## Rationale

- The GMT constant is the most widely accepted in modern Mayanist scholarship (Thompson 1950, revised by Lounsbury 1983).
- It is consistent with astronomical events recorded in the Dresden Codex (e.g., Venus table observations).
- It correctly places the December 21, 2012 "end date" at Long Count 13.0.0.0.0.
- Cross-validation with archaeological radiocarbon dates supports this correlation.

## Alternatives Considered

- **584,285 (Spinden)**: 2 days later. Once popular but now largely rejected. Inconsistent with astronomical records in the Dresden Codex.
- **584,280 (Wells)**: 3 days earlier. A minority position with limited scholarly support.
- **Astronomical correlation**: Some researchers propose values based on specific eclipse records, but these vary by ±1-2 days and lack consensus.

## Consequences

- All Mayan calendar computations in sankhya (Long Count, Tzolkin, Haab, Calendar Round) are anchored to this constant.
- If future scholarship revises the constant, changing the single `EPOCH_JDN` value will propagate correctly through all derived computations.
- The epoch module's `correlate()` function uses this constant for cross-civilizational date mapping.
