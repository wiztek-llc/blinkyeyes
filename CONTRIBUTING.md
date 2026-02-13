# Contributing to Blinky

Thanks for your interest in contributing to Blinky! Here's how you can help.

## Reporting Bugs

Use the [bug report template](https://github.com/tekwiz/blinky/issues/new?template=bug_report.yml) to file a bug. Include steps to reproduce, expected vs. actual behavior, and your OS/version.

## Suggesting Features

Use the [feature request template](https://github.com/tekwiz/blinky/issues/new?template=feature_request.yml). Describe the problem you're trying to solve and your proposed solution.

## Submitting Pull Requests

1. Fork the repo and create a branch from `main`
2. Make your changes
3. Run the checks:
   ```bash
   cargo test
   cargo clippy
   npx tsc --noEmit
   ```
4. Submit a pull request

## Code Style

- Follow existing patterns in the codebase
- No major refactors without prior discussion in an issue
- Keep commits focused and well-described

## Versioning

Releases follow [semantic versioning](https://semver.org/). Update `CHANGELOG.md` when adding user-facing changes.

When cutting a release, move items from the `[Unreleased]` section to a new `[x.y.z] - YYYY-MM-DD` heading.

## Code of Conduct

Be kind, be constructive, be respectful. We're all here to make a useful tool better.
