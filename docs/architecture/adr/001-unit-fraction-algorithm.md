# ADR-001: Egyptian Fraction Decomposition Algorithm

## Status

Accepted

## Context

Egyptian fractions must decompose any rational p/q into a sum of distinct unit fractions 1/n. Multiple algorithms exist: the greedy (Fibonacci-Sylvester) algorithm, continued fraction expansion, and various optimal decomposition methods.

## Decision

Use the **greedy algorithm** (Fibonacci-Sylvester) with a 100-iteration cap.

## Rationale

- The greedy algorithm is the most natural historically — it mirrors the process described in the Rhind Papyrus (always subtract the largest possible unit fraction).
- It produces correct (though not always shortest) decompositions for all valid inputs.
- The 100-iteration cap prevents infinite loops on pathological fractions where the greedy algorithm produces extremely long chains.
- Simplicity: the algorithm is easy to verify against known historical decompositions.

## Alternatives Considered

- **Continued fraction method**: Produces shorter decompositions for some inputs but has no historical basis in Egyptian mathematics.
- **Optimal (shortest) decomposition**: NP-hard in the general case; computational expense is not justified for a historical math library.

## Consequences

- Some decompositions are longer than theoretically optimal (e.g., 4/17 produces 4 terms instead of a possible 3).
- The iteration cap means extremely pathological inputs will return `Err` rather than running indefinitely.
- Overflow checking on `u64` arithmetic means very large denominators in intermediate steps will error rather than silently truncate.
