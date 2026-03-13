# Contributing to entropyfa

Thank you for considering contributing to entropyfa.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/your-username/entropyfa-cli.git`
3. Create a branch: `git checkout -b my-feature`
4. Make your changes
5. Run checks: `cargo fmt --all -- --check && cargo clippy --workspace -- -D warnings && cargo test --workspace`
6. Commit and push
7. Open a pull request

## Development Setup

You need Rust stable (1.75+). Install via [rustup](https://rustup.rs/).

```sh
cargo build --workspace
cargo test --workspace
```

## Code Style

- Run `cargo fmt` before committing
- All clippy warnings must be resolved (`-D warnings`)
- All tests must pass

## Adding Reference Data

Reference data lives in `engine/src/data/`. Each domain has its own module with embedded constants sourced from IRS publications. When adding or updating data:

1. Cite the IRS publication or authoritative source
2. Include the tax year the data applies to
3. Add tests that verify key values against the source document

For the current maintenance workflow, including the Claude Code / Codex prompt-pack flow, see [`docs/data-pipeline.md`](docs/data-pipeline.md).

## Adding Compute Commands

Compute logic lives in `engine/src/compute/`. CLI wiring lives in `cli/src/commands/`. Every compute command must:

1. Accept input as JSON via `--json '<JSON>'`
2. Produce machine-readable JSON envelopes on stdout
3. Send any human-oriented dashboards or warnings to stderr
4. Support `--schema` to emit input schema for agent discovery

## Reporting Issues

Open an issue on GitHub with:
- What you expected
- What happened
- Steps to reproduce
- `entropyfa --version` output

## License

By contributing, you agree that your contributions will be licensed under the MIT OR Apache-2.0 license.
