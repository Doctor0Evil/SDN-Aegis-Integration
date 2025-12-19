# RegStack — Regulation‑ready Reference Stack

Purpose
- RegStack is the regulation‑ready profile and spec for SDN‑Aegis: an "evidence machine" that makes safety envelopes, consent traces, and invariants first‑class and independently auditable.

Scope
- This file is the canonical spec for in‑repo invariants, CLI expectations, and acceptance rules that code and CI must implement.

Key invariants (must be preserved by code & CI)

1. Energy mapping
- A_src = B * 10^-d_src
- A_E = c_E * A_src,  A_S = c_S * A_src
- B_E = floor(A_E * 10^{d_ALN}), B_S = floor(A_S * 10^{d_ALN})
- Properties: monotone in B (if B increases, B_E/B_S non‑decreasing), strictly non‑minting (no code path may create additional AU.ET/CSP except via the approved mapping), and integer rounding is done with explicit floor with NaN/overflow guards.

2. Global caps
- Global supply caps `MAXTOTALAUET` and `MAXTOTALCSP` are configured per deployment.
- Any event that would cause total supply > cap must be rejected with a deterministic error; supply must be tracked in a single authoritative `GlobalSupply` structure.

3. Ledger rules
- Append‑only event ledger: every event has `seq = last_seq + 1`, `preveventhash = previous.eventhash`, and `eventhash = H(serialized_event)`.
- No accepted event may create negative balances on any axis.
- No event may increase total supply beyond configured caps; any attempt to do so must be rejected.

4. Safety envelopes (RadEnvelopeQpu)
- RadEnvelope axes: `dose (d_ion)`, `SAR (sar_mw_per_kg)`, `current density (j_ma_per_m2)`.
- `can_apply(delta)` uses saturating add checks per axis compared to per‑axis maxima, returns `false` if any axis would exceed its cap.
- `apply(delta)` clamps at maxima to preserve non‑negativity and upper bounds; some axes (dose) do not decay; others (SAR/current) have `step_recovery(delta_t)` modelled with an exponential decay `x * exp(-k*dt)`.
- Invariant: if only accepted actions pass `can_apply`, the system can never reach a state with any axis > configured max.

5. SafetyEpochManifest
- Safety epoch is a canonical snapshot containing: per‑agent EnergyState, per‑agent RadEnvelope, SafetyMeta, and `prevepochhash`.
- Epoch hash = SHA‑256(canonical JSON of snapshot + prevepochhash). Any change to inputs changes the `epochhash`.
- Safety epochs are retained and published to the audit logs; any rollback must be explicit and auditable.

6. Consent (ALN‑ConsentTrace)
- Consent traces are DID‑anchored, append‑only events.
- PII stored off‑chain; on‑chain entries are hashes referencing encrypted payloads.
- Revocation is an append event and does not break the hash chain or the ability to audit historical consent (though off‑chain raw PII can be erased per GDPR/CCPA).

Merge / acceptance rules
- Nothing may be merged into a RegStack profile branch unless the PR description references the exact invariant(s) in `REGSTACK.md` it implements and the accompanying tests (or test‑plans) are present.

CLI surface (required commands)
- `aegis blueprint-verify` — recompute canonical `BLUEPRINT.json` and compare SHA‑256 to stored hash; exit non‑zero on mismatch.
- `aegis qpu-validate-shard <path>` — parse datashard(s), validate schema and deterministic digest.
- `aegis ledger-apply <event.json>` — apply an event through the ledger engine with deterministic validation and return new eventhash.
- `aegis ledger-audit-admin <epoch|range>` — produce an audit report for auditors.

Reporting & CI behavior
- CI must run `aegis blueprint-verify` on every PR and fail the job when the computed hash differs from the stored value (unless the PR explicitly updates `BLUEPRINT.json` and documents why).
- CI must fail on any property test violation (non‑minting, negative balances, envelope violations).
- Any admin adjustment must appear in the audit log and be signed by an authorized key.

Doc and process
- This file is authoritative for regulators and ethics boards.
- Implementation details belong in `DEVGUIDE.md` and `REGSTACK_TESTPLAN.md` (test descriptions).

Contact & change control
- Any change to a formula, compression constant, or envelope thresholds triggers a required `BLUEPRINT.json` bump and a documented reason in PR body.
