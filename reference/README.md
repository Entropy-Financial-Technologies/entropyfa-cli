# Reference Root

This directory is the canonical repo-owned root for installable reference packs.

Reference packs are plain Markdown artifacts organized under this root.

`manifest.json` is the machine-readable index that installation and discovery tooling uses to detect pack availability and bundle metadata.

## Retirement Packs

The retirement reference packs live under `reference/retirement/2026/`:

- `distribution_rules.md`
- `uniform_lifetime_table.md`
- `single_life_expectancy_table.md`
- `joint_life_table.md`

These packs are the canonical source for agent-readable 2026 RMD rules, with each file pairing a rigid machine block with short primer text and review metadata.

## Efficient Lookup

Large reference files (especially `joint_life_table.md` at 28K+ lines) should
**not** be read in full. Instead:

- **Grep for specific values:** e.g. grep for `"owner_age": 75` with 2–3
  context lines, then filter for the target `spouse_age`. This returns ~20
  lines instead of 28,000.
- **Read with offset/limit** if you know the approximate line range.
- The smaller files (`uniform_lifetime_table.md`, `single_life_expectancy_table.md`,
  `distribution_rules.md`) are under 600 lines and safe to read in full.

`single_life_expectancy_table.md` packages the reviewed artifact keyed `single_life_table` so the filename stays descriptive while the canonical reviewed artifact contract remains stable.
