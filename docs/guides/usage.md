# Usage Guide

> **Sankhya** (Sanskrit: enumeration/analysis) — ancient mathematical systems,
> historical calendars, and archaeoastronomy as Cyrius functions.

## Philosophy

Sankhya is a **pure computation library** — no I/O, every function synchronous
and deterministic. You feed it an integer, a day count, a Julian Day Number, or a
`CalendarDate`, and read back the same quantity in another representation. The
only "output" path is the serial layer, which builds a JSON string in memory and
parses it back.

Fallible functions return a `Result`: `Ok(value)` on success, or
`sk_fail(code, msg)` (an `Err` wrapping a `SkErr{code, msg}` record) on failure.
Read success with `is_ok(r)` and unwrap with `result_unwrap(r)`; read the error
code with `sk_result_code(r)` and the message with `sk_err_msg(err_code_of(r))`.

> **No `cargo run --example`.** The Rust crate shipped runnable
> `examples/*.rs`; Cyrius has no example runner, so those worked examples are
> folded into this guide below.

## Quick Start

**In-tree** (developing sankhya itself) — `lib.cyr` pulls every module together
in dependency order:

```cyrius
include "src/lib.cyr"

alloc_init();

# 3600 in base 60
var digits = to_sexagesimal(3600);   # vec: [1, 0, 0]  →  "1;0;0"
```

**As a consumer** (e.g. hisab) — include the single bundled file; stdlib and
sibling deps resolve from your own `[deps]`:

```cyrius
include "dist/sankhya.cyr"

alloc_init();
```

## Worked Examples

### Mayan Long Count

`long_count_from_julian_day` is fallible (JDN must be on/after the Mayan epoch,
JDN 584283), so it returns a `Result`. Tzolkin/Haab/Venus take a **day count from
the epoch** and are infallible.

```cyrius
# Long Count for December 21, 2012 (JDN 2456283)
var r = long_count_from_julian_day(2456283);
if (is_ok(r) == 1) {
    var lc = result_unwrap(r);
    var s  = longcount_to_str(lc);       # "13.0.0.0.0"
    # component accessors:
    var bak = lc_baktun(lc);             # 13
    var kin = lc_kin(lc);                # 0
}

# Tzolkin / Haab / Venus for a later date (JDN 2461031)
var days    = 2461031 - EPOCH_JDN;       # EPOCH_JDN = 584283
var tzolkin = tzolkin_from_days(days);   # pointer → tzolkin_to_str(tzolkin)
var haab    = haab_from_days(days);      # pointer → haab_to_str(haab)
var phase   = venus_phase(days);         # VenusPhase enum → venusphase_name(phase)

# Vigesimal (base-20)
var vig = to_vigesimal(8000);            # vec: [1, 0, 0, 0]  →  "1.0.0.0"

# Calendar Round forward search: next 4 Ahau 8 Kumku after day 1
var cr = find_calendar_round(4, DS_AHAU, 8, HM_KUMKU, 1);   # Result(Ok(day))
```

### Babylonian square root (Heron's method)

`babylonian_sqrt(n, iterations)` takes an **f64** (build it with `f64_from` or
`f64_parse`) and returns `Result` of an f64. This is the YBC 7289 tablet's
sqrt(2) algorithm.

```cyrius
var r = babylonian_sqrt(f64_from(2), 10);
if (is_ok(r) == 1) {
    var sqrt2 = result_unwrap(r);        # f64 ≈ 1.414213562373095
}

# Sexagesimal (base-60) representation
var sx = to_sexagesimal(3600);           # vec: [1, 0, 0]  →  "1;0;0"

# Plimpton 322 Pythagorean triples
var triples = generate_plimpton_triples();   # vec of (a, b, c) rows

# Saros eclipse cycle: next eclipse after J2000.0 (JDN 2451545.0)
var next = saros_cycle(f64_from(2451545));   # f64 JDN of the next eclipse
```

### Greek: Sieve of Eratosthenes

`sieve(limit)` returns a vec of primes below `limit`. GCD/LCM are plain integer
functions; `archimedes_pi` returns a two-value tuple of bounds.

```cyrius
var primes = sieve(100);                 # vec: [2, 3, 5, 7, 11, ... , 97]
var n      = vec_len(primes);            # 25

var g = gcd(48, 18);                     # 6
var l = lcm(12, 18);                     # 36

# Archimedes' bounds on pi after 5 iterations (192-gon)
var (lo, hi) = archimedes_pi(5);         # f64 lower/upper bounds

# Golden ratio + Fibonacci-ratio convergence
var phi   = greek_phi();                 # f64 ≈ 1.618033988749895
var ratio = fibonacci_ratio(40);         # f64 → approaches phi

# Antikythera gear ratios
var gears = antikythera_gear_ratios();   # vec of named cycles
```

### Roman numerals

`to_roman_str` / `from_roman` are `Result`-returning (range/format validation).
Arithmetic comes in two forms: `roman_*` on integer values, `romannum_*` on
`RomanNumeral` records built with `romannum_from_value`.

```cyrius
var r = to_roman_str(1776);
var s = result_unwrap(r);                # "MDCCLXXVI"

var v = from_roman("MCMXCIX");
var n = result_unwrap(v);                # 1999

# Arithmetic on values (each returns Result)
var sum  = roman_add(1776, 248);         # Ok(2024)
var diff = roman_subtract(1776, 248);    # Ok(1528)
var prod = roman_multiply(12, 12);       # Ok(144)
var quot = roman_divide(17, 5);          # Ok(3) (+ remainder path)

# Arithmetic on RomanNumeral records
var a = result_unwrap(romannum_from_value(1776));
var b = result_unwrap(romannum_from_value(248));
var c = romannum_add(a, b);              # Ok(2024)

# Validation (0/1, never panics)
var ok  = is_valid_roman("XIV");         # 1
var bad = is_valid_roman("IIII");        # 0
```

### Cross-calendar `convert()`

The `epoch` capstone bridges every system. Wrap a source date in a
`CalendarDate` variant (`caldate_gregorian`, etc.), then `convert()` routes it
through a Julian Day Number and correlates it across all calendars into a
`MultiCalendarDate`. Read fields with the `mcd_*` accessors.

```cyrius
# Gregorian 2012-12-21 → all systems
var greg = gregorian_date_new(2012, 12, 21);
var date = caldate_gregorian(greg);
var r    = convert(date);

if (is_ok(r) == 1) {
    var mcd = result_unwrap(r);
    var jdn = mcd_jdn(mcd);              # Julian Day Number
    var lc  = mcd_mayan_lc(mcd);         # Mayan Long Count pointer (or 0)
    var tz  = mcd_tzolkin(mcd);          # Tzolkin pointer (or 0)
    var age = mcd_age(mcd);              # precessional AgePosition
    var jy  = mcd_julian_year(mcd);      # f64 Julian year
}

# Direct JDN correlation (skip the CalendarDate wrapper)
var c = correlate(J2000_JDN);            # Result(Ok(MultiCalendarDate))

# Epoch helpers
var jdn  = julian_year_to_jdn(1);        # ~0 CE
var year = jdn_to_julian_year(jdn);
var bp   = bp_to_jdn(f64_from(12800));   # 12,800 years Before Present → JDN
var pos  = precessional_age(J2000_JDN);  # AgePosition record
var days = cycle_period(CN_PRECESSION);   # f64 days in one precession cycle
var sages = all_sages_traditions();      # vec of Seven Sages traditions
```

### JSON round-trip (serial layer)

Every domain type has `<type>_to_json(p)` → `Str`, `<type>_from_json(v)` →
pointer, and `<type>_eq(a, b)` for verification. Parse a JSON string into a
`bayan` value with `sk_json_parse` before calling `_from_json`.

```cyrius
# Long Count → JSON → Long Count
var lc   = result_unwrap(long_count_from_julian_day(2456283));
var js   = longcount_to_json(lc);        # {"baktun":13,"katun":0,"tun":0,"uinal":0,"kin":0}

var node = sk_json_parse(js);            # bayan value
var back = longcount_from_json(node);    # reconstructed Long Count
var same = longcount_eq(lc, back);       # 1 — round-trip is lossless

# The whole cross-calendar record round-trips the same way
var mcd  = result_unwrap(convert(caldate_gregorian(gregorian_date_new(2012, 12, 21))));
var mjs  = multi_calendar_date_to_json(mcd);
var mcd2 = multi_calendar_date_from_json(sk_json_parse(mjs));
var eq   = multi_calendar_date_eq(mcd, mcd2);   # 1 for sankhya-native fields
```

> The `MultiCalendarDate` serializes its itihas eras/civilizations/events and
> avatara Saptarishi context on the `_to_json` path; those external-context
> arrays fully reconstruct on `_from_json` once `itihas`/`avatara` ship
> `from_json` constructors (see [roadmap](../development/roadmap.md), backlog #3).

## Conventions

- **Years** use astronomical numbering — negative = BCE (`(0 - 2499)` for 2500 BCE),
  positive = CE. Epoch constants: `EPOCH_JDN` (Mayan, 584283), `J2000_JDN`,
  `YOUNGER_DRYAS_JDN`.
- **Floats** are opaque f64 handles — build with `f64_from(int)` / `f64_parse("…")`,
  operate with `f64_add/sub/mul/div/sqrt`, never native float literals.
- **Errors** never panic. Fallible calls return `Result`; check `is_ok(r)` before
  `result_unwrap(r)`. The adversarial suite (159 assertions) enforces this.
- **Logging** is off unless initialized — call `sankhya_log_init()` (sakshi) to see
  the debug/info traces on `convert()` / `calendar_to_jdn`.

## Testing & Benchmarks

```sh
cyrius tests tests/tcyr        # 32 suites, ~1,346 assertions → "32 passed, 0 failed"
cyrius bench tests/sankhya.bcyr # 18 benchmarks
```
