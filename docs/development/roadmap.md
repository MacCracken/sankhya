# Development Roadmap

> **Current**: 1.0.0

## v1.1

- [ ] Inca quipu (knot-based recording system)
- [ ] Vedic astronomical computations (Surya Siddhanta)
- [ ] Chinese astronomical calendar (Sexagenary cycle)
- [ ] Greek geometric constructions (compass and straightedge)
- [ ] Expanded Antikythera mechanism simulation

## Gematria & Letter-Value Computation (depends on varna 1.3+)

Consumes varna's `script::numerals` character→number mappings to provide word/phrase-level gematria computation across scripts. Same pattern as Katapayadi (already in sankhya) but generalized.

- [ ] **`gematria` module** — word value computation consuming varna's `char_value()` API
- [ ] **Hebrew gematria**: Standard, ordinal, reduced (Mispar Gadol/Siduri/Katan). Word matching by equal value
- [ ] **Greek isopsephy**: Word/phrase value computation, historical cipher applications
- [ ] **Arabic abjad calculation**: Word values, traditional numerology
- [ ] **English simple gematria**: a=1..z=26, ordinal and reduced methods
- [ ] **Cross-script equivalence**: Find words with equal gematria values across scripts (Hebrew↔Greek↔Arabic)
- [ ] **Classical cipher primitives**: Caesar shift, Vigenere table generation, substitution cipher mapping — all built on varna's character↔number round-trip. Foundation for future crypto crate
- [ ] **itihas bridge**: Historical context for when/where each gematria system was used

## Future

- [ ] avatara integration — optional dep on avatara crate to enrich Seven Sages with archetype profiles (IncarnateSage personality data, system prompts, tradition metadata)
- [ ] itihas integration — optional dep on itihas for historical context (calendar epoch dates, civilization metadata for ancient math systems)
- [ ] AI-assisted historical date correlation
- [ ] Cross-civilization calendar converter
- [ ] Interactive visualization examples
- [ ] WASM target support
