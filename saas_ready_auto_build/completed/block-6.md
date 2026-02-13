# Block 6: Website Deployment Pipeline

## Files Created
- `.github/workflows/deploy-site.yml` — GitHub Actions workflow to auto-deploy the website to the px server via rsync over SSH

## Files Modified
- None

## Deviations from Spec
- None. All items implemented exactly as specified.

## Acceptance Criteria Results
- [x] `.github/workflows/deploy-site.yml` is valid YAML (verified with Python yaml.safe_load)
- [x] Workflow triggers on push to main when website files change (`paths: ['website/**']`)
- [x] Workflow can also be triggered manually via workflow_dispatch
- [x] SSH setup step uses secrets (not hardcoded values) — `DEPLOY_SSH_KEY`, `DEPLOY_HOST`
- [x] rsync command syncs the `website/` directory to the server (`rsync -avz --delete website/ user@host:/var/www/blinkyeyes.com/`)
- [x] Required secrets are documented in the workflow file (DEPLOY_SSH_KEY, DEPLOY_HOST, DEPLOY_USER)
- [x] Deploy key setup instructions are documented (5-step guide in comment header)

## Known Issues / TODOs for Later Blocks
- The three GitHub Secrets (DEPLOY_SSH_KEY, DEPLOY_HOST, DEPLOY_USER) must be configured in the GitHub repository settings before the workflow will succeed.
- The deploy user on the server must have write access to `/var/www/blinkyeyes.com/` (covered by Block 5's setup.sh).
- rsync is pre-installed on Ubuntu GitHub runners, so no explicit install step is needed.
