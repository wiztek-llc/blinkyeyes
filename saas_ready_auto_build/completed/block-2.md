# Block 2: Release Pipeline (Cross-Platform Build + GitHub Releases)

## Files Created
- `.github/workflows/release.yml` — Cross-platform release workflow triggered on `v*` tags, builds macOS (universal), Windows, and Linux binaries via `tauri-apps/tauri-action@v0`

## Files Modified
- `CONTRIBUTING.md` — Added guidance on moving `[Unreleased]` changelog entries to a versioned heading when cutting a release

## Deviations from Spec
- None. All items implemented exactly as specified.

## Acceptance Criteria Results
- [x] `.github/workflows/release.yml` is valid YAML (verified with `python3 yaml.safe_load`)
- [x] Workflow triggers only on `v*` tags (`on.push.tags: ['v*']`)
- [x] Build matrix covers macOS (`macos-latest`), Windows (`windows-latest`), and Linux (`ubuntu-22.04`)
- [x] Tauri action is configured with correct parameters (`tagName`, `releaseName`, `releaseBody`, `releaseDraft`, `prerelease`, `args --target`)
- [x] Release notes reference the changelog (links to `CHANGELOG.md` on GitHub)
- [x] Code signing secrets are documented (top-level comment block and inline env comments for macOS and Windows)
- [x] The release process is documented in the workflow file (6-step comment block at top)
- [x] Version comes from the git tag (not hardcoded) — uses `v__VERSION__` pattern which the Tauri action resolves from `tauri.conf.json`

## Known Issues / TODOs for Later Blocks
- macOS builds are unsigned — users will see a Gatekeeper warning. Code signing requires an Apple Developer certificate. Secret names are documented and ready to uncomment.
- Windows builds are unsigned — Authenticode signing requires a code signing certificate. Secret names are documented.
- The `GITHUB_TOKEN` used by the Tauri action is the automatic token provided by GitHub Actions — no manual secret setup needed for creating releases.
- `fail-fast: false` is set so all platform builds continue even if one fails, ensuring maximum artifact availability per release.
