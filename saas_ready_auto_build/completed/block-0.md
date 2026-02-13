# Block 0: Repository Setup & GitHub Configuration

## Files Created
- `README.md` — Full README with header/badges, features, installation, building from source, 20-20-20 rule explanation, contributing, and license sections
- `LICENSE` — MIT License, 2026, "Blinky Contributors"
- `CONTRIBUTING.md` — Bug reporting, feature requests, PR process, code style, versioning, code of conduct
- `CHANGELOG.md` — Initial unreleased entry
- `.github/ISSUE_TEMPLATE/bug_report.yml` — YAML-based bug report form with all required fields
- `.github/ISSUE_TEMPLATE/feature_request.yml` — YAML-based feature request form
- `.github/PULL_REQUEST_TEMPLATE.md` — PR template with checklist

## Files Modified
- `.gitignore` — Added `.idea/`, `*.db`, `.env` patterns

## Deviations from Spec
- None. All items implemented exactly as specified.

## Acceptance Criteria Results
- [x] `README.md` exists and renders correctly in a Markdown previewer
- [x] `LICENSE` contains valid MIT license text
- [x] `CONTRIBUTING.md` exists with clear instructions
- [x] `CHANGELOG.md` exists with the initial release entry
- [x] `.github/ISSUE_TEMPLATE/bug_report.yml` is valid YAML (verified with python3 yaml.safe_load)
- [x] `.github/ISSUE_TEMPLATE/feature_request.yml` is valid YAML (verified with python3 yaml.safe_load)
- [x] `.github/PULL_REQUEST_TEMPLATE.md` exists
- [x] `.gitignore` covers all necessary patterns (target/, node_modules, dist/, .DS_Store, *.db, .env, .idea/)
- [x] All files are created and ready to be committed

## Known Issues / TODOs for Later Blocks
- No git remote is configured yet. The GitHub owner/repo used in URLs is `tekwiz/blinky` — update if different.
- `.DS_Store` and `.idea/` files were already staged before this block. They should be removed from tracking (`git rm --cached`) in a cleanup pass.
- README hero screenshot `website/assets/screenshot-dashboard.png` doesn't exist yet — will be created in Block 3 or Block 7.
