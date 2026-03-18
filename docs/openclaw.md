# OpenClaw

`entropyfa` ships an official OpenClaw skill for financial planning workflows:

- package slug: `entropyfa`
- display name: `entropyFA Financial Planning`
- description: `Verified financial planning data and blazing-fast, deterministic calculators for Monte Carlo projection, goal solving, Roth conversions, RMDs, income tax, estate tax, and pension analysis.`

The skill teaches OpenClaw when to use verified embedded reference data, when to ask the CLI for schema, and when to run deterministic calculations locally.

## What OpenClaw Gets

With `entropyFA Financial Planning`, OpenClaw can:

- discover available planning datasets with `data coverage`
- retrieve verified yearly values plus source URLs with `data lookup`
- inspect required inputs with `compute <command> --schema`
- run deterministic tax, retirement, estate, Roth conversion, pension, and projection commands with JSON output
- keep `compute projection` machine-readable by default and only use `--visual` when a terminal dashboard is explicitly wanted

## Prerequisites

Install the local `entropyfa` CLI:

```sh
curl -fsSL https://get.entropyfa.com | sh
```

That default install is user-local and lands in `~/.entropyfa/bin` without requiring `sudo`.

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

## Recommended Workflow

Use the skill like this:

1. Run `entropyfa data coverage` when you need to discover available datasets.
2. Run `entropyfa data lookup ...` when the user needs official annual values, source URLs, or verification metadata.
3. Run `entropyfa compute <command> --schema` if required inputs are missing.
4. Run `entropyfa compute <command> --json '<JSON>'` once the inputs are known.
5. Use `entropyfa compute projection --visual --json ...` only when the user explicitly wants the terminal dashboard.

## Trust And Source Review

This is the official entropyfa-published OpenClaw skill.

- install from the official slug: `entropyfa`
- review the source in [integrations/openclaw/entropyfa](../integrations/openclaw/entropyfa)
- prefer the GitHub repo as the source of truth for the skill contents

The underlying `entropyfa` CLI is local by default, returns JSON on `stdout`, and returns source URLs in `data lookup` responses by default.

## Example Commands

Discover data:

```sh
entropyfa data coverage
```

Retrieve a source-backed lookup:

```sh
entropyfa data lookup --category tax --key federal_income_tax_brackets --year 2026 --filing-status single
```

Inspect a compute schema:

```sh
entropyfa compute roth-conversion --schema
```

Run a deterministic compute command:

```sh
entropyfa compute federal-tax --json '{"filing_status":"single","income":{"wages":150000}}'
```

Run projection without visuals:

```sh
entropyfa compute projection --json '{"starting_balance":1000000,"time_horizon_months":360,"return_assumption":{"annual_mean":0.07,"annual_std_dev":0.15},"cash_flows":[{"amount":-4000,"frequency":"monthly"}]}'
```

Run projection with visuals:

```sh
entropyfa compute projection --visual --json '{"starting_balance":1000000,"time_horizon_months":360,"return_assumption":{"annual_mean":0.07,"annual_std_dev":0.15},"cash_flows":[{"amount":-4000,"frequency":"monthly"}]}'
```
