# Contributing

Thanks for your interest in contributing to `discogs-rs`.

## Development Setup

1. Install Rust (stable) via `rustup`.
2. Clone the repository.
3. Run validation commands:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo check --examples
```

## Workflow

1. Create a feature branch from `master`.
2. Keep changes scoped and reviewable.
3. Update docs/tests when behavior changes.
4. Open a pull request with:
   - clear summary
   - motivation
   - testing evidence

## Commit Messages

Use short, descriptive commit messages.

Examples:
- `feat: add user-token convenience constructors`
- `fix: enforce auth level before request dispatch`
- `docs: clarify marketplace examples`

## Pull Request Checklist

- [ ] Formatting passes
- [ ] Clippy passes with `-D warnings`
- [ ] Tests pass
- [ ] Public API/docs updated if needed
- [ ] Changelog updated for user-facing changes

## Reporting Issues

When filing issues, include:
- expected behavior
- actual behavior
- reproduction steps
- Rust version (`rustc --version`)
- OS and architecture
