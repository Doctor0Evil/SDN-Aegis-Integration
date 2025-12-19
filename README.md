# SDN-Aegis-Integration

SDN-Aegis-Integration is a Rust-first, safety-critical SDN–IoT gateway that fuses anomaly-aware AVA-MA analysis with AU.ET/CSP energy ledgers and ICNIRP/IEC-aligned safety envelopes, enforcing non-minting, bounded-risk flows for neuromorphic, BCI, and smart-city cybernetic infrastructure.

## Repository layout

See the `BLUEPRINT.json` and top-level files for the canonical layout. Key crates live under `crates/` and include:

- `aegis-core` — shared types and invariants
- `aegis-ledger-auet-csp` — non-minting AU.ET/CSP ledger mapping
- `aegis-safety-envelope` — RF/biological safety envelope checks
- `aegis-ava-ma` — anomaly-aware AVA-MA analysis
- `aegis-attestation-auet` — snapshot hashing and Merkle proofs
- `aegis-sdn-plane` — SDN controller/adapter types
- `aegis-iot-gateway` — gateway, mapper and server
- `aegis-bci-bridge` — BCI-specific mapping and safety hooks
- `aegis-sim-harness` — deterministic simulation and tests
- `sdn-aegis-cli` — CLI for blueprint, shard validation, and sim runs

## Getting started

- Build the workspace: `cargo build --workspace --locked`
- Run tests: `cargo test --workspace --locked`
- Verify blueprint (stub): `cargo run -p sdn-aegis-cli -- blueprint-verify`

Contributions welcome — open issues or PRs with proposed additions or safety proofs.

