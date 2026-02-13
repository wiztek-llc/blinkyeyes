# Block 1: CI Pipeline (Lint, Test, Type-Check)

## Files Created
- `.github/workflows/ci.yml` — CI workflow with lint, test, and type-check steps

## Files Modified
- None

## Deviations from Spec
- **`working-directory: src-tauri`** used for all Cargo commands instead of running from root. The Cargo.toml and Cargo.lock live in `src-tauri/`, not the repo root, so all `cargo` commands must run from that directory. This is functionally equivalent to `--manifest-path src-tauri/Cargo.toml` but cleaner.
- **Node.js caching** uses `setup-node`'s built-in `cache: npm` instead of a separate `actions/cache` step for `node_modules`. This is the recommended approach as of `actions/setup-node@v4` — it caches the npm global cache and `npm ci` restores from it. It's simpler and achieves the same result.

## Acceptance Criteria Results
- [x] `.github/workflows/ci.yml` is valid YAML (verified with `python3 yaml.safe_load`)
- [x] Workflow triggers on PR to main and push to main
- [x] All check steps are present: tsc, fmt, clippy, test, check
- [x] Caching is configured for both Cargo (`actions/cache` keyed on `Cargo.lock`) and npm (`setup-node` cache)
- [x] Linux system dependencies for Tauri v2 are installed (`libwebkit2gtk-4.1-dev`, `libappindicator3-dev`, `librsvg2-dev`, `patchelf`, `libxss-dev`)
- [x] The workflow name ("CI") matches what the README badge references (`actions/workflows/ci.yml/badge.svg`)

## Known Issues / TODOs for Later Blocks
- The CI workflow only runs on `ubuntu-latest`. Cross-platform CI (macOS, Windows) could be added as a future enhancement, but the spec only calls for a single `check` job on Ubuntu.
- Block 2 (Release Pipeline) builds on this workflow — it should reference the same system dependencies and caching strategy.
