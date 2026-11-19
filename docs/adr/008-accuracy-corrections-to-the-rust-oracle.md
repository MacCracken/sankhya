# 008 — Correct the Rust oracle where it disagrees with its own sources

**Status**: Accepted (v3.1.3)
**Date**: 2026-11-18

## Context

The port's correctness bar is "matches what the Rust did" (CLAUDE.md), and `rust-old/` is
frozen as the oracle. CLAUDE.md's first principle is also that "all math must be
historically accurate", and it says to diverge from the oracle only with an ADR. Through
3.1.2 the two never conflicted, because nobody had checked the oracle against its own
sources.

The 3.1.3 review did that. For each item below the Rust code disagrees with the source its
own doc comment cites, or with a date anyone can look up. Each was confirmed
independently of the reviewer that found it, usually with a check that does not reuse the
formula being questioned.

## Decision

Correct each of the following. Every correction is pinned by a test against an external
value, and the CHANGELOG marks each one as an output change.

| Module | Rust oracle | Corrected | Evidence |
|---|---|---|---|
| hebrew | epoch offset 347996 (midnight JD 347996.5) | 347997 (347997.5) | 1 Tishrei AM 1 = Mon 7 Oct 3761 BCE Julian = noon JDN 347998 (Dershowitz & Reingold, R.D. −1373427). The Rust put Rosh Hashana 5785 on Wed 2 Oct 2024, a weekday dehiyya Lo ADU forbids; it was Thu 3 Oct. With 347997, `day mod 7` is the JDN's real weekday. |
| aztec | correlation JDN 2275856 | 2276828 | The Rust's doc names 13 Aug 1521 (Julian), which is JDN 2276828; 2275856 is 15 Dec 1518. On 2276828 the GMT Tzolkin is 1 Chicchan, the serpent sign: 1 Coatl, as Caso has it. |
| persian | last break year 2526 | 3178 | jalaali-js, the cited source, ends its breaks array at 3178 (checked against jalaali-js 1.2.6). With 2526, leap years were wrong from 2521 AP and consecutive years stopped joining up from 2624 AP. |
| islamic (Hijri) | years summed one by one from year 1 | `354(y−1) + floor((3 + 11y)/30)` | Dershowitz & Reingold's closed form equals the Rust's sum for every year ≥ 1 (checked over 1..3000). For year ≤ 0 the sum added nothing, so no pre-epoch date round-tripped; the closed form is also correct there (checked over −3000..0). |
| babylonian | reciprocal table of 29 entries | 30, adding 75 → 0;0,48 | 75 (1,15) is the only regular number ≤ 81 the table lacked, and the standard Old Babylonian table lists "1,15 48". |
| islamic (Khayyam) | `c ≈ 0 && d < 0` → CubeAndThingEqualNumber | that branch also needs b > 0 | The Rust enum's own docs define the type as x³ + bx = d. With the sign of b ignored, x³ = 15x + 4 was filed there instead of CubeEqualThingAndNumber. Only the label changes; the roots do not. |
| astro | precession coefficients | Meeus eq. 21.2 (IAU 1976, Lieske 1977), in arcseconds | The Rust's degree constants mixed up the T coefficients, the t² terms and z's t³ term (0.0000073° where 0.018203″ is 0.0000051°). That is ~0.27° of error by 3000 BCE from J2000, and more from other epochs. The new code matches an independent evaluation of Meeus 21.2 and 21.4 to 1e-9°. |
| astro | azimuth: only the cosine term divided by cos(alt)·cos(lat) | A = atan2(−cos δ sin H, sin δ cos φ − cos δ sin φ cos H) | The Rust's azimuth was exact only on the horizon, and 23° off at altitude 83°. For δ 20°, φ 40°, H 30° it gave 224.05° where an east-north-up vector check gives 240.94°. |
| astro | `heliacal_rising` accepted star − Sun in (0, 180) | (180, 360), after a day outside the window | A morning first sighting needs the star west of the Sun; the Rust's own comment says so. The Rust found Sirius's evening phase, 6 June 2000 from 30° N, near heliacal setting; it now rises in July. A start date already inside the window returned day 0, a rising that had already happened. |
| astro | solar mean anomaly 357.52811° | 357.52911° | Meeus eq. 25.3. The difference in longitude is under 3.5e-5°. |
| epoch | Long Count of `jdn as u64` (truncated) | of the civil day `floor(jdn + 0.5)` | A midnight JDN, which is what `convert()` passes for a calendar date, read as the previous day. Gregorian 21 Dec 2012 gave 12.19.19.17.19 while `convert(13.0.0.0.0)` gave 21 Dec 2012. The Hebrew and Aztec conversions already used the civil day. |

Two further changes restore parity rather than diverge from it. Rust's association order
in the obliquity polynomial is kept (`(c·t)·t`), and the double wrap `((x % m) + m) % m`
uses a new exact `f64_rem`, which is the truncating remainder Rust's `%` computes. The
floor-based `f64_fmod` used before rounded, and differed for many negative inputs.
Obliquity, GMST and LST now match the Rust formulas bit for bit at every sampled JDN.

## Consequences

- **Positive**: Hebrew, Aztec, far-future Persian and pre-epoch Hijri dates, precessed
  positions, horizontal azimuths, heliacal risings and Long Counts for calendar dates are
  now right. Each is pinned to an external value, not to another implementation of the
  same formula.
- **Negative**: Stored output changes, and every Hebrew date moves by a day. A consumer that
  saved any of these values, or compared against the Rust crate, must re-baseline. The
  CHANGELOG lists each change.
- **Neutral**: `rust-old/` stays frozen and now differs from sankhya in the places above.
  "Matches the Rust" now means "matches the Rust except as ADR-008 records". Models that
  are simplified rather than wrong are left alone: the arcus visionis heliacal model,
  which puts Sirius about three weeks before its observed date, and the two-term equation
  of centre. They are on the roadmap.

## Alternatives considered

- **Keep the Rust values and document them as known errors**: rejected for these items.
  Principle #1 is accuracy, and each Rust value contradicts the source the Rust itself
  cites. Documented errors in a calendar library still produce wrong dates.
- **Fix only the gross errors (Hebrew, Aztec)**: rejected. The smaller corrections
  (`SL_MA`, the reciprocal table) have the same justification and cost nothing extra to
  pin with tests.
