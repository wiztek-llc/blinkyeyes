# Block 0: Onboarding State Infrastructure

## Files Created
- `src-tauri/migrations/002_onboarding.sql` — ALTER TABLE statements for 4 new columns
- `src-tauri/src/onboarding.rs` — Business logic: build_onboarding_state, complete_onboarding, mark_tooltip_seen, reset_onboarding

## Files Modified
- `src-tauri/src/state.rs` — Added 4 fields to UserSettings, added OnboardingState struct
- `src-tauri/src/db.rs` — Migration 002 runner with auto-complete logic, updated load_settings/save_settings for new columns, added 4 new tests
- `src-tauri/src/timer.rs` — Demo break returns to Paused (not Working) when onboarding not complete; first_break_completed tracking on break completion
- `src-tauri/src/commands.rs` — 5 new commands: get_onboarding_state, complete_onboarding, mark_tooltip_seen, trigger_demo_break, reset_onboarding
- `src-tauri/src/lib.rs` — Added `mod onboarding`, registered 5 new commands, timer starts in Paused if onboarding not complete
- `src/lib/types.ts` — Added OnboardingState interface, added 4 fields to UserSettings
- `src/lib/commands.ts` — Added 5 new command wrappers

## Deviations from Spec
- None. All contracts implemented exactly as specified.

## Acceptance Criteria Results
- [x] Migration 002 runs successfully on a fresh database (verified via test_onboarding_defaults_on_fresh_db)
- [x] Migration 002 runs successfully on an existing database with break records (verified via test_onboarding_auto_complete_for_existing_users)
- [x] `get_onboarding_state` returns `onboarding_completed: false` on fresh install (verified via test_build_onboarding_state_fresh)
- [x] `get_onboarding_state` returns `onboarding_completed: true` on existing install with data (verified via test_onboarding_auto_complete_for_existing_users)
- [x] `complete_onboarding` sets the flag and starts the timer (calls timer::resume after setting flag)
- [x] `mark_tooltip_seen("streak")` adds "streak" to the seen list (verified via test_mark_tooltip_seen)
- [x] `trigger_demo_break` starts a 5-second break and returns to paused when done (timer transitions to Breaking with 5s duration; on completion returns to Paused since onboarding_completed is false)
- [x] `reset_onboarding` clears all onboarding state and pauses the timer (verified via test_reset_onboarding)
- [x] Timer does NOT count down on a fresh install before `complete_onboarding` is called (starts in Paused phase)
- [x] Timer works normally for existing users (onboarding auto-completed by migration, starts in Working phase)
- [x] `cargo check` passes (3 pre-existing dead_code warnings only)
- [x] `cargo test` passes — 32 tests, 0 failures
- [x] `npx tsc --noEmit` passes — 0 type errors
- [x] All existing tests still pass (no regressions)

## Known Issues / TODOs for Later Blocks
- The `trigger_demo_break` command does not insert a break record into the DB (intentional — demo breaks shouldn't appear in analytics)
- The `first-break-celebrated` event is emitted from the timer's break completion handler, ready for Block 7 to consume on the frontend
- The `onboarding-completed` event is emitted from the `complete_onboarding` command, ready for Block 1 to listen to for UI transitions
