# Reference Root

This directory is the canonical repo-owned root for installable reference packs.

Reference packs are plain Markdown artifacts organized under this root.

`manifest.json` is the machine-readable index that installation and discovery tooling uses to detect pack availability and bundle metadata.

## Retirement Packs

The retirement reference packs live under `reference/retirement/2026/`:

- `distribution_rules.md`
- `uniform_lifetime_table.md`
- `single_life_table.md`
- `joint_life_table.md`

These packs are the canonical source for agent-readable 2026 RMD rules, with each file pairing a rigid machine block with short primer text and review metadata.
