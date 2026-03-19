# Compute Visuals

This document covers the human-facing terminal visuals that `entropyfa` can render alongside its machine-readable JSON output.

## Current Visual Output

Today the main built-in visual is the Monte Carlo projection dashboard. It is opt-in:

```sh
entropyfa compute projection --visual --json '{"starting_balance":1000000,"time_horizon_months":360,"return_assumption":{"annual_mean":0.07,"annual_std_dev":0.15},"cash_flows":[{"amount":-4000,"frequency":"monthly"}]}'
```

The CLI behavior is intentional:

- JSON result payload stays on `stdout`
- the terminal dashboard renders on `stderr`
- the dashboard renders only when you ask for it with `--visual`
- the dashboard only renders when `stderr` is attached to a real terminal
- the dashboard is skipped automatically in non-interactive environments, narrow terminals, or when there is not enough data to plot

That means agents, scripts, and pipes can keep reading clean JSON from `stdout` without needing to strip chart output.

## Bucketed JSON Output

Bucketed projection requests return extra bucket-aware fields in the JSON response on `stdout`, including bucket terminal percentiles, bucket depletion counts, and deterministic ending balances by bucket.

The terminal dashboard does not draw those bucket-level charts yet. It still summarizes the aggregate projection only, which keeps the visual readable while the bucket-aware JSON remains available for downstream tooling.

## What The Projection Dashboard Shows

The projection dashboard is a human summary of the Monte Carlo result. It uses the computed percentile time series and, when available, overlays the linear path as a reference.

At a high level it shows:

- ending-value summary
- percentile bands over time
- median path
- linear reference path when projection mode includes it
- time horizon and runtime context

When `--visual` is enabled, the CLI automatically requests the percentile bands needed for the dashboard if they are not already present in the input. Without `--visual`, that extra dashboard-specific percentile work is skipped.

## Example

The README includes a screenshot of the current projection visual:

![Monte Carlo projection chart](../assets/monte-carlo-chart.png)

## Important Behavior

- `compute projection` does not render a dashboard unless you opt in with `--visual`
- the visual is an operator convenience, not a second output format
- the JSON payload remains the canonical machine-readable result
- if you need deterministic automation, consume `stdout` and ignore `stderr`

## Relationship To The Data Docs

This visual is separate from the embedded reference-data surface:

- [embedded-data.md](embedded-data.md) documents `data lookup`
- this document covers human-facing compute visuals
- [data-pipeline.md](data-pipeline.md) covers the maintainer workflow that verifies the embedded data
