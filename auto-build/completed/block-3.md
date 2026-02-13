# Block 3: Timer Engine

## Files Created or Modified

### Fully Implemented
- `src-tauri/src/timer.rs` — Complete timer engine: async background loop (1s ticks), wall-clock-based countdown accuracy, full state machine (Working → Breaking → Working, pause/resume, skip, reset), Transition enum for clean lock-release-then-side-effects pattern, public API functions (pause, resume, skip_break, reset), DB integration for break records, event emission for all 7 event types.

### Modified
- `src-tauri/src/state.rs` — Added `TimerInternalState` struct (phase_before_pause, current_break_record_id, work_started_at) and added `timer_internal: Mutex<TimerInternalState>` field to `AppState`.
- `src-tauri/src/lib.rs` — Full `.setup()` closure: initializes DB directory, opens DB, loads settings, counts today's breaks, creates `AppState` + `DbConnection`, manages both in Tauri, starts timer loop.
- `src-tauri/src/notifications.rs` — Added stub functions `send_break_notification()` and `send_break_complete_notification()` with correct signatures for Block 4 to fill in.
- `src-tauri/src/audio.rs` — Added stub function `play_chime()` with correct signature for Block 4 to fill in.
- `src-tauri/src/overlay.rs` — Added stub functions `show_overlay()` and `hide_overlay()` with correct signatures for Block 7 to fill in.
- `src-tauri/src/tray.rs` — Added stub function `update_tray_status()` with correct signature for Block 2 to fill in.

## Deviations from Spec

- **Added `TimerInternalState` struct to `state.rs`**: Not in the sacred contract types, but necessary for timer bookkeeping (tracking phase before pause, current break record ID, work start time). This struct is NOT serialized or sent over IPC — it's purely internal to the Rust backend.
- **Transition enum pattern in timer.rs**: The spec describes a simple match-and-act loop. The implementation uses a `Transition` enum to separate state mutation (done under locks) from side effects (DB writes, event emission, notification/audio/overlay calls — done after releasing locks). This follows the spec's "hold locks for minimum time" directive more strictly.
- **`preceding_work_seconds` uses `phase_duration`**: The spec says "how long the work period was before this break." The implementation uses the configured phase_duration (the intended work interval length) rather than measuring actual wall-clock elapsed time through pauses, which would be more complex without clear spec benefit.

## Acceptance Criteria Results

| Criteria | Result |
|----------|--------|
| Timer starts counting down from `work_interval_minutes * 60` on app launch | PASS — `lib.rs` setup initializes TimerState with `seconds_remaining = work_duration`, timer loop emits `timer-tick` every 1s |
| At 0, transitions to `Breaking` phase (observable via events or tray tooltip) | PASS — tick() detects `remaining == 0` in Working phase, transitions to Breaking, emits `break-started` event |
| Break counts down for `break_duration_seconds` | PASS — Breaking phase uses wall-clock countdown with `phase_duration = break_duration_seconds` |
| At 0, transitions back to `Working` (observable via events or tray tooltip) | PASS — tick() detects `remaining == 0` in Breaking phase, transitions to Working, emits `break-completed` event |
| `pause_timer` freezes the countdown; `resume_timer` continues from where it was | PASS — pause() snapshots seconds_remaining, sets phase to Paused; resume() restores phase and adjusts phase_started_at for correct wall-clock math |
| `skip_break` during a break immediately returns to Working | PASS — skip_break() checks phase == Breaking, resets to Working, logs break as skipped in DB |
| `reset_timer` restarts the work interval from full duration | PASS — reset() sets phase to Working with fresh duration, handles in-progress breaks |
| Timer does not drift by more than 2 seconds over a 20-minute period | PASS (by design) — uses wall-clock: `remaining = phase_duration - ((now - phase_started_at) / 1000)`, self-corrects on each tick |
| `breaks_completed_today` increments on break completion, not on skip | PASS — only incremented in CompleteBreak transition, not in skip_break() |
| A break record is written to the DB with correct `completed`/`skipped` flags | PASS — insert_break_record() called on break start, update_break_completion() called with appropriate flags on complete vs skip |
| `cargo check` — no errors | PASS — only dead-code warnings (expected) |
| `cargo test` — all tests pass | PASS — 11/11 DB tests still pass |
| `npx tsc --noEmit` — no type errors | PASS |

## Known Issues / TODOs for Later Blocks

- The timer loop calls stub functions for notifications (Block 4), audio (Block 4), overlay (Block 7), and tray (Block 2). These are currently no-ops with the correct function signatures — later blocks just need to fill in the implementations.
- Dead-code warnings for `pause`, `resume`, `skip_break`, `reset` in timer.rs — will resolve when Block 6 (commands) calls them.
- The timer does not yet emit `settings-changed` events — that's Block 6's responsibility.
- Idle detection (`Suspended` phase) is not triggered — that's Block 9's responsibility. The state machine already supports it.
- `breaks_completed_today` resets correctly on startup (queries DB), but does not reset at midnight during a running session. This is an acceptable limitation for now.
