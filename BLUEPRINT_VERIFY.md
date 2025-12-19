# BLUEPRINT_VERIFY — Spec for `aegis blueprint-verify`

Purpose
- Define the exact, canonical behavior of the `aegis blueprint-verify` CLI command so implementers and CI can reproduce the SHA‑256 validity stamp deterministically.

Inputs
- Workspace member list from `Cargo.toml` (workspace members array). Include both crate paths and `sdn-aegis-cli`.
- `QPU.Datashard.SDN-Aegis.v1.aln` file(s) that enumerate vnode rows.
- ALN dataset files in `datasets/` (e.g., `ALN.SDN-Aegis.EnergyModel.v1.aln`).
- `BLUEPRINT.json` file which contains the stored `sha256` field (trusted source for CI comparisons).

Canonical JSON output (what to compute)
1. Gather items:
   - `modules`: list of crate names in workspace (sorted lexicographically)
   - `datashards`: list of datashard file paths and a canonical digest per file (digest algorithm: SHA‑256 of UTF‑8 normalized content, newline normalised to LF)
   - `datasets`: list and digest as above
   - `meta`: mapping of constants found in the repo (e.g., `c_E`, `c_S`, `d_aln`) if discoverable; otherwise require that `BLUEPRINT.json` contains these explicitly.
2. Canonicalization rules:
   - Sort all top‑level keys lexicographically.
   - For objects and arrays, sort keys and items (where semantic ordering is not required) to produce deterministic JSON.
   - Use UTF‑8; compact JSON (no extra whitespace) except ensure consistent `\n` endings.
3. Serialization: produce a UTF‑8 byte sequence of canonical JSON, then compute `sha = SHA256(bytes)`.
4. Output format: hex string with `0x` prefix and lowercase hex (e.g., `0x9f2c...`).

Behavior
- If `sha` equals the stored `BLUEPRINT.json.sha256`, print `OK: blueprint hash matches: <sha>` and exit 0.
- If it differs, print `ERROR: blueprint mismatch: computed <sha> != stored <stored_sha>` and exit non‑zero.
- In verbose mode (`--show-canonical`), print the canonical JSON and the computed hash.

Failure and update semantics
- Changing formulas, dataset values, or datashards must update `BLUEPRINT.json` in the same PR and include a short `BLUEPRINTCHANGE.md` explaining why; CI must require this.

Examples
- `cargo run -p sdn-aegis-cli -- blueprint-verify` → `OK: blueprint hash matches: 0xe9...`
- `cargo run -p sdn-aegis-cli -- blueprint-verify --show-canonical` → prints canonical JSON and hash

Note to implementers
- Keep canonicalization deterministic and explicit in tests. Provide unit tests that show the same input (possibly reordered in the workspace) yields the same canonical JSON and hash.
