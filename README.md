# cliflow

`cliflow` stores curated command recipes for common developer tools so you do not have to remember exact syntax.

Recipes are TOML files under `recipes/`. They can be listed, searched, shown, or resolved and run.

## Run

```bash
cargo run
```

Running `cliflow` without a subcommand opens the interactive search UI.

## Commands

```bash
cliflow tools
cliflow ui
cliflow list [namespace]
cliflow search <query>
cliflow show <namespace>/<id>
cliflow run <namespace>/<id>
cliflow run <namespace>/<id> --dry-run
cliflow run <namespace>/<id> --yes
cliflow run <namespace>/<id> --set name=value
```

High-danger recipes always ask for confirmation before execution, even with `--yes`.

## Development

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo check --all-features
```

## Todo

- [x] Decide the first supported commands and workflows
- [x] Add CLI argument parsing
- [x] Add a simple recipe data model
- [x] Load recipes from embedded files
- [x] Load local recipe overrides from `~/.config/cliflow/recipes/`
- [x] Implement `tools`
- [x] Implement `list`
- [x] Implement `show`
- [x] Implement `search`
- [x] Implement `run --dry-run`
- [x] Implement guarded command execution
- [x] Add Git recipes
- [x] Add Docker recipes
- [x] Add AWS recipes
- [x] Add recipe validation tests
- [ ] Add GitHub CLI recipes
- [ ] Add shell completions
- [ ] Add more namespaces

## Status

Early development.
