# Release Checklist

This checklist defines the standard release workflow for `discogs-rs`, including pre-release checks, versioning, validation, and publishing.

## 1. Pre-release Checks

- [ ] Freeze scope: release goals and change boundaries are finalized.
- [ ] Validate workspace state: only expected changes are included.
- [ ] Ensure documentation is synchronized: `README.md`, `CHANGELOG.md`, and implementation behavior match.
- [ ] Verify package metadata in `Cargo.toml` (`license`, `repository`, `description`) is ready for public release.

Optional commands:

```bash
git status --short
rg -n '^version\s*=\s*"' Cargo.toml
```

## 2. Versioning

- [ ] Confirm next version (for example, `0.1.0 -> 0.1.1`).
- [ ] Update `version` in `Cargo.toml`.
- [ ] Add a matching release entry in `CHANGELOG.md` with date and scope.
- [ ] Re-check README examples against the current public API.

Recommended commands:

```bash
rg -n '^version\s*=\s*"' Cargo.toml
rg -n '^## \[[0-9]+\.[0-9]+\.[0-9]+\]' CHANGELOG.md
```

## 3. Release Validation

- [ ] Formatting checks pass.
- [ ] Static checks pass (including warning policy).
- [ ] Unit and integration tests pass.
- [ ] Dry-run packaging and publishing succeed.

Recommended commands:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo publish --dry-run
```

## 4. Publish Steps

- [ ] Commit release-related changes (at minimum version + changelog).
- [ ] Create and push a version tag.
- [ ] Publish the crate.
- [ ] Create a GitHub Release aligned with changelog notes.

Reference commands:

```bash
git add Cargo.toml CHANGELOG.md README.md RELEASE_CHECKLIST.md
git commit -m "release: vX.Y.Z"
git tag -a vX.Y.Z -m "Release vX.Y.Z"
git push origin <branch-name>
git push origin vX.Y.Z
cargo publish
```

## 5. Post-release Verification

- [ ] New version is visible on crates.io with correct metadata.
- [ ] Downstream projects can resolve and fetch the new version.
- [ ] Key examples compile in a clean environment.

Reference command:

```bash
cargo search discogs-rs --limit 5
```

## 6. Failure and Recovery Strategy

- [ ] If publish fails, keep commit/tag and investigate before retrying.
- [ ] If a critical issue is found after publish, ship a follow-up patch release (do not rewrite published crate history).
- [ ] Document impact and fix release in `CHANGELOG.md`.
