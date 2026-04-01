# Development Roadmap

> **Current**: 2.0.0

## v1.2 — Cross-Civilization Calendar Converter + Archaeoastronomy (SHIPPED)

- [x] Unified `convert()` API: takes a date in any supported system, returns all others
- [x] JDN as the internal pivot for all conversions
- [x] Gregorian calendar (proleptic, March-based algorithm)
- [x] Hebrew calendar — Metonic cycle, molad, 4 dehiyyot, 6 year types
- [x] Coptic calendar (13 months, Anno Martyrum)
- [x] Persian/Solar Hijri — Jalaali arithmetic algorithm, 2820-year cycle
- [x] Chinese Sexagenary cycle — Heavenly Stems + Earthly Branches, 60-year cycle
- [x] Aztec Tonalpohualli (260-day) / Xiuhpohualli (365-day) with Caso correlation
- [x] Archaeoastronomy module: coordinate types, star catalog (20 stars), precession (IAU/Lieske), heliacal rising (Schaefer 1987), monument alignment

### Deferred to v1.3+

- [ ] Egyptian civil calendar (365-day, no leap) with proper month/day output
- [ ] Hindu Panchanga — tithi, nakshatra, yoga, karana (Dershowitz & Reingold ch. 10; Surya Siddhanta)

## v1.3 — Surya Siddhanta & Nine Chapters

Two richest unimplemented ancient mathematical sources. No library in any language implements these algorithms.

### Surya Siddhanta (Indian astronomical treatise, c. 400 CE)

- [ ] Jya (sine) table construction — the original trigonometric table, 24 values at 3.75° intervals
- [ ] Planetary mean position algorithms (Sun, Moon, five visible planets)
- [ ] Eclipse prediction using lunar node calculations
- [ ] Precession model (ayana — distinct from the Babylonian/Greek model already in epoch)

Sources:
- Burgess, *Surya Siddhanta: A Textbook of Hindu Astronomy* (1858, reprinted)
- Plofker, *Mathematics in India* (Princeton, 2009) ch. 4
- Validated computationally in Ramasubramanian et al., *Indian Journal of History of Science* (2025)

### Nine Chapters on the Mathematical Art (Jiu Zhang Suan Shu, c. 100 BCE–50 CE)

- [ ] Chapter 1: Area formulas for fields (rectangle, triangle, trapezoid, circle, segment, annulus)
- [ ] Chapter 4: Square and cube root extraction (iterative algorithm)
- [ ] Chapter 8: Fangcheng (simultaneous linear equations / Gaussian elimination, ~2000 years before Gauss)
- [ ] Chapter 9: Right triangle problems (Gougu theorem applications)

Sources:
- Shen Kangshen, Crossley & Lun, *The Nine Chapters on the Mathematical Art: Companion and Commentary* (Oxford, 1999)
- Dauben, *The Mathematics of Egypt, Mesopotamia, China, India, and Islam* (Princeton, 2007) ch. 3

## v1.4 — Expanded Primary Sources

### Rhind Papyrus (problems beyond unit fractions)

Sankhya implements unit fractions and doubling multiplication; 84 total problems are catalogued.

- [ ] Problems 24–34: Aha (linear equation) problems — "a quantity, its 1/7 added to it, becomes 19"
- [ ] Problems 41–60: Volume/area computations — granaries (cylinders), pyramids, truncated pyramids
- [ ] Problems 61–84: Series, progressions, and practical division problems
- [ ] Egyptian pi: Problem 50 implies π ≈ 256/81 (~3.1605); Moscow Papyrus may encode a different value — support both

Sources:
- Chace, *The Rhind Mathematical Papyrus* (NCTM, 1927; reprinted 1979)
- Imhausen, *Mathematics in Ancient Egypt: A Contextual History* (Princeton, 2016)

### Plimpton 322 — Multiple Scholarly Interpretations

Active scholarly dispute on interpretation. Sankhya should support all three readings.

- [ ] Classical interpretation: Pythagorean triples table (Neugebauer & Sachs, 1945)
- [ ] Reciprocal pairs interpretation: regular number table (Robson, *Historia Mathematica*, 2001)
- [ ] Exact trigonometry interpretation: sexagesimal ratio table (Mansfield & Wildberger, *Historia Mathematica*, 2017)

### Al-Kashi (Islamic mathematics, c. 1420 CE)

- [ ] Iterative algorithm for sin(1°) — converges to arbitrary precision, predates Western methods by centuries
- [ ] Al-Kashi's pi: 16 decimal places (2π = 6.2831853071795865), computed c. 1424

Sources:
- Berggren, *Episodes in the Mathematics of Medieval Islam* (Springer, 2016) ch. 9
- Katz, *A History of Mathematics* (3rd ed., Pearson, 2009)

## Expanded Calendar Coverage

Additional calendar systems with published arithmetic algorithms.

- [ ] **Ethiopian** — Shares Coptic structure, different epoch (Aug 29, 8 CE) and month names. Easy add from existing `coptic.rs` pattern
- [ ] **Julian** — Proleptic Julian calendar, distinct from Gregorian for pre-1582 dates. Critical for historical date interpretation
- [ ] **Hindu Panchanga** — Lunisolar, tithi/nakshatra/yoga/karana. Most complex remaining calendar. Covered by Time4J and D&R ch. 10

Sources:
- Dershowitz & Reingold, *Calendrical Calculations* (4th ed., 2018) — chs. 4, 3, 10
- Time4J (Java) — reference implementation for Hindu and Ethiopian calendars

## Validation Against Published Test Vectors

Dershowitz & Reingold publish tabulations (1900–2200) as accepted test vectors. Every shared calendar system should be validated.

- [ ] D&R test vectors for Gregorian ↔ Hebrew conversions
- [ ] D&R test vectors for Gregorian ↔ Islamic conversions
- [ ] D&R test vectors for Gregorian ↔ Persian conversions
- [ ] D&R test vectors for Gregorian ↔ Coptic conversions
- [ ] D&R test vectors for Mayan Long Count correlation dates

Sources:
- Dershowitz & Reingold calendar code and test data: https://www.reingold.co/calendars.shtml

## Enhanced Archaeoastronomy

v1.2 shipped: coordinate types, 20-star catalog, IAU precession (Lieske 1977), heliacal rising (Schaefer 1987), solar position, monument alignment. Gaps identified from Skyfield/Astropy/Stellarium comparison.

- [ ] **IAU 2006 precession model** (Capitaine et al.) — supersedes Lieske 1977, better deep-time accuracy
- [ ] **Atmospheric refraction model** — Affects heliacal rising predictions by ~0.5°. Standard model from Meeus ch. 16
- [ ] **Lunar standstill computations** (major/minor) — Critical for Neolithic monument analysis (Stonehenge, Callanish). 18.6-year cycle
- [ ] **Eclipse prediction for historical dates** — Saros-based forward prediction from known eclipses. Tractable without full ephemeris
- [ ] **Horizon profile computations** — Given terrain altitude data, compute effective horizon for a location
- [ ] **Heliacal setting** — Complement to heliacal rising (last visibility at dusk)
- [ ] **Optional `astro` crate integration** — The Rust `astro` crate (crates.io) has VSP087 planetary positioning. Feature-gated bridge for professional-grade solar/lunar positions

Sources:
- Capitaine et al., "Expressions for IAU 2000 precession quantities", *Astronomy & Astrophysics* 412, 567–586 (2003)
- Meeus, *Astronomical Algorithms* (2nd ed., 1998), ch. 16 (refraction), ch. 54 (eclipses)
- Skyfield (Python) — reference for professional positional astronomy: https://rhodesmill.org/skyfield/
- Astropy — reference for coordinate transformations: https://www.astropy.org/
- Stellarium ArchaeoLines plugin — reference for visual archaeoastronomy

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

## Cross-Civilizational Analysis

Correlations between ancient mathematical systems studied in academia that sankhya can uniquely enable.

- [ ] Shared astronomical constants comparison — Babylonian, Indian, Greek sidereal year values computed and compared programmatically
- [ ] Mathematical transmission mapping — Babylonian base-60 → Greek astronomy → Islamic refinement → Indian adoption, with conversion functions making connections explicit
- [ ] Eclipse cycle correlations — Saros (Babylonian), Metonic (Greek), Indian algorithms applied to the same events
- [ ] Multiple scholarly interpretations as first-class API — enum variants or configuration for disputed ancient values

Sources:
- Hunger & Pingree, *Astral Sciences in Mesopotamia* (Brill, 1999)
- CPAK conference series (Conference on Precession and Ancient Knowledge) — precession awareness across cultures
- Pingree, *From Astral Omens to Astrology* (Rome, 1997) — transmission of Babylonian astronomy to India and Greece

## Future

### World-Class / Professional Education Level

The goal: sankhya becomes the definitive reference implementation for ancient computational traditions — suitable for professional archaeoastronomers, academic researchers, and university-level education. No competing library in any language covers this scope.

Competitive position (as of v1.2.0):
- **Time4J** (Java): ~15 calendar systems, no ancient math, no astronomy
- **ICU4X** (Rust): ~10 calendars, modern i18n focus, no ancient math
- **convertdate** (Python): ~10 calendars, no math, no astronomy
- **Skyfield/Astropy** (Python): professional positional astronomy, no calendars, no ancient math
- **Stellarium** (C++): visual archaeoastronomy, GUI-only, not a library
- **sankhya** (Rust): 10+ calendars + 8 civilization math + archaeoastronomy + epoch correlation — **unique**

### Other

- [ ] Inca quipu (knot-based recording system)
- [ ] Greek geometric constructions (compass and straightedge)
- [ ] Expanded Antikythera mechanism simulation
- [ ] AI-assisted historical date correlation
- [ ] Interactive visualization examples
- [ ] WASM target support
