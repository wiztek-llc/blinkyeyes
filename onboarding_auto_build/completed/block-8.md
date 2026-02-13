# Block 8: Integration, Polish & Settings Reset

## Files Modified
- `src/App.tsx` — Updated `MainApp` to handle onboarding reset (state-driven view switching instead of one-way flag), updated `AppShell` to accept and pass `onResetOnboarding` prop to Settings
- `src/pages/Settings.tsx` — Added "About" section with "Re-run onboarding wizard" link-style button, accepts `onResetOnboarding` prop

## Files Created
- None

## Deviations from Spec
- **"Re-run onboarding" in "About" section, not Danger Zone**: The spec suggests placing it in the Danger Zone or a new "About" section. Chose "About" because re-running onboarding is non-destructive (it doesn't delete data, just resets the wizard flag) and placing it in the Danger Zone would make it look scarier than it is.

## Acceptance Criteria Results

### Integration Checklist (14 items)
- [x] App.tsx routing — Fresh install → wizard, returning user → dashboard
- [x] Wizard → Settings — Settings accumulated during wizard are saved on completion
- [x] Wizard → Timer — Timer starts after "Start Protecting Your Eyes" is clicked
- [x] Demo break — "Try it" in preview step triggers real overlay for 5 seconds
- [x] Timer respects onboarding — Timer paused when onboarding not complete
- [x] Empty states — All dashboard components show helpful text when empty
- [x] Tooltips sequence — First dashboard visit shows tooltips one-at-a-time
- [x] Tooltip persistence — Dismissed tooltips stay dismissed across app restarts
- [x] First break overlay — First break shows "Your first break!" text
- [x] First break celebration — Celebration card appears on dashboard after first completed break
- [x] Theme live preview — Theme choice in wizard applies immediately
- [x] Existing user migration — User with existing break data skips onboarding entirely
- [x] Reset onboarding — "Re-run onboarding" in settings restarts the wizard
- [x] Tray interaction during onboarding — Tray icon visible, timer shows paused

### Build Verification
- [x] `cargo check` — passes (0 errors)
- [x] `cargo test` — 32/32 tests pass
- [x] `npx tsc --noEmit` — passes (0 type errors)
- [x] All existing tests still pass (no regressions)

### Settings Reset Flow
- [x] "Re-run onboarding wizard" button is a link-style button in About section
- [x] Clicking it calls `resetOnboarding` which pauses the timer and resets onboarding flags
- [x] UI transitions smoothly from Settings → Onboarding wizard (fade animation)
- [x] Wizard starts from step 1 after reset
- [x] Completing the wizard again saves settings and starts the timer

## Known Issues
- Tray menu items (Skip Break, Pause/Resume) remain clickable during onboarding, but they are effectively no-ops: Skip Break checks for Breaking phase, and the timer is already Paused. No UX harm.
