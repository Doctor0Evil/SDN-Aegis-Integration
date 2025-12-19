# RegStack Test Plan — High‑level property & integration tests

Purpose
- Describe deterministically what tests must exist and what they must assert so implementers and CI can write and run them.

1) Ledger property tests (proptest / fuzz)
- Monotonicity test for mapping:
  - Strategy: random `B` (u128 in source units) and fixed params `(d_src,d_aln,c_e,c_s)`; assert `map_source_to_internal(B2) >= map_source_to_internal(B1)` when `B2 >= B1`.
- Non‑minting global property:
  - Strategy: generate many mapping events with bounded source balances summing to `S`. Use `map_source_to_internal` on each and assert `sum(auet_units) <= MAXTOTALAUET` and `sum(csp_units) <= MAXTOTALCSP`.
- Sequence & hash chain:
  - Create random sequences of ledger events; after each `apply`, assert `seq` increments, `preveventhash` equals previous `eventhash`, and tampering with any previous event leads to hash mismatch on replay.

2) RadEnvelope proptests
- Sequence safety invariants:
  - Strategy: generate random sequences of deltas that pass `can_apply` and call `apply`; assert after each step every axis ≤ configured maxima.
- Rejection test:
  - Strategy: craft deltas that would exceed maxima and assert `can_apply` false and `apply` rejects.
- Decay and recovery test:
  - Strategy: set a SAR value, run `step_recovery(dt)` with given `k` and assert `0 ≤ x_after ≤ x_before` and exponential bound holds.

3) Non‑minting flow tests
- Flow envelope exhaustion:
  - Simulate a `NonMintingFlowEnvelope` for a flow; generate events that burn AU.ET/CSP and increment RadEnvelope; assert flows are rejected once any cap exhausted.

4) Simulation scenarios (integration tests)
- BCI lane overload: simulate repeated high‑SAR bursts from BCI flows and assert the safety envelope rejects further BCI allocations once thresholds reached.
- IoT storm + SDN reroute: simulate flash‑crowd device telemetry; assert energy mapping + ledger supply remain within caps after SDN policy changes.
- Smart‑city AR mobility: simulate TSN/latency guarantees with AU.ET budgets and assert no deficits or safety violations.

5) CLI & blueprint tests
- `blueprint-verify` acceptance test:
  - Provide canonical workspace layout + dataset fixtures; compute hash locally with the spec algorithm and assert `aegis blueprint-verify` prints the same hash and returns 0.
- Datashard validation test:
  - Feed intentionally malformed `QPU.Datashard` rows and assert `qpu-validate-shard` fails with deterministic error codes.

6) Audit & SafetyEpoch tests
- SafetyEpochManifest determinism:
  - Produce a snapshot, compute `epochhash`, then replay serialization and assert recomputed hash matches.
- SafetyEpoch tamper detection:
  - Flip one byte in the snapshot and assert hash mismatch.

CI expectations
- The CI must run the above tests (unit/proptest/integration) and fail if any invariant is violated.
- The CI must publish SafetyEpoch artifacts for review and store them in `snapshots/` with signed metadata.

Notes for implementers
- Tests must be reproducible and deterministic. Use fixed seeds for any RNG in CI tests and document the seeds in test metadata.
- Use proptest strategies that cover edge values (0, maxima, large numbers near u128 limits) and typical ranges.
- Any test that requires external hardware (BCI devices, Loihi) must be tagged `#[ignore]` in normal CI and run only in specialized integration pipelines with test rig attachments.

When a collaborator implements a test, they must reference the section above in the PR body (e.g., "Implements REGSTACK_TESTPLAN: Ledger property tests — non‑minting").
