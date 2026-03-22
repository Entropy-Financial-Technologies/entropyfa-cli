# Flexible Reference Pack Installation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add flexible reference-pack packaging, installation, and discovery to `entropyfa` so OSS users, package managers, and platform/container deployments can install the CLI with or without markdown reference packs.

**Architecture:** Keep the binary usable without local packs, but add a first-class reference-pack bundle, profile-aware install logic, and a small `env` discovery command. The CLI repo will own release artifacts, path resolution rules, and installer behavior; platform-specific workspace binding in `entropy-platform` is a separate follow-on plan.

**Tech Stack:** Rust CLI workspace, shell installer, GitHub Actions release workflow, markdown reference artifacts, serde JSON output, Rust integration tests

---

## File Structure

This plan changes only the `entropyfa-cli` repo. The goal is to keep responsibilities clear:

- `install.sh`
  - profile-aware installer
  - downloads binary, packs, or full bundle depending on flags
- `.github/workflows/release.yml`
  - publishes binary-only, reference-pack, and full-bundle release artifacts
- `reference/`
  - canonical markdown reference-pack root shipped as release content
- `cli/src/...`
  - new `env` command and shared path-resolution helpers
- `cli/tests/integration.rs`
  - CLI contract tests for `env --json` and new packaging/discovery behavior
- `README.md` and `docs/openclaw.md`
  - public install and packaging guidance

Do not mix in `entropy-platform` changes here. That repo needs its own plan for image bundling and workspace visibility.

## Planned File Changes

- Create: `reference/manifest.json`
- Create: `reference/README.md`
- Modify: `install.sh`
- Modify: `.github/workflows/release.yml`
- Modify: `README.md`
- Modify: `docs/openclaw.md`
- Modify: `cli/src/main.rs`
- Modify: `cli/src/lib.rs` or the existing command-registration file if present
- Create: `cli/src/commands/env.rs`
- Create: `cli/src/support/reference_paths.rs`
- Modify: `cli/tests/integration.rs`

If command registration lives elsewhere after inspection, keep the new code isolated and only touch the minimum registration files needed.

## Task 1: Establish Canonical Reference-Pack Root In The Repo

**Files:**
- Create: `reference/manifest.json`
- Create: `reference/README.md`
- Test: `cli/tests/integration.rs`

- [ ] **Step 1: Inspect the current reference-pack branch and identify the minimal files needed for a first canonical `reference/` root**

Run:

```bash
find .worktrees/reference-packs-federal-tax-slice/reference -maxdepth 3 -type f | sort
```

Expected: a small set of markdown packs and any existing manifest-like files that can inform the new root.

- [ ] **Step 2: Write the failing integration test for `entropyfa env --json` reporting `packs_present: false` cleanly when `reference/manifest.json` is missing**

Add a focused test in `cli/tests/integration.rs` that:

```rust
#[test]
fn env_json_reports_missing_reference_root_cleanly() {
    // run entropyfa env --json with ENTROPYFA_REFERENCE_ROOT pointing at an empty temp dir
    // assert ok = true
    // assert reference.packs_present == false
    // assert reference.manifest == null
}
```

- [ ] **Step 3: Run the focused test to verify it fails because the command does not exist yet**

Run:

```bash
cargo test -p entropyfa --test integration env_json_reports_missing_reference_root_cleanly -v
```

Expected: FAIL because `env` is not a recognized command or the JSON contract does not exist yet.

- [ ] **Step 4: Create the minimal canonical repo-owned reference root**

Create:

`reference/manifest.json`

Suggested starter content:

```json
{
  "bundle_version": "dev",
  "generated_at": null,
  "categories": {},
  "pack_count": 0
}
```

Create:

`reference/README.md`

Document:
- purpose of the reference root
- that packs are plain markdown artifacts
- that `manifest.json` is the machine-readable index

- [ ] **Step 5: Commit the reference-root scaffolding**

```bash
git add reference/manifest.json reference/README.md cli/tests/integration.rs
git commit -m "chore: add canonical reference-pack root"
```

## Task 2: Add Reference Root Resolution Helper

**Files:**
- Create: `cli/src/support/reference_paths.rs`
- Modify: `cli/src/main.rs`
- Modify: command-registration file if separate
- Test: `cli/tests/integration.rs`

- [ ] **Step 1: Write the failing integration tests for reference-root precedence**

Add tests that exercise:
- explicit `--reference-root`
- `ENTROPYFA_REFERENCE_ROOT`
- default fallback when neither is set

Suggested assertions:

```rust
#[test]
fn env_json_prefers_cli_reference_root_flag_over_env_var() {}

#[test]
fn env_json_uses_env_reference_root_when_flag_absent() {}

#[test]
fn env_json_falls_back_to_default_reference_root() {}
```

- [ ] **Step 2: Run the focused tests and verify they fail**

Run:

```bash
cargo test -p entropyfa --test integration env_json_prefers_cli_reference_root_flag_over_env_var -v
```

Expected: FAIL because no shared resolution logic or `env` command exists yet.

- [ ] **Step 3: Implement a focused path-resolution helper**

Create `cli/src/support/reference_paths.rs` with small pure functions:

```rust
pub enum InstallProfile {
    BinaryOnly,
    Full,
    Platform,
    Unknown,
}

pub struct ResolvedReferenceRoot {
    pub path: PathBuf,
    pub source: &'static str,
}

pub fn resolve_reference_root(
    explicit: Option<&Path>,
    env_var: Option<&OsStr>,
    home_dir: Option<&Path>,
    profile_hint: InstallProfile,
) -> ResolvedReferenceRoot { /* ... */ }
```

Rules:
- explicit flag wins
- env var next
- otherwise use profile default
- default to `~/.entropyfa/reference` for non-platform installs
- allow `/opt/entropyfa/reference` only when profile hint is `Platform`

- [ ] **Step 4: Wire the helper into the CLI crate without changing calculator behavior**

Touch only the minimum registration or support files needed so later commands can reuse the helper.

- [ ] **Step 5: Run the focused tests to verify they pass**

Run:

```bash
cargo test -p entropyfa --test integration env_json_uses_env_reference_root_when_flag_absent -v
```

Expected: PASS once the helper is wired into the new command in the next task, or still failing only because the command is not yet registered.

- [ ] **Step 6: Commit the path-resolution helper**

```bash
git add cli/src/support/reference_paths.rs cli/src/main.rs cli/tests/integration.rs
git commit -m "feat: add reference-root resolution helper"
```

## Task 3: Add `entropyfa env --json`

**Files:**
- Create: `cli/src/commands/env.rs`
- Modify: `cli/src/main.rs`
- Modify: command-registration file if separate
- Test: `cli/tests/integration.rs`

- [ ] **Step 1: Write the failing integration tests for the `env --json` contract**

Add tests that assert:
- `entropyfa env --json` exits successfully
- the response includes version, binary path, resolved reference root, packs_present, and manifest metadata when available
- plain `entropyfa env` prints a concise human summary on stderr or stdout according to repo conventions

Suggested JSON assertion shape:

```rust
assert_eq!(body["ok"], true);
assert!(body["data"]["binary_path"].is_string());
assert!(body["data"]["version"].is_string());
assert!(body["data"]["reference"]["resolved_root"].is_string());
assert!(body["data"]["reference"]["packs_present"].is_boolean());
```

- [ ] **Step 2: Run the focused test and verify it fails**

Run:

```bash
cargo test -p entropyfa --test integration env_json_reports_installed_manifest_metadata -v
```

Expected: FAIL because the command does not exist yet.

- [ ] **Step 3: Implement the minimal `env` command**

Create `cli/src/commands/env.rs` with:
- CLI arg parsing for `--json`
- optional `--reference-root`
- manifest read logic
- JSON envelope output matching current CLI style

Suggested response shape:

```json
{
  "ok": true,
  "data": {
    "version": "0.1.0",
    "binary_path": "/path/to/entropyfa",
    "reference": {
      "resolved_root": "/path/to/reference",
      "resolution_source": "flag|env|default",
      "packs_present": true,
      "manifest": {
        "bundle_version": "v2026.03.0",
        "generated_at": "2026-03-22T00:00:00Z",
        "categories": { "tax": ["2026"] },
        "pack_count": 7
      }
    },
    "install_profile": "full"
  }
}
```

Keep the manifest permissive enough that missing fields degrade gracefully.

- [ ] **Step 4: Register the command in the CLI**

Update the top-level CLI registration so:

```bash
entropyfa env --json
```

works without affecting existing command behavior.

- [ ] **Step 5: Run the focused tests to verify they pass**

Run:

```bash
cargo test -p entropyfa --test integration env_json_reports_missing_reference_root_cleanly -v
cargo test -p entropyfa --test integration env_json_reports_installed_manifest_metadata -v
```

Expected: PASS.

- [ ] **Step 6: Commit the `env` command**

```bash
git add cli/src/commands/env.rs cli/src/main.rs cli/tests/integration.rs
git commit -m "feat: add env command for install discovery"
```

## Task 4: Teach The Release Workflow To Publish Pack And Full Bundles

**Files:**
- Modify: `.github/workflows/release.yml`
- Modify: `reference/manifest.json` if needed for packaging assumptions
- Test: local dry-run inspection of workflow logic

- [ ] **Step 1: Write a small release-workflow checklist in comments or plan notes before changing YAML**

The workflow must publish:
- binary-only tarballs
- one reference-pack bundle
- one full bundle per target

Avoid overengineering. Do not add a second workflow if the existing one can stay readable.

- [ ] **Step 2: Add a failing smoke check by scripting the intended packaging locally**

Run locally before editing YAML:

```bash
mkdir -p /tmp/entropyfa-release-smoke
tar czf /tmp/entropyfa-release-smoke/reference-test.tar.gz reference
test -f /tmp/entropyfa-release-smoke/reference-test.tar.gz
```

Expected: this only proves the tar layout is viable; there is no workflow support yet.

- [ ] **Step 3: Update `.github/workflows/release.yml` minimally**

Inside the existing `build` job:
- keep the binary tarball packaging
- add a per-target full-bundle tarball that contains:
  - `bin/entropyfa`
  - `reference/...`

Add one non-matrix job or one guarded matrix step to publish:
- `entropyfa-reference-packs-${{ github.ref_name }}.tar.gz`

Do not duplicate the binary build unnecessarily.

- [ ] **Step 4: Review the generated file names for compatibility with the installer**

Target names should stay predictable:
- `entropyfa-x86_64-unknown-linux-musl.tar.gz`
- `entropyfa-full-x86_64-unknown-linux-musl.tar.gz`
- `entropyfa-reference-packs-vX.Y.Z.tar.gz`

- [ ] **Step 5: Commit the workflow update**

```bash
git add .github/workflows/release.yml
git commit -m "build: publish reference-pack and full release bundles"
```

## Task 5: Make `install.sh` Profile-Aware

**Files:**
- Modify: `install.sh`
- Test: ad hoc shell runs against local fake release assets if needed
- Test: `cli/tests/integration.rs` only for CLI-facing effects, not shell installer internals

- [ ] **Step 1: Write the installer behavior matrix in comments before editing**

Cases to support:
- default install behaves like `--profile full`
- `--profile binary-only`
- `--profile full`
- `--profile platform`
- `--reference-dir`
- existing `--system`

- [ ] **Step 2: Write a failing shell smoke script locally**

Use temp dirs and mocked URLs if needed. The goal is to prove argument parsing and install layout before you change the real flow.

Example smoke checks:

```bash
TMP_ROOT=$(mktemp -d)
sh ./install.sh --help
```

Expected initially: there is no profile support yet.

- [ ] **Step 3: Refactor `install.sh` to parse explicit profiles**

Rules:
- default to `full`
- `binary-only` downloads only `entropyfa-<target>.tar.gz`
- `full` downloads the binary tarball and the reference-pack bundle, or the prebuilt full bundle if that keeps the script simpler
- `platform` installs without shell profile mutation and should accept `--reference-dir`

Add:
- `DEFAULT_REFERENCE_DIR="$HOME/.entropyfa/reference"`
- `REFERENCE_DIR="${ENTROPYFA_REFERENCE_ROOT:-...}"` only when not overridden

- [ ] **Step 4: Implement install layouts**

Expected output layouts:

Local full install:

```text
~/.entropyfa/
  bin/entropyfa
  reference/...
```

Platform install example:

```text
/usr/local/bin/entropyfa
/opt/entropyfa/reference/...
```

- [ ] **Step 5: Preserve existing usability**

Keep:
- architecture detection
- latest-release lookup
- cargo-shadowing warning

Do not edit shell profiles for `platform` installs.

- [ ] **Step 6: Run manual installer smoke checks**

Run:

```bash
TMP_ROOT=$(mktemp -d)
sh ./install.sh --profile binary-only --install-dir "$TMP_ROOT/bin"
sh ./install.sh --profile full --install-dir "$TMP_ROOT/full/bin" --reference-dir "$TMP_ROOT/full/reference"
```

Expected:
- binary-only installs only the binary
- full installs binary plus packs

Use temporary local edits or mocked download locations if the live release assets are not yet present. Document the exact limitation in the commit message or notes if fully automated smoke is not possible yet.

- [ ] **Step 7: Commit installer changes**

```bash
git add install.sh
git commit -m "feat: add profile-aware installer for reference packs"
```

## Task 6: Update Docs For OSS And OpenClaw Users

**Files:**
- Modify: `README.md`
- Modify: `docs/openclaw.md`
- Test: human review of commands and paths

- [ ] **Step 1: Write the failing doc expectations as a checklist**

Docs must explain:
- binary-only vs full vs platform installs
- reference-pack path resolution
- `entropyfa env --json`
- that OpenClaw should read packs directly from the filesystem
- that container/home-directory assumptions differ from local installs

- [ ] **Step 2: Update `README.md`**

Add:
- install profiles
- reference-pack bundles
- `entropyfa env --json`
- note that calculators can run without packs if explicit assumptions are supplied

Remove or soften claims that the binary alone is the whole product surface.

- [ ] **Step 3: Update `docs/openclaw.md`**

Shift the OpenClaw guidance from:
- `data coverage` / `data lookup` centric discovery

Toward:
- `compute <command> --schema`
- direct file reads from the installed reference root
- `entropyfa env --json` for path discovery

Keep this doc honest about current platform-specific setup versus generic OSS installs.

- [ ] **Step 4: Commit the doc updates**

```bash
git add README.md docs/openclaw.md
git commit -m "docs: document reference-pack installation profiles"
```

## Task 7: Final Verification

**Files:**
- Verify all changed files

- [ ] **Step 1: Run formatting and integration tests**

Run:

```bash
cargo fmt --all -- --check
cargo test -p entropyfa --test integration
```

Expected: PASS.

- [ ] **Step 2: Run full workspace verification**

Run:

```bash
cargo test --workspace
```

Expected: PASS.

- [ ] **Step 3: Smoke the new command**

Run:

```bash
cargo run -p entropyfa -- env --json
```

Expected: `ok: true` JSON envelope with version, binary path, and reference metadata.

- [ ] **Step 4: Inspect installer help and a temp install path**

Run:

```bash
sh ./install.sh --help
```

Expected: usage text includes `--profile` and `--reference-dir`.

- [ ] **Step 5: Record any release-workflow gaps that cannot be fully tested locally**

If GitHub Actions packaging cannot be end-to-end validated locally, note exactly what was verified and what remains to be validated on the next tag build.

- [ ] **Step 6: Commit final cleanups if needed**

```bash
git add .
git commit -m "test: verify flexible reference-pack installation"
```

Only do this if there are real tracked changes after verification. Do not create an empty commit.

## Follow-On Work Not In This Plan

These items belong to separate plans or repos:

- `entropy-platform` gateway image changes to bundle reference packs under `/opt/entropyfa/reference`
- workspace-visible bind/copy path for agents inside OpenClaw
- migration of calculator schemas to emit `recommended_reference_packs`
- broader deprecation of `data lookup` and `data coverage`

The current plan is intentionally scoped to the `entropyfa-cli` repo so it can land as a clean, testable slice.

