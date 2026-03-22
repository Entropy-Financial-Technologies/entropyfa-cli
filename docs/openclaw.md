# OpenClaw

`entropyfa` ships an official OpenClaw skill for financial planning workflows:

- package slug: `entropyfa`
- display name: `entropyFA Financial Planning`
- description: `Verified financial planning data and blazing-fast, deterministic calculators for Monte Carlo projection, goal solving, Roth conversions, RMDs, income tax, estate tax, and pension analysis.`

The recommended underlying CLI workflow is to discover the active install with `entropyfa env --json`, ask for calculator contracts with `--schema`, and read any installed reference files directly from the resolved filesystem root when that path exists. The packaged skill in `integrations/openclaw/entropyfa/SKILL.md` still begins with legacy `data coverage` / `data lookup` guidance, so stock installs should be treated as backward-compatible rather than rewritten around the newer flow.

## What OpenClaw Gets

With `entropyFA Financial Planning`, OpenClaw can:

- inspect required inputs with `compute <command> --schema`
- discover the installed binary path and resolved reference root with `entropyfa env --json`
- read installed markdown reference files directly from the resolved filesystem root when packs are present
- run deterministic tax, retirement, estate, Roth conversion, pension, and projection commands with JSON output
- keep `compute projection` machine-readable by default and only use `--visual` when a terminal dashboard is explicitly wanted

## Prerequisites

Install the local `entropyfa` CLI:

```sh
curl -fsSL https://get.entropyfa.com | sh
```

That default install behaves like `--profile full`:

- binary at `~/.entropyfa/bin/entropyfa`
- reference root at `~/.entropyfa/reference/...` plus any reviewed packs included in the current release

Alternative install shapes:

```sh
# Binary only
curl -fsSL https://get.entropyfa.com | sh -s -- --profile binary-only

# Shared image / container / platform-style layout
curl -fsSL https://get.entropyfa.com | sh -s -- --profile platform \
  --install-dir /usr/local/bin \
  --reference-dir /opt/entropyfa/reference
```

`binary-only` is valid for standalone OSS compute usage. `platform` is the better fit for shared images or containers because it skips shell-profile edits and uses explicit filesystem paths.

Platform/container layouts should place the reference tree at `/opt/entropyfa/reference`. Explicit `--reference-root` or `ENTROPYFA_REFERENCE_ROOT=/opt/entropyfa/reference` are the usual way to point the CLI at a non-default tree, and `ENTROPYFA_INSTALL_PROFILE=platform`, a binary installed under `/opt/entropyfa/...`, a managed `/usr/local/bin/entropyfa` install paired with `/opt/entropyfa/reference/.entropyfa-managed`, or installer-written `entropyfa.install.json` metadata for custom layouts can all trigger discovery of that layout. A binary-only install may legitimately resolve a reference root that is not present on disk.

Install OpenClaw:

```sh
curl -fsSL --proto '=https' --tlsv1.2 https://openclaw.ai/install.sh | bash
```

Install ClawHub:

```sh
npm i -g clawhub
```

## Install The Official Skill

Install into the current OpenClaw workspace:

```sh
clawhub install entropyfa
```

ClawHub installs skills into `./skills` under the current working directory by default, and OpenClaw loads them from the workspace on the next session.

## Local Workspace Install

If you want to install directly from this repo instead of ClawHub, copy the skill into your OpenClaw workspace:

```sh
mkdir -p skills
cp -R integrations/openclaw/entropyfa ./skills/entropyfa
```

OpenClaw also loads shared skills from `~/.openclaw/skills` if you prefer a machine-wide install.

## Reference Root Discovery

Before reading packs or deciding which install layout you are in, ask the CLI where it resolved the reference root:

```sh
entropyfa env --json
```

The CLI resolves the reference root in this order:

1. explicit `--reference-root`
2. `ENTROPYFA_REFERENCE_ROOT`
3. installer-written `entropyfa.install.json` metadata beside the active binary
4. runtime platform hint such as `ENTROPYFA_INSTALL_PROFILE=platform`, or platform auto-detection for binaries under `/opt/entropyfa/...` and managed `/usr/local/bin/entropyfa` installs
5. default local root at `~/.entropyfa/reference`

That means local installs usually resolve to `~/.entropyfa/reference`, while platform/container-style installs can also auto-detect `/opt/entropyfa/reference` when the binary lives under `/opt/entropyfa/...` or when a managed `/usr/local/bin/entropyfa` install is paired with `/opt/entropyfa/reference/.entropyfa-managed`. Custom `full` and `platform` installs created by the official installer also rediscover their reference root through the binary-side metadata file. Manual layouts outside those defaults still need an explicit hint such as `ENTROPYFA_REFERENCE_ROOT=/path/to/reference`, `ENTROPYFA_INSTALL_PROFILE=platform`, or `--reference-root`.

## Recommended Workflow

Use the skill like this:

1. Run `entropyfa env --json` to discover the active binary path and resolved reference root.
2. Read the relevant markdown files directly from the resolved reference root when they are installed and you need yearly thresholds, rules, assumptions, or reviewed context.
3. Run `entropyfa compute <command> --schema` when required inputs are missing or you need the contract for a calculator.
4. Run `entropyfa compute <command> --json '<JSON>'` once the inputs and assumptions are known.
5. Use `entropyfa compute projection --visual --json ...` only when the user explicitly wants the terminal dashboard.

The recommended CLI workflow is `env --json` + `--schema` + direct filesystem reads. The packaged OpenClaw skill still starts from `data coverage` / `data lookup`, so treat those commands as legacy guidance in the stock skill and as a customization point if you want to adopt the newer flow inside OpenClaw itself.

## Local Vs Container Assumptions

Do not assume the same filesystem layout across environments:

- user-local OSS installs commonly use `~/.entropyfa/bin` and `~/.entropyfa/reference`
- platform/container installs commonly use `/usr/local/bin` and `/opt/entropyfa/reference`
- binary-only installs may have no local reference packs at all

Calculators can still run without local packs when the request includes explicit assumptions. The packs matter when OpenClaw needs reviewed markdown context on disk for agent inspection.

Follow-on work in `entropy-platform` will need to make the platform-installed reference root visible inside the agent workspace. That workspace binding is not implemented in this repo yet, so keep docs and prompts honest about the difference between generic OSS installs and platform image behavior.

## Trust And Source Review

This is the official entropyfa-published OpenClaw skill.

- install from the official slug: `entropyfa`
- review the source in [integrations/openclaw/entropyfa](../integrations/openclaw/entropyfa)
- prefer the GitHub repo as the source of truth for the skill contents

The underlying `entropyfa` CLI is local by default, returns JSON on `stdout`, and exposes the resolved reference-root metadata through `entropyfa env --json`.

## Example Commands

Discover the active binary and reference root:

```sh
entropyfa env --json
```

Inspect a compute schema:

```sh
entropyfa compute roth-conversion --schema
```

Run a deterministic compute command:

```sh
entropyfa compute federal-tax --json '{"filing_status":"single","income":{"wages":150000}}'
```

This example uses `jq`:

```sh
REFERENCE_ROOT=$(entropyfa env --json | jq -r '.data.reference.resolved_root')
if [ -d "$REFERENCE_ROOT" ]; then
  ls "$REFERENCE_ROOT"
else
  echo "No local reference root is present at: $REFERENCE_ROOT"
fi
```

Run projection without visuals:

```sh
entropyfa compute projection --json '{"starting_balance":1000000,"time_horizon_months":360,"return_assumption":{"annual_mean":0.07,"annual_std_dev":0.15},"cash_flows":[{"amount":-4000,"frequency":"monthly"}]}'
```

Run projection with visuals:

```sh
entropyfa compute projection --visual --json '{"starting_balance":1000000,"time_horizon_months":360,"return_assumption":{"annual_mean":0.07,"annual_std_dev":0.15},"cash_flows":[{"amount":-4000,"frequency":"monthly"}]}'
```
