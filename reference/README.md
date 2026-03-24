# Reference Root

This directory is the canonical repo-owned root for installable reference packs.

Reference packs are plain Markdown artifacts organized under this root.

`manifest.json` is the machine-readable index that installation and discovery tooling uses to detect pack availability and bundle metadata.

The maintainer pipeline uses `cargo run -p entropyfa-engine --bin data-pipeline -- status` as the migration dashboard. That status view now tracks:

- reviewed JSON artifact coverage
- markdown-pack coverage
- legacy-only entries where reviewed JSON exists but the markdown pack has not been written yet

Running `data-pipeline apply --run <RUN_ID>` now writes or refreshes the markdown pack for that entry and updates `manifest.json`.

## Retirement Packs

The retirement reference packs live under `reference/retirement/2026/`:

- `distribution_rules.md`
- `uniform_lifetime_table.md`
- `single_life_expectancy_table.md`
- `joint_life_table.md`

These packs are the canonical source for agent-readable 2026 RMD rules, with each file pairing a rigid machine block with short primer text and review metadata.

Generated packs now include a required reviewed primer above the machine block. The required sections are:

- `What This Is`
- `Lookup Parameters`
- `Interpretation Notes`
- `Does Not Include`
- `Caveats`

`Typical Uses` is optional.

The maintainer pipeline enforces that:

- the primary agent writes these primer sections
- the verifier independently reviews them for factual accuracy and fit
- `review` and `apply` block if any required section is missing, blank, or disputed

`single_life_expectancy_table.md` packages the reviewed artifact keyed `single_life_table` so the filename stays descriptive while the canonical reviewed artifact contract remains stable.

## Other Categories

As additional entries move through the updated pipeline, new category/year directories under this root are created automatically from the reviewed artifact during `apply`.
