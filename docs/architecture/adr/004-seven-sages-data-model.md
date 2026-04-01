# ADR-004: Seven Sages Data Model

## Status

Accepted

## Context

The epoch module includes cross-civilizational Seven Sages tradition data. This data is semi-structured (names, source texts, star associations, narratives) and varies significantly across traditions. The data must be serializable, extensible, and zero-copy where possible.

## Decision

Use a `SagesTradition` struct with `Cow<'static, str>` fields, returned as a freshly allocated `Vec` from `seven_sages()`.

## Rationale

- `Cow<'static, str>` allows all string data to be `Borrowed` from static memory (zero-copy) when returned from the built-in function, while still supporting `Owned` variants for deserialized or user-constructed instances.
- The struct is flat and serializable — no nested hierarchies or trait objects.
- A `Vec` return (rather than `&'static [SagesTradition]`) avoids lifetime complexity while maintaining the `Cow::Borrowed` optimization.
- The `Civilization` enum ties each tradition to its corresponding sankhya module.

## Alternatives Considered

- **Static array**: `&'static [SagesTradition]` — cleaner but `Cow<'static, str>` fields require complex lifetime annotations in static contexts.
- **HashMap**: Less structured, harder to serialize consistently.
- **Separate files/data format**: External JSON/TOML data — adds I/O to a pure computation library, violating sankhya's design principle.

## Consequences

- Each call to `seven_sages()` allocates a new `Vec`, but the string data is zero-copy.
- Adding new civilizations requires modifying the `seven_sages()` function and the `Civilization` enum (which is `#[non_exhaustive]`).
- The `SagesTradition` struct can be extended with new fields in a backwards-compatible way.
