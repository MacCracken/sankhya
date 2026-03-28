# Security Policy

## Scope

Sankhya is a pure mathematics library implementing ancient mathematical systems (Mayan, Babylonian, Egyptian, Vedic, Chinese, Greek) for Rust. The core library performs no I/O and contains no `unsafe` code.

## Attack Surface

| Area | Risk | Mitigation |
|------|------|------------|
| Numerical stability | Overflow in calendar conversions | Checked arithmetic; returns `Err(OverflowError)` |
| Egyptian fractions | Non-terminating decomposition | Iteration limits; returns error on excessive depth |
| Magic squares | Large allocation for big n | Input validation; size bounds |
| Serde deserialization | Crafted JSON | Enum validation via serde derive |
| Dependencies | Supply chain compromise | cargo-deny, cargo-audit in CI; minimal deps |

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x | Yes |

## Reporting

- Contact: **security@agnos.dev**
- Do not open public issues for security vulnerabilities
- 48-hour acknowledgement SLA
- 90-day coordinated disclosure

## Design Principles

- Zero `unsafe` code
- No `unwrap()` or `panic!()` in library code — all errors via `Result`
- All public types are `Send + Sync` (compile-time verified)
- No network I/O
- Minimal dependency surface (hisab, serde, thiserror, tracing)
