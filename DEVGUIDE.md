# DEVGUIDE — Developer & IDE rules for RegStack

Purpose
- This guide prescribes the minimal engineering rules required for every module to be auditable, provable, and CI‑enforced.

Function & module rules
- Every public function that affects Energy, Ledger, or Safety **must** include:
  - A one‑line mathematical formula and an invariant comment (e.g., "Maps source balance B to internal units using c_E,c_S,d_aln — monotone, floor rounding").
  - Unit tests for known inputs and property tests for invariants.
- Floating→integer conversions must always use an explicit helper (e.g., `to_u128_floor`), with documented behavior for NaN and overflow.

State machine rules
- All state machines must implement these helpers and make them public for testing:
  - `hash_item(item) -> String` — canonical serialization & SHA‑256 hex with 0x prefix.
  - `apply_item(item) -> Result<NewState, Error>` — applies and returns new state atomically; must validate invariants first.
  - `replay_log(events) -> State` — deterministic replay that verifies each `preveventhash`.

Admin adjustments
- Admin adjustments must be represented as `EnergyReason::AdminAdjust { admin_id, reason }` and require a signature; the ledger must log these and require `ledger-audit-admin` to produce a signed explanation.

Testing & CI
- CI jobs (RegStack profile) must include:
  - `cargo test --workspace --locked --features=regstack` (or a separate profile as defined) and fail on any invariant breach.
  - `aegis blueprint-verify` step that recomputes and compares the blueprint hash.
  - Property tests from `REGSTACK_TESTPLAN.md` executed with fixed RNG seeds and recorded artifacts.

Documentation
- Inline derivations: each algorithm file must include a short derivation or reference to the canonical math in `REGSTACK.md`.
- In PRs that change formulas/thresholds: the PR title must start with `REGSTACK:` and the body must include the old vs new values, the reason, and the expected impact on hashes.

Commit & release policies
- Changes to `BLUEPRINT.json` MUST include a `BLUEPRINTCHANGE.md` explaining the change and the recomputed `BLUEPRINTHASH`.
- Tagging releases: create a `regstack-vX.Y.Z` tag with signed SHA tag and attach SafetyEpoch artifacts used for that release.

IDE agent rules (for automated contributors)
- Agents must not commit code without referencing a test and a `REGSTACK_TESTPLAN.md` test case.
- Agents must run the `blueprint-verify` local check before opening a PR and include computed `BLUEPRINTHASH` in the PR body.
