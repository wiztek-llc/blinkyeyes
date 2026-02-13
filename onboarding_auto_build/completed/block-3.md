# Block 3: Setup Step (Step 2)

## Files modified
- `src/components/onboarding/SetupStep.tsx` — Full implementation replacing placeholder

## Files created
- None

## Deviations from spec
- **No deviations.** Implemented all three setting groups (work interval, notifications, theme) with the specified presets, toggles, and live theme preview.

## Acceptance criteria results
- [x] Work interval presets work, with "20 min" highlighted as recommended
- [x] Custom interval input appears when "Custom" is selected
- [x] All three notification toggles work independently
- [x] Warning appears if all notifications are disabled
- [x] Theme selector applies the chosen theme live
- [x] Settings choices persist when navigating back and forth between steps
- [x] The layout fits in the viewport without scrolling
- [x] Next and Back buttons work correctly
- [x] Dark mode renders correctly

## Verification
- `cargo check` — passes (no changes to Rust code)
- `cargo test` — 32 tests pass (no regressions)
- `npx tsc --noEmit` — passes clean

## Known issues / TODOs for later blocks
- Theme live preview applies immediately but does not revert if user navigates back and changes it (spec notes this is acceptable behavior — settings are accumulated and saved on completion in Block 4's ReadyStep).
- Custom interval input allows values 1–120 matching the backend validation range.
