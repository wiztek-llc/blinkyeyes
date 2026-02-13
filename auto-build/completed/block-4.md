# Block 4: Notifications & Audio

## Files Created or Modified
- `src-tauri/src/notifications.rs` — Replaced stub with full implementation using `tauri-plugin-notification`
- `src-tauri/src/audio.rs` — Replaced stub with rodio-based playback on background thread
- `src-tauri/sounds/chime.wav` — Generated proper WAV file (was empty placeholder)

## Deviations from Spec
None. Implementation matches all spec requirements exactly.

## Acceptance Criteria Results

- [x] When a break starts, an OS notification appears — `send_break_notification` calls `app.notification().builder().title(...).body(...).show()`. Called by timer when `notification_enabled` is true.
- [x] The notification does NOT bring any window to the foreground — OS notifications are inherently non-intrusive; no window focus calls.
- [x] When a break completes, a chime sound plays — `play_chime` is called by timer when `sound_enabled` is true. Uses rodio `Decoder` + `Sink`.
- [x] The chime volume changes when `sound_volume` setting is adjusted — `play_chime` reads `sound_volume` from `AppState.settings` on each call and applies via `sink.set_volume()`.
- [x] Setting `sound_enabled: false` prevents the chime from playing — Timer checks `sound_enabled` before calling `play_chime` (in `Transition::CompleteBreak` handler).
- [x] Setting `notification_enabled: false` prevents the notification from firing — Timer checks `notification_enabled` before calling `send_break_notification` (in `Transition::StartBreak` handler).
- [x] Audio playback does not block or delay the timer loop — `play_chime` spawns a `std::thread::spawn` background thread that creates its own `OutputStream`, plays the sound, and exits.

## Known Issues or TODOs
- `send_break_complete_notification` is implemented but not called by the timer (spec says it's "optional, secondary to the chime"). Can be wired in during Block 10 integration if desired.
- Chime WAV is programmatically generated (880Hz sine + harmonic, exponential decay). Replace with a professionally produced sound for production polish.
- On Linux without a running PulseAudio/PipeWire daemon, `OutputStream::try_default()` will fail gracefully (logged to stderr, no crash).
- macOS notification permission dialog is triggered by the OS on first notification send; denial is handled gracefully (error logged, no crash).
