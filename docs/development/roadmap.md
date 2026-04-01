# Development Roadmap

> **Current**: 1.1.0

## v1.2 — Cross-Civilization Calendar Converter

Unified any-to-any calendar conversion using JDN as the internal pivot, following the algorithms in Dershowitz & Reingold, *Calendrical Calculations* (4th ed., Cambridge University Press, 2018).

- [ ] Unified `convert()` API: takes a date in any supported system, returns all others
- [ ] JDN as the internal pivot for all conversions
- [ ] Gregorian calendar (currently only Julian year approximation)
- [ ] Egyptian civil calendar (365-day, no leap) with proper month/day output
- [ ] Hebrew calendar — Metonic cycle, molad calculation (Dershowitz & Reingold ch. 8)
- [ ] Hindu Panchanga — tithi, nakshatra, yoga, karana (Dershowitz & Reingold ch. 10; Surya Siddhanta)
- [ ] Coptic/Ethiopian calendars (Dershowitz & Reingold ch. 4)
- [ ] Persian/Solar Hijri — Jalaali algorithm, 3000-year accuracy (Dershowitz & Reingold ch. 15)
- [ ] Chinese Sexagenary cycle — Heavenly Stems + Earthly Branches, 60-year cycle
- [ ] Aztec Tonalpohualli (260-day) / Xiuhpohualli (365-day) — natural extension of existing Mesoamerican work

Sources:
- Dershowitz & Reingold, *Calendrical Calculations*, 4th ed. (2018) — reference algorithms for 20+ calendar systems
- Time4J (Java) — gold standard for calendar coverage, no Rust equivalent exists
- VedicDateTime (R) — Panchanga conversion reference

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

## v1.5 — Archaeoastronomy Computations

No programmatic library serves archaeoastronomers outside Stellarium. Sankhya's epoch module is positioned to fill this gap.

- [ ] Heliacal rising/setting calculations for arbitrary stars at arbitrary locations and dates
- [ ] Precession-corrected star positions for deep historical dates (reliable to ~3000 BCE)
- [ ] Monument orientation analysis — given lat/lon and bearing, compute aligned celestial events
- [ ] Horizon profile computations with altitude/azimuth/declination overlays

Sources:
- Ruggles, *Archaeoastronomy and Ethnoastronomy: Building Bridges Between Cultures* (Cambridge, 2011)
- Schaefer, "Heliacal Rise Phenomena", *Journal for the History of Astronomy* (1987)
- Stellarium ArchaeoLines plugin (reference implementation for visual archaeoastronomy)

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

- [ ] Inca quipu (knot-based recording system)
- [ ] Greek geometric constructions (compass and straightedge)
- [ ] Expanded Antikythera mechanism simulation
- [ ] AI-assisted historical date correlation
- [ ] Interactive visualization examples
- [ ] WASM target support
