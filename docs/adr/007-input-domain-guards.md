# 007 — Reject inputs the Rust types ruled out

**Status**: Accepted (v3.1.3)
**Date**: 2026-11-18

## Context

The Rust crate's types bounded its inputs before any code ran. A day was a `u8`, a day
count a `u64`, Roman arithmetic took `u32`, a month was an enum with a fixed set of
variants, and a JSON field of the wrong type was a serde error. Cyrius has one integer
type, `i64`, and no enums with a checked range, so the port receives values the Rust
never could: negative counts, month index 12 or -1, `i64::MIN`, and NaN or ±Inf where an
f64 is expected.

The P(-1) sweep for 3.1.3 found what those values did. Some crashed: `vec_get` aborted on
a month of -1, `i64::MIN / -1` trapped with SIGFPE in four functions, and `sieve` wrote
through an unchecked `alloc`. Some hung: `correlate` and the Coptic and Hebrew year
searches never returned for |JDN| beyond about 1e14. Some returned a wrong `Ok`:
`roman_add(-1, 5)` gave IV, Gregorian day -5 gave a JDN, Hijri month 12 read as Muharram,
and the Khwarizmi solvers returned NaN roots. Each of these breaks CLAUDE.md's "zero panics"
rule or the adversarial suite's "Err or finite, never NaN/Inf" rule.

## Decision

Where an input is outside the domain the Rust type allowed, the Cyrius function treats it
as an error instead of computing with it. Every input the Rust could receive behaves
exactly as before. In detail:

- **A function that returns a `Result` returns an `Err`**, using the code the Rust used for
  the nearest in-domain failure. Negative counts and operands, a month outside its
  calendar, a day below 1, and Roman operands outside `0..u32::MAX` are `InvalidDate`,
  `InvalidBase` or `InvalidFraction`. Non-finite coefficients and JDNs, and |JDN| > 1e14 in
  `correlate`, are `Computation` or `InvalidDate`. A null or unknown `CalendarDate` is
  `InvalidDate`.
- **A function with no error path stays total.** `greek_gcd` and `greek_lcm` take
  magnitudes. `fibonacci_ratio` clamps n to 1474, where every smaller n is bit-identical
  and the ratio has long since stopped moving. `solstice_sunrise_azimuth` clamps its
  cosine to the horizon beyond the polar circles, and `monument_alignment` skips those
  latitudes explicitly, so its output is unchanged. `jdn_to_coptic` and `jdn_to_hebrew`
  clamp the day count, at ±2^52 and ±1e14, so every input terminates. A decoder seeing a
  missing or mistyped enum field yields the code -1, and the name lookups render -1 as
  `"?"`, so a later `_to_json` cannot abort.
- **The value Rust's wrapping arithmetic defined is kept where it is the point.**
  `egyptian_multiply` loops on `!= 0`, so a negative operand, which is a u64 ≥ 2^63 to the
  Rust, multiplies to the same bits.

## Consequences

- **Positive**: The library cannot be crashed, hung or tricked into a garbage `Ok` by an
  out-of-domain value. The fuzz harness and `serial_malformed.tcyr` check this for random
  input.
- **Negative**: A few calls that used to return `Ok` now return `Err`, for example a
  negative Roman operand or a Gregorian day of 0 or below. No Rust caller could make those
  calls, but a Cyrius caller that relied on the garbage result will see the change.
- **Neutral**: The JSON decoders still do not return an `Err` the way serde does; they
  return a record holding -1 codes, which the domain functions then reject. A
  `Result`-returning decoder API would change every `_from_json` signature, so it is
  roadmap work, not a patch-release change.

## Alternatives considered

- **Emulate the Rust types exactly** (reinterpret a negative i64 as a large u64 throughout):
  rejected except for `egyptian_multiply`, where it is a single loop condition. Elsewhere
  the u64 reading of a negative number is a value no caller means, and the arithmetic to
  carry it (unsigned division, comparison) would add a lot of code to produce garbage the
  Rust could not produce either.
- **Leave it to callers**: rejected. The zero-panic rule is the library's to keep, and the
  bad values often come from the library's own JSON decoders.
