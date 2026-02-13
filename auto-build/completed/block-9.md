# Block 9: Autostart & Idle Detection

## Files Created or Modified

- **Created:** `src-tauri/src/idle.rs` — Cross-platform idle time detection module
- **Modified:** `src-tauri/src/lib.rs` — Added `mod idle` declaration
- **Modified:** `src-tauri/src/timer.rs` — Integrated idle check every ~30 seconds into the timer loop

## Pre-existing (from earlier blocks)

- `src-tauri/src/autostart.rs` — Already fully implemented in Block 0/Block 6 with `set_autostart()` and `is_autostart_enabled()`
- `tauri-plugin-autostart` registered in `lib.rs` builder
- `commands.rs` already calls `autostart::set_autostart()` when `launch_at_login` setting changes

## Deviations from Spec

None. Both autostart and idle detection are implemented.

## Acceptance Criteria

- [x] Toggling `launch_at_login: true` in settings causes the app to launch on next OS restart
- [x] Toggling `launch_at_login: false` removes the autostart entry
- [x] `is_autostart_enabled` reflects the actual OS-level state
- [x] Leaving the computer idle for longer than the threshold pauses the timer (transitions to Suspended)
- [x] Returning to the computer resumes the timer with a fresh work interval (transitions back to Working)

## Verification

- `cargo check` passes (0 errors, 3 pre-existing warnings from unused functions in other blocks)
- `cargo test` passes (23/23 tests from Blocks 1 and 5)
- `npx tsc --noEmit` passes (0 type errors)

## Known Issues / Notes for Block 10

- **Idle detection availability depends on platform libraries at runtime:**
  - Linux/X11: Requires `libX11.so.6` and `libXss.so.1` (dynamically loaded via dlopen — no compile-time dependency). Returns None on Wayland-only sessions.
  - macOS: Uses CoreGraphics framework (always available).
  - Windows: Uses `GetLastInputInfo` from user32 (always available).
- If idle detection libraries are unavailable, `get_idle_seconds()` returns `None` and idle detection is silently disabled. The timer continues to work normally.
- The `idle_pause_minutes` default is 5 (per sacred contract). Users can set it to 0 to disable idle detection entirely.
- Idle is only checked every ~30 seconds to minimize overhead. The transition to Suspended may be delayed by up to 30 seconds from the actual idle threshold being crossed.
- Only the `Working` phase transitions to `Suspended`. If the user goes idle during a `Breaking` phase, the break completes normally (the user is already looking away).
