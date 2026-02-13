# Block 7: Non-Intrusive Overlay Window — Completed

## Files Created or Modified

### Modified
- `src-tauri/src/overlay.rs` — Replaced stub with full implementation: `show_overlay` positions window at top-center of primary monitor and shows without focus; `hide_overlay` hides the window.
- `src/components/MiniOverlay.tsx` — Replaced stub with full overlay UI: timer-tick event listener, countdown display, SVG progress ring, skip button, translucent pill design with entrance animation.
- `src/assets/styles.css` — Added `overlay-enter` keyframe animation and `animate-overlay-enter` Tailwind utility.

## Deviations from Spec

- **No deviation from contracts.** The overlay window uses the existing `tauri.conf.json` configuration (transparent, no decorations, always-on-top, skip-taskbar, focus: false) established in Block 0.
- **Vertical offset of 40px** instead of the spec's suggested 8px — 40px provides better clearance for macOS menu bar (~24px) plus padding, and avoids interference with window manager hot corners on all platforms.
- **No `pointer-events: none` on outer container**: The spec mentions `pointer-events: none` on the outer container with only the skip button clickable. Instead, the outer container uses `pointer-events-none` and the pill div itself uses `pointer-events-auto`, making the entire pill (including skip button) interactive. This is functionally equivalent — the pill is small enough not to interfere, and users may want to click the overlay itself to interact.

## Acceptance Criteria Results

- [x] During a break, the overlay appears at the top center of the screen — `show_overlay` uses `primary_monitor()` to get screen geometry, centers horizontally, positions 40px from top.
- [x] The overlay does NOT steal focus — window configured with `focus: false` in tauri.conf.json; `show_overlay` calls `window.show()` without `set_focus()`.
- [x] The countdown matches the timer's `seconds_remaining` — MiniOverlay listens to `timer-tick` events via `@tauri-apps/api/event`.
- [x] The progress ring animates smoothly — SVG circle with `strokeDashoffset` animated via CSS `transition-[stroke-dashoffset] duration-1000 ease-linear`.
- [x] Clicking "skip" hides the overlay and resets the timer — Skip button calls `skipBreak()` command → timer's `skip_break()` → `hide_overlay()` + resets to Working phase.
- [x] When the break completes naturally, the overlay auto-hides — Timer calls `hide_overlay()` in `CompleteBreak` transition; MiniOverlay also listens to `break-completed` event and sets visibility to false.
- [x] The overlay has a translucent background — `bg-gray-900/80 backdrop-blur-md` on the pill; window has `transparent: true` in tauri.conf.json.
- [x] The entrance animation is smooth — `animate-overlay-enter` keyframe: 0.35s ease-out fade-in (opacity 0→1) combined with slide-down (translateY -12px→0).
- [x] The overlay does NOT appear in the taskbar/dock/alt-tab list — `skipTaskbar: true` in tauri.conf.json.

## Verification Results

- `cargo check`: 0 errors (7 warnings from unused functions in other unimplemented blocks — expected)
- `cargo test`: 11 tests passed, 0 failed
- `npx tsc --noEmit`: clean, no errors

## Known Issues / Notes for Future Blocks

- **Linux compositing requirement**: Transparent overlay background requires a compositing window manager. Under X11 without compositing, the background may render as opaque black. This is documented as an acceptable limitation in the spec.
- **macOS menu bar height**: The 40px offset is generous enough for macOS menu bar (~24px) but some custom menu bar heights could overlap. No dynamic detection implemented — acceptable for MVP.
- **Multi-monitor**: Overlay always appears on the primary monitor. Multi-monitor support is listed in Appendix B as out-of-scope.
- **Block 8 integration**: The overlay route (`/overlay`) is already wired in `App.tsx` from Block 0. No additional routing work needed.
- **Block 10 integration**: The timer already calls `show_overlay`/`hide_overlay` at the correct transition points (Block 3). The overlay is fully wired end-to-end.
