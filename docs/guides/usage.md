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
Since cyrius 6.6 a `Result` is a register **pair**, so bind both halves:
`var r_t, r_v = f(x);`. Test with `is_ok(r_t)`, unwrap with `result_unwrap(r_t, r_v)`,
read the code with `sk_result_code(r_t, r_v)` and the message with `sk_err_msg(r_v)`.

> Every sample below is compiled and checked by `tests/tcyr/usage_examples.tcyr`;
> edit the two together.

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
var r_t, r_v = long_count_from_julian_day(2456283);
if (is_ok(r_t) == 1) {
    var lc = result_unwrap(r_t, r_v);
    var s = longcount_to_str(lc);        # "13.0.0.0.0"
    var bak = lc_baktun(lc);             # 13
    var kin = lc_kin(lc);                # 0
}

# Tzolkin / Haab / Venus for a later date (JDN 2461031)
var days = 2461031 - EPOCH_JDN;          # EPOCH_JDN = 584283
var tzolkin = tzolkin_from_days(days);   # tzolkin_to_str(tzolkin)
var haab = haab_from_days(days);         # haab_to_str(haab)
var phase = venus_phase(days);           # VenusPhase code -> venusphase_name(phase)

# Vigesimal (base-20)
var vig = to_vigesimal(8000);            # vec [1, 0, 0, 0]

# Calendar Round forward search: next 4 Ahau 8 Kumku after day 1
var cr_t, cr_v = find_calendar_round(4, DS_AHAU, 8, HM_KUMKU, 1);   # Ok(18980)
```

### Babylonian square root (Heron's method)

`babylonian_sqrt(n, iterations)` takes an **f64** (build it with `f64_from` or
`f64_parse`) and returns `Result` of an f64. This is the YBC 7289 tablet's
sqrt(2) algorithm.

```cyrius
var r_t, r_v = babylonian_sqrt(f64_from(2), 10);
if (is_ok(r_t) == 1) {
    var sqrt2 = result_unwrap(r_t, r_v);   # f64 ~ 1.414213562373095
}
var sx = to_sexagesimal(3600);           # vec [1, 0, 0]
var triples = generate_plimpton_triples();   # vec of the 15 tablet rows
var next = saros_cycle(f64_from(2451545));   # f64 JDN one Saros (6585.32 days) later
```

### Greek: Sieve of Eratosthenes

`sieve(limit)` returns a vec of the primes up to `limit`. GCD/LCM are plain integer
functions; `archimedes_pi` returns a register pair of bounds.

```cyrius
var primes = sieve(100);                 # vec [2, 3, 5, 7, 11, ... , 97]
var n = vec_len(primes);                 # 25
var g = greek_gcd(48, 18);               # 6
var l = greek_lcm(12, 18);               # 36
# Archimedes' bounds on pi after 5 doublings (a register pair: inner, outer)
var lo, hi = archimedes_pi(5);
var phi = greek_phi();                   # f64 ~ 1.618033988749895
var ratio = fibonacci_ratio(40);         # f64 -> approaches phi
var gears = antikythera_gear_ratios();   # vec of named cycles
```

### Roman numerals

`to_roman_str` / `from_roman` are `Result`-returning (range/format validation).
Arithmetic comes in two forms: `roman_*` on integer values, `romannum_*` on
`RomanNumeral` records built with `romannum_from_value`.

```cyrius
var r_t, r_v = to_roman_str(1776);
var s = result_unwrap(r_t, r_v);         # "MDCCLXXVI"
var v_t, v_v = from_roman(str_from("MCMXCIX"));
var n = result_unwrap(v_t, v_v);         # 1999
# Arithmetic on values: each returns a Result of a RomanNumeral
var sum_t, sum_v = roman_add(1776, 248);         # Ok(MMXXIV)
var q_t, q_v = roman_divide(17, 5);              # Ok(quotient III, remainder II)
# Validation (1/0, never panics)
var good = is_valid_roman(str_from("XIV"));      # 1
var bad = is_valid_roman(str_from("IIII"));      # 0
```

Operands are Rust's `u32`: a negative or larger operand is `Err(InvalidBase)`, and a
sum or product past `u32::MAX` is `Err(Overflow)` with Rust's message.

### Cross-calendar `convert()`

The `epoch` capstone bridges every system. Wrap a source date in a
`CalendarDate` variant (`caldate_gregorian`, etc.), then `convert()` routes it
through a Julian Day Number and correlates it across all calendars into a
`MultiCalendarDate`. Read fields with the `mcd_*` accessors.

```cyrius
# Gregorian 2012-12-21 -> all systems (months are 0-based enum codes: GM_DECEMBER)
var greg = gregorian_date_new(2012, GM_DECEMBER, 21);
var r_t, r_v = convert(caldate_gregorian(greg));
if (is_ok(r_t) == 1) {
    var mcd = result_unwrap(r_t, r_v);
    var jdn = mcd_jdn(mcd);              # f64 Julian Day Number (2456282.5)
    var lc = mcd_mayan_lc(mcd);          # Long Count record (0 before the epoch)
    var age = mcd_age(mcd);              # precessional AgePosition record
}
# Direct JDN correlation (no CalendarDate wrapper); no init call is needed
var c_t, c_v = correlate(J2000_JDN);
var bp = bp_to_jdn(f64_from(12800));     # 12,800 years Before Present -> JDN
var pos = precessional_age(J2000_JDN);   # AgePosition record
var sages = seven_sages();               # vec of the six Seven Sages traditions
```

The module constants fill themselves in on first use (from 3.1.3), so `*_init()` calls
are optional. A month outside its calendar's range, a day below 1, an unknown variant,
or a non-finite or |JDN| > 1e14 input is an `Err(InvalidDate)`, never a crash or a
garbage `Ok`.

### JSON round-trip (serial layer)

Every domain type has `<type>_to_json(p)` → `Str`, `<type>_from_json(v)` →
pointer, and `<type>_eq(a, b)` for verification. Parse a JSON string into a
`bayan` value with `sk_json_parse` before calling `_from_json`.

```cyrius
var r_t, r_v = long_count_from_julian_day(2456283);
var lc = result_unwrap(r_t, r_v);
var js = longcount_to_json(lc);          # {"baktun":13,"katun":0,"tun":0,"uinal":0,"kin":0}
var back = longcount_from_json(sk_json_parse(js));
var same = longcount_eq(lc, back);       # 1
```

A decoder never crashes on malformed JSON (`tests/tcyr/serial_malformed.tcyr` feeds
every one of them hostile input), but unlike serde it does not return an `Err` either:
a missing or mistyped enum field decodes to the code -1, which renders as `"?"` and which
the domain functions (`convert`, `*_to_jdn`) reject. A `Result`-returning decoder API is
on the roadmap.

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
- **Errors** never panic. Fallible calls return a `Result` pair; check `is_ok(r_t)`
  before `result_unwrap(r_t, r_v)`. The adversarial suite enforces this.
- **Logging** is off unless initialized, as the Rust crate's was: call
  `sankhya_log_init()` (or `sankhya_log_init_level(level)`) to see sankhya's sakshi
  traces. Until then no log site prints or formats anything.

## Testing & Benchmarks

```sh
cyrius test tests/tcyr        # 35 suites -> "35 passed, 0 failed"
cyrius bench tests/sankhya.bcyr # 24 benchmarks
```
