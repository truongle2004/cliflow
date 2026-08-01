# lazycmds

`lazycmds` is a search-first terminal helper for developer commands.

It stores short executable command recipes and longer multi-step workflows so you can quickly find what to run and understand when to run it.

## Run

```bash
cargo run
```

Running `lazycmds` without a subcommand opens the interactive Ratatui UI.

## Interactive UI

The UI is designed around search first:

- Type to search recipes and workflows.
- Recipes and workflows are shown in separate panes.
- Press `Tab` to switch between panes.
- Press `Up` / `Down` to select an item.
- Press `PgUp` / `PgDn` to scroll long workflow details.
- Press `Esc` or `Ctrl-C` to quit.

Recipes are single commands. Workflows are multi-step examples for real situations, such as moving work from `main` to a feature branch, rebasing a diverged branch, or stashing work during an urgent context switch.

## Content

Embedded recipes live in `recipes/` as TOML files.

Embedded workflows live in `content/<tool>/workflows/` as YAML files.

Local recipe overrides can be added under:

```bash
~/.config/lazycmds/recipes/
```

Workflow content should describe multi-step flows. Single CLI commands belong in `recipes/`, not `content/*/workflows/`.

## Current Content

Recipe namespaces:

- `git`
- `docker`
- `aws`
- `linux`

Git workflows include:

- Git CLI setup defaults
- Moving uncommitted `main` changes to a feature branch
- Moving an accidental `main` commit to a branch
- Updating a branch before pushing
- Full feature branch lifecycle
- Stash/context-switch scenarios
- Release tagging
- Bisect debugging
- Cherry-picking fixes
- Worktree hotfix flow
- Submodules, Git LFS, and team history audit flows

Linux workflows include log investigation, process and port troubleshooting, storage maintenance,
codebase maintenance, resource audits, and file operations.

## Development

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo check --all-features
```

If embedded recipes or workflows appear stale after moving the project folder, rebuild the binary:

```bash
cargo clean
cargo run
```
