# Security Policy

## Scope

Sankhya is a pure mathematics library implementing ancient mathematical systems
(Mayan, Babylonian, Egyptian, Vedic, Chinese, Greek, Roman, Islamic), historical
calendars (Gregorian, Coptic, Persian, Hebrew, Aztec), archaeoastronomy, and
cross-civilizational epoch correlation for Cyrius. The core library performs no
network I/O beyond stdout for test output.

## Attack Surface

| Area | Risk | Mitigation |
|------|------|------------|
| Numerical stability | Overflow in calendar conversions | Checked arithmetic via `rem_euclid`/`div_euclid`; returns `Err` (`sk_fail`) |
| Egyptian fractions | Non-terminating decomposition | Iteration limits; returns error on excessive depth |
| Magic squares | Large allocation for big n | Input validation; size bounds |
| Newton's method | Non-convergence (Khayyam cubics) | Iteration cap (200); multiple starting points; returns error |
| Roman numeral parsing | Non-canonical input | Round-trip validation against canonical form |
| JSON deserialization | Crafted JSON | Hand-written `<type>_from_json` validators over the bayan parser (no serde derive) |
| Floating point | NaN/Infinity propagation | NaN/Infinity validation on all public f64 inputs (`f64_util`) |
| Dependencies | Supply chain compromise | Folded Cyrius stdlib + git-tagged sibling deps (varna, itihas, avatara); no unpinned packages |
| Transitive net/http | Unused HTTP client symbols | Pulled in only to resolve itihas's bundled hoosh client; never invoked by sankhya |

## Supported Versions

| Version | Supported |
|---------|-----------|
| 3.x (Cyrius) | Yes |
| 2.x (Rust, `rust-old/`) | Reference only, not maintained |

## Reporting

- Contact: **security@agnos.dev**
- Do not open public issues for security vulnerabilities
- 48-hour acknowledgement SLA
- 90-day coordinated disclosure

## Design Principles

- No `unsafe` equivalent — all memory via `alloc`/`store64`/`load64`
- No `unwrap()`/`panic!()` equivalent — all errors via `Result` (`Ok`/`sk_fail`)
- NaN/Infinity input validation on all floating-point public functions
- No network I/O in the core library (transitive `net`/`http` symbols resolve
  itihas's bundled hoosh client but are never called)
- Minimal dependency surface — folded Cyrius stdlib (bayan, sakshi, math,
  ganita) plus git-tagged sibling deps (varna, itihas, avatara)
- All data statically defined at compile time
