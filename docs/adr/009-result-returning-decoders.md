# 009 — `<type>_from_json` returns a Result, as serde did

**Status**: Accepted (v3.2.0)
**Date**: 2026-11-18

## Context

The Rust crate deserialized with serde: `serde_json::from_str::<T>(s)` returns
`Result<T, serde_json::Error>`. The port's hand-written `<type>_from_json(v)` returned the
record directly. Through 3.1.2 a missing or mistyped field crashed about 44 of them; 3.1.3
stopped the crashes, but a decoder still could not say that its input was wrong. It returned
a record with 0, `""` or -1 in the bad fields, and that garbage surfaced only later, if a
domain function such as `convert` happened to reject it.

Changing the return type of 60 public functions is a breaking change. No repository in this
workspace calls sankhya's decoders: hisab does not use them, and nothing declares
`[deps.sankhya]`. The user chose to replace the API in place and to ship it as 3.2.0.

## Decision

Every `<type>_from_json(v)` returns a Result pair: `Ok(value)`, or
`Err(SkErr{SKE_DESERIALIZE, msg})`. `SKE_DESERIALIZE` (6) is a new error code; the Rust's
decode errors were `serde_json::Error`, not a `SankhyaError` variant, so there was no
existing code to reuse.

Decoding follows what serde's derive did for the Rust type:

- **Structs** must be JSON objects, and every non-`Option` field must be present. Unknown
  keys are ignored.
- **Integers** must be JSON integers in the range of the Rust field type (`u8`, `u32`, `i64`,
  …). A float is the wrong type even when it is integral.
- **f64** fields accept integers and floats. `null` is an Err, because serde_json writes NaN
  and ±Inf as `null` and cannot read them back either.
- **Option** fields are `None` when the key is missing or `null`.
- **Unit enums** are the variant's name as a string.
- **Enums carrying data** are externally tagged objects with exactly one key.
- **No domain validation** is done. A Gregorian day of 40 decodes; `gregorian_to_jdn` rejects
  it.

Messages use serde's wording (`missing field \`year\``, `unknown variant \`Foo\`, expected one
of …`, `invalid type: floating point \`1.5\`, expected u8`, `invalid value: integer \`-5\`,
expected u8`). They lack serde_json's ` at line L column C` suffix, because bayan's value tree
keeps no positions.

The shared helpers (`sk_de_*`) live in `serial_util.cyr`. Each decoder is a short chain of
`?`, so a nested field's error propagates unchanged.

## Consequences

- **Positive**: Malformed input is an `Err` at the decoder, with a message naming the field
  and the expected type, as the Rust crate gave its callers. There is one decoder per type.
- **Negative**: Every caller of a decoder must bind the pair (`var t, v = …`). A single-variable
  bind is a compile error in cyrius ≥ 6.6, so every stale call site fails at its own line rather
  than miscompiling. Strict SemVer would call this 4.0.0; it ships as 3.2.0 by the user's
  choice, and the CHANGELOG says so.
- **Neutral**: The 3.1.3 safety net (`sk_name_at` rendering an out-of-range enum code as `"?"`)
  stays, for records a caller builds by hand. The `*_from_name` helpers remain public.

## Alternatives considered

- **Add `<type>_try_from_json` beside the old decoders** (non-breaking, a true minor): rejected
  by the user. It would have meant 60 more functions, and two decoders per type that answer
  malformed input differently.
- **Validate domain ranges while decoding**: rejected. serde's derive did not, and doing it here
  would give sankhya two places where a calendar rule could be wrong.
