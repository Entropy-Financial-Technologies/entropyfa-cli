# OpenClaw

`entropyfa` ships an official OpenClaw skill for financial planning workflows:

- package slug: `entropyfa`
- display name: `entropyFA Financial Planning`
- description: `Verified financial planning data and blazing-fast, deterministic calculators for Monte Carlo projection, goal solving, Roth conversions, RMDs, income tax, estate tax, and pension analysis.`

The skill teaches OpenClaw when to use verified embedded reference data, when to ask the CLI for schema, and when to run deterministic calculations locally.

## What OpenClaw Gets

With `entropyFA Financial Planning`, OpenClaw can:

- inspect required inputs with `compute <command> --schema`
- discover the installed binary path and resolved reference root with `entropyfa env --json`
- read reviewed markdown reference packs directly from the filesystem
- run deterministic tax, retirement, estate, Roth conversion, pension, and projection commands with JSON output
- keep `compute projection` machine-readable by default and only use `--visual` when a terminal dashboard is explicitly wanted

## Prerequisites

Install the local `entropyfa` CLI:

```sh
curl -fsSL https://get.entropyfa.com | sh
```

That default install behaves like `--profile full`:

- binary at `~/.entropyfa/bin/entropyfa`
- reference packs at `~/.entropyfa/reference/...`

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
3. install-profile default

That means local installs usually resolve to `~/.entropyfa/reference`, while platform/container-style installs usually resolve to `/opt/entropyfa/reference`.

## Recommended Workflow

Use the skill like this:

1. Run `entropyfa env --json` to discover the active binary path and resolved reference root.
2. Read the relevant markdown packs directly from the resolved reference root when you need yearly thresholds, rules, assumptions, or reviewed context.
3. Run `entropyfa compute <command> --schema` when required inputs are missing or you need the contract for a calculator.
4. Run `entropyfa compute <command> --json '<JSON>'` once the inputs and assumptions are known.
5. Use `entropyfa compute projection --visual --json ...` only when the user explicitly wants the terminal dashboard.

`data lookup` and `data coverage` still exist, but they are no longer the preferred discovery path for OpenClaw workflows that need broader yearly reference context. The markdown packs on disk are the primary human/agent reference surface.

## Local Vs Container Assumptions

Do not assume the same filesystem layout across environments:

- user-local OSS installs commonly use `~/.entropyfa/bin` and `~/.entropyfa/reference`
- platform/container installs commonly use `/usr/local/bin` and `/opt/entropyfa/reference`
- binary-only installs may have no local reference packs at all

Calculators can still run without local packs when the request includes explicit assumptions. The packs matter when OpenClaw needs reviewed markdown context on disk.

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

Read files directly from the resolved reference root:

```sh
ls ~/.entropyfa/reference
```

Run projection without visuals:

```sh
entropyfa compute projection --json '{"starting_balance":1000000,"time_horizon_months":360,"return_assumption":{"annual_mean":0.07,"annual_std_dev":0.15},"cash_flows":[{"amount":-4000,"frequency":"monthly"}]}'
```

Run projection with visuals:

```sh
entropyfa compute projection --visual --json '{"starting_balance":1000000,"time_horizon_months":360,"return_assumption":{"annual_mean":0.07,"annual_std_dev":0.15},"cash_flows":[{"amount":-4000,"frequency":"monthly"}]}'
```
