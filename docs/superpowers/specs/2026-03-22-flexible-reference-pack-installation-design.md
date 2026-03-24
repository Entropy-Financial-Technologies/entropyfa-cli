# Flexible Reference Pack Installation Design

## Purpose

This spec defines how `entropyfa` should ship and install its compute binary and markdown reference packs so the project works cleanly in three environments:

- open source local installs
- third-party packaging and custom platform deployments
- Entropy's OpenClaw gateway containers

The goal is to keep the CLI as the deterministic compute layer while making reference packs available as plain filesystem artifacts that agents can read directly.

## Why This Exists

The current release and install model is binary-centric:

- the release workflow publishes only per-target binary tarballs
- `install.sh` installs only the `entropyfa` binary
- the OpenClaw gateway image layers only the CLI binary into the container image

That is too rigid for the new reference-pack architecture.

It fails in two ways:

1. it does not provide a standard way to install or upgrade markdown reference packs
2. it assumes user-home install paths that do not fit the OpenClaw container model, where `/home/entropy` is tmpfs-backed and workspace access is intentionally constrained

## Core Decisions

These decisions are locked for the packaging model:

- the `entropyfa` binary must remain usable without installed reference packs when the caller supplies explicit assumptions
- reference packs are an optional but first-class install artifact
- agents should read reference packs directly from the filesystem, not through a CLI lookup indirection layer
- install location for reference packs must be configurable
- the CLI must resolve reference-pack roots through explicit precedence rules
- OpenClaw/container installs should not use `~/.entropyfa/reference` as the canonical container path

## Goals

The installation model should:

- support binary-only installs
- support full installs with bundled reference packs
- support platform/container installs with caller-chosen paths
- support monthly reference-pack updates
- provide one stable discovery contract so agents and platforms can determine what is installed
- avoid forcing all users into Entropy's platform-specific layout

## Non-Goals

This design does not:

- preserve the current embedded lookup-first product model
- require every install to carry local reference packs
- make the CLI responsible for browsing reference content
- require one universal install path across all environments

## Current Constraints

The current repo behaves like this:

- `install.sh` installs only the binary into `~/.entropyfa/bin` by default
- the release workflow publishes only binary tarballs
- the OpenClaw gateway image installs `entropyfa` into `/usr/local/bin/entropyfa`

The current Entropy platform also constrains runtime layout:

- VMs pull the gateway image from Artifact Registry and recreate advisor containers from that image
- the gateway container mounts `/home/entropy` as tmpfs
- OpenClaw agents are restricted to workspace-local filesystem access
- advisor workspaces can already receive host bind mounts under the workspace tree

Those constraints mean:

- the binary can live in the image
- reference packs should also live in the image for versioned delivery
- but agents need a workspace-visible copy or bind mount to read them directly

## Recommended Packaging Model

The product should be split into three installable concerns:

1. compute binary
2. reference-pack bundle
3. install profile

### 1. Compute Binary

The `entropyfa` executable remains the deterministic calculator surface.

It must always work in binary-only mode as long as the caller supplies the assumptions required by the chosen calculator.

That keeps the project flexible for:

- cargo installs
- Homebrew/Nix/apt packaging
- third-party containers
- internal platforms that want to vendor their own reference data

### 2. Reference-Pack Bundle

Reference packs should ship as a separate versioned artifact.

The bundle should contain the markdown packs under a stable root, for example:

```text
reference/
  tax/
    2026/
      federal_income_tax_brackets.md
      federal_standard_deductions.md
  retirement/
    2026/
      uniform_lifetime_table.md
  manifest.json
```

The manifest should describe:

- bundle version
- generated timestamp
- included categories and years
- pack count

### 3. Install Profile

The installer should support distinct profiles rather than one hardcoded layout.

Recommended profiles:

- `binary-only`
- `full`
- `platform`

Profile intent:

- `binary-only`: install just the CLI
- `full`: install CLI plus reference packs into user-local defaults
- `platform`: install CLI and packs into caller-specified paths suitable for images or managed environments

## Path Resolution Rules

The CLI should resolve the reference root in this order:

1. `--reference-root`
2. `ENTROPYFA_REFERENCE_ROOT`
3. install-profile default

Recommended defaults:

- local full install: `~/.entropyfa/reference`
- platform/container install: `/opt/entropyfa/reference`
- binary-only install: no implicit pack root required

The binary should not fail simply because the default path does not exist. Missing packs should only matter when a workflow expects them and has not supplied explicit assumptions.

## Release Artifacts

The release workflow should publish three artifact families:

- `entropyfa-<target>.tar.gz`
- `entropyfa-reference-packs-<version>.tar.gz`
- `entropyfa-full-<target>.tar.gz`

Semantics:

- binary artifact: CLI only
- reference-pack artifact: markdown packs only
- full artifact: CLI plus reference packs laid out for the default full install

This keeps the OSS story flexible:

- package managers can ship only the binary
- some users can install packs separately
- Entropy can consume the full bundle for internal environments if useful

## Installer Contract

`install.sh` should evolve from a binary-only installer into a profile-aware installer.

Recommended flags:

- `--profile binary-only`
- `--profile full`
- `--profile platform`
- `--install-dir PATH`
- `--reference-dir PATH`

Rules:

- `binary-only` installs only the executable
- `full` installs the executable and the reference-pack bundle using user-local defaults unless overridden
- `platform` requires or strongly prefers explicit paths and should not modify shell profiles

The existing default quick install should map to `full`, not `binary-only`.

## Discovery Contract

Add a small machine-readable environment command:

```sh
entropyfa env --json
```

It should report:

- CLI version
- binary path
- resolved reference root
- whether reference packs are present
- manifest metadata if present
- active install profile if known

This gives agents and platforms one stable way to discover the local installation without scraping the filesystem layout heuristically.

## OpenClaw / Platform Integration

Entropy's platform should use the `platform` install profile.

### Image-owned assets

The gateway image should contain:

- the binary at a stable path such as `/usr/local/bin/entropyfa` or `/opt/entropyfa/bin/entropyfa`
- the canonical reference-pack bundle at `/opt/entropyfa/reference`

### Workspace visibility

Because OpenClaw agent FS access is workspace-scoped, agents should not be expected to read `/opt/entropyfa/reference` directly.

Instead, the platform should make the same canonical packs visible inside workspaces through one of these mechanisms:

- bind mount them into workspace-visible content
- copy them into a workspace-local reference area during workspace sync

The preferred near-term platform path is:

- host-side canonical copy under `/data/firm/content/firm/entropyfa/reference`
- bind into advisor and household workspaces via the existing firm-content path

That gives agents plain file access while keeping image ownership and rollout versioning clean.

### Update model

For Entropy's platform, monthly updates should ship by:

1. publishing a new `entropyfa` release
2. rebuilding the gateway image with the new binary and/or packs
3. pushing the image to Artifact Registry
4. rolling the new image to VMs so advisor containers are recreated from the updated image

This is better than mutating running containers or relying on a home-directory installer inside gateway containers.

## Agent Workflow

The target agent flow becomes:

1. run `entropyfa compute <calculator> --schema`
2. inspect `recommended_reference_packs`
3. read the packs directly from the resolved reference root or workspace-visible copies
4. gather client facts
5. decide whether to use defaults or explicit overrides
6. run the calculator

That keeps:

- reference content as files
- calculator contracts in the CLI
- deterministic math in code

## Migration Plan

The migration should be incremental.

### Phase 1

- add the reference-pack artifact layout
- add profile-aware install logic
- add `entropyfa env --json`
- publish release artifacts for binary-only and reference packs

### Phase 2

- update the OpenClaw docs and skill guidance to prefer direct file access to packs
- update calculator schemas to advertise `recommended_reference_packs`
- stop positioning CLI lookup commands as the primary reference-data surface

### Phase 3

- adapt the Entropy platform gateway image build to bundle reference packs
- make packs workspace-visible inside OpenClaw
- use image rollout for monthly updates

## Success Criteria

This design is successful if:

- a local OSS user can install only the binary and still run calculators with explicit assumptions
- a local OSS user can install packs into `~/.entropyfa/reference` and read them directly
- a platform/container user can install packs into `/opt/entropyfa/reference` or another chosen path without patching the CLI
- agents can discover the resolved install state through `entropyfa env --json`
- Entropy's OpenClaw platform can ship fresh monthly reference packs through normal image rollout

