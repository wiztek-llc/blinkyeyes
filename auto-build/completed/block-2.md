# Block 2: System Tray — Completed

## Files Created or Modified

### Created
- `src-tauri/icons/tray-default.png` (22x22 RGBA eye icon, muted blue)
- `src-tauri/icons/tray-default@2x.png` (44x44 HiDPI variant)
- `src-tauri/icons/tray-active.png` (22x22 RGBA eye icon, green)
- `src-tauri/icons/tray-active@2x.png` (44x44 HiDPI variant)
- `src-tauri/icons/tray-paused.png` (22x22 RGBA eye icon, gray with pause indicator)
- `src-tauri/icons/tray-paused@2x.png` (44x44 HiDPI variant)

### Modified
- `src-tauri/src/tray.rs` — Full implementation (was a stub)
- `src-tauri/src/lib.rs` — Added `create_tray()` call in setup, `TrayMenuState` managed, `on_window_event` close handler
- `src-tauri/tauri.conf.json` — Main window `visible: false` (tray-managed)

## Deviations from Spec

- **No deviation from contracts.** All menu items, event handlers, and `update_tray_status` signature match the spec exactly.
- **Main window now `visible: false`** as the spec intended. Block 0 had kept it `true` for initial verification (noted in decisions.md).
- **`show_menu_on_left_click(false)`** — The deprecated `menu_on_left_click` was replaced with the Tauri 2.2+ replacement. Left-click toggles the window; right-click opens the context menu.

## Acceptance Criteria Results

- [x] App launches with a visible tray icon — `create_tray()` called in setup, icon embedded via `include_bytes!`
- [x] Left-clicking the icon toggles the main window — `handle_tray_event` matches `MouseButton::Left` + `MouseButtonState::Up`, toggles show/hide
- [x] Right-clicking shows the context menu — `show_menu_on_left_click(false)` ensures menu only opens on right-click
- [x] "Quit Blinky" exits the process — `app.exit(0)` in menu handler
- [x] "Open Dashboard" shows and focuses the main window — `window.show()` + `window.set_focus()`
- [x] The tooltip text reflects the current timer state — `update_tray_status` sets tooltip per phase (Working/Breaking/Paused/Suspended)
- [x] The icon changes when the phase changes — `set_icon()` called with phase-appropriate embedded PNG

## Verification Results

- `cargo check`: 0 errors (7 warnings from unused functions in other blocks — expected)
- `cargo test`: 11 tests passed, 0 failed
- `npx tsc --noEmit`: clean, no errors

## Known Issues / Notes for Future Blocks

- **Linux tray event limitation**: Per Tauri docs, `TrayIconEvent` is unsupported on Linux — the click event to toggle the window may not fire. The context menu still works via right-click. The "Open Dashboard" menu item provides an alternative way to show the window.
- **Window close = hide**: The `on_window_event` handler intercepts `CloseRequested` on all windows and hides instead of closing. The only way to quit is via the tray "Quit Blinky" menu item. This is standard tray-app behavior.
- **Icon quality**: Current tray icons are MVP-quality programmatic PNGs (simple eye shapes). Can be replaced with polished designs later without any code changes.
