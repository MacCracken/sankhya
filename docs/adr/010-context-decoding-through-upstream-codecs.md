# 010 — Decode `MultiCalendarDate`'s context through itihas's and avatara's own codecs

**Status**: Accepted (v3.3.0)
**Date**: 2026-11-20

## Context

`correlate()` returns a `MultiCalendarDate` that carries four context vecs from the sibling
deps: itihas eras, civilizations and events, and the seven avatara Saptarishi profiles. The
Rust crate derived serde for all of it, so `serde_json::from_str` rebuilt every record, and
it wrote `saptarishi_profiles: Vec<(String, ArchetypeProfile)>` as an array of
`[name, profile]` tuples carrying the whole profile.

Up to 3.2.0 neither sibling had a decoder, and avatara had no profile serializer. sankhya
therefore wrote three profile fields (`{name, tradition, description}`) per Saptarishi entry,
and its decoder kept the four arrays' raw bayan nodes so that only the element count survived.
The raw nodes were also a crash. Re-encoding a decoded date whose context was not empty passed
a bayan node to `era_to_json` as if it were an itihas record, and the process segfaulted with
exit 139 (roadmap #3).

## Decision

**The codecs live with the types.** sankhya calls them and owns only the glue.

- **itihas 2.7.0** adds `era_from_json`, `civ_from_json` and `event_from_json`. They read the
  shape itihas's own `*_to_json` has written since 3.0, and they return `Ok`, or `Err` of an
  ItihasErr. sankhya forwards that message unchanged as `SKE_DESERIALIZE`.
- **avatara 2.17.0** adds `profile_to_json_into`, `profile_from_json` and `profile_eq` in a
  separate `dist/avatara-json.cyr` bundle (avatara ADR-011). Only this bundle calls bayan, so
  `dist/avatara.cyr` keeps avatara's zero-dependency rule. The profile shape is the Rust
  avatara 1.0.1 serde shape plus `domain`, a field that exists only in the Cyrius avatara.
- **sankhya** takes the json bundle in `[deps.avatara]`. Its bayan comes from the stdlib
  monolith already in `[deps].stdlib`, so the build gains no new dependency.
  `serial_epoch.cyr` writes each Saptarishi entry as serde's `[name, profile]` tuple and
  decodes every element of all four arrays. `multi_calendar_date_eq` compares every field
  of every record.
  - A tuple is checked as serde's tuple visitor checks one: too short gives "invalid length",
    and a third element gives "trailing characters".
  - A name holding U+0000 is refused, because the pair stores the name as a C string. avatara
    refuses one inside a profile for the same reason.

## Consequences

- **Positive:**
  - A `MultiCalendarDate` round-trips losslessly: decoding then re-encoding gives the same
    bytes for every date the tests sample.
  - The 3.2.0 crash is gone.
  - `saptarishi_profiles` again has the structure serde gave it.
- **Negative:**
  - ⚠ **The output changes.** `saptarishi_profiles` goes from `{name, tradition,
    description}` objects to `[name, profile]` tuples. Every other byte of the encoding is
    unchanged.
  - The JSON is larger, about 3.4× at JDN 584283 (3,137 to 10,603 B), because each of the
    seven profiles carries its long texts.
  - `serial/mcd_json_roundtrip` is about 4.3× slower, and the time is spent mostly in
    avatara's profile writer and in bayan's parser. That is the price of a lossless round
    trip; no other bench moves.
- **Neutral:**
  - The eras, civilizations and events keep itihas's Cyrius shape: integer enum codes,
    `founded`/`ended`, and `;`-joined `traits`/`language`. The Rust 1.x serde shape used
    variant names, `founding_year`/`end_year`, and arrays. This was already the case from 3.0.
    It is now fixed by itihas's decoder, so the shape can only move to serde's together with
    itihas.
  - A decode error inside the context carries the sibling's message, not one written by
    sankhya.

## Alternatives considered

- **Own the codecs in sankhya, in the Rust 1.x shape.** Proposed first, and rejected by the
  user. sankhya would have kept a second copy of three itihas record layouts and the avatara
  profile layout, and that copy would drift every time a sibling changed a field.
- **Keep the count-only round trip and fix only the crash.** Rejected. It would have left the
  decoder lossy and the Saptarishi output still unlike serde's.
- **Fold avatara's JSON codec into `dist/avatara.cyr`.** Rejected upstream (avatara ADR-011).
  Every avatara consumer would then need bayan, against avatara's zero-dependency rule.
