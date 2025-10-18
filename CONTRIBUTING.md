# Contributing to unitx

Thanks for helping! Quick rules:
- Small, focused PRs are easier to review.
- Add tests for new features/bugfixes.
- `cargo fmt` + `cargo clippy` must pass.

## Dev setup
```bash
cargo build
cargo test
```

## Commit style

Use clear, imperative messages: `feat(core): add C↔F↔K`

Reference issues when relevant: `Fixes #12`.

## Good first issues

- Add units (pressure, data size)
- Improve error messages / docs
- Add OpenAPI example