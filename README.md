# cliflow

`cliflow` is a small Rust CLI project for helping developers find command-line workflows.

## Run

```bash
cargo run
```

## Development

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo check --all-features
```

## Todo

- [ ] Decide the first supported commands and workflows
- [ ] Add CLI argument parsing
- [ ] Add a simple workflow data model
- [ ] Load workflows from files
- [ ] Implement `list`
- [ ] Implement `show`
- [ ] Implement `search`
- [ ] Add Git workflows
- [ ] Add GitHub CLI workflows

## Status

Early development.
