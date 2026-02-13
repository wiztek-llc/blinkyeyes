# Block 0: Project Scaffolding & Build Pipeline

## Files Created

### Root configs
- `package.json` — React 19, Tauri v2, Vite 6, Tailwind v4, TypeScript ~5.8
- `tsconfig.json` — ES2020, react-jsx, bundler moduleResolution, noEmit
- `tsconfig.node.json` — for Vite config type checking
- `.gitignore` — added `src-tauri/target/`, `src-tauri/gen/`, `dist/`
- `vite.config.ts` — React + Tailwind v4 Vite plugins, port 1420, Tauri dev host
- `tailwind.config.js` — stub (Tailwind v4 uses CSS `@import "tailwindcss"`)
- `index.html` — root HTML loading `/src/main.tsx`

### Frontend (`src/`)
- `main.tsx` — React entry point with BrowserRouter
- `App.tsx` — Router shell (Dashboard `/`, Settings `/settings`, Overlay `/overlay`)
- `vite-env.d.ts` — Vite type declarations
- `lib/types.ts` — All sacred contract types (TimerPhase, TimerState, UserSettings, BreakRecord, DailyStats, AnalyticsSummary)
- `lib/commands.ts` — Typed `invoke()` wrappers for all 12 IPC commands
- `assets/styles.css` — `@import "tailwindcss";`
- `pages/Dashboard.tsx` — stub with all component imports
- `pages/Settings.tsx` — stub
- `components/TimerStatus.tsx` — stub
- `components/StreakCard.tsx` — stub
- `components/DailyChart.tsx` — stub
- `components/WeeklyHeatmap.tsx` — stub
- `components/ComplianceRate.tsx` — stub
- `components/MiniOverlay.tsx` — stub
- `hooks/useTimer.ts` — stub returning null
- `hooks/useSettings.ts` — stub returning defaults
- `hooks/useAnalytics.ts` — stub returning null

### Files Removed
- `src/index.ts` — replaced by `src/main.tsx`

### Tauri Backend (`src-tauri/`)
- `Cargo.toml` — tauri v2 (tray-icon, image-png), tauri-plugin-{notification,autostart,shell}, serde, rusqlite (bundled), rodio, chrono (serde), dirs, tokio (full)
- `build.rs` — `tauri_build::build()`
- `tauri.conf.json` — Two windows: main (480x640, visible, centered) and overlay (360x80, hidden, no decorations, transparent, always-on-top, skipTaskbar)
- `capabilities/default.json` — core:default, window show/hide/close/focus/center, events, notification, autostart, shell
- `icons/` — RGBA placeholder PNGs (32x32, 128x128, 128x128@2x, icon.png, icon.ico, icon.icns)
- `sounds/chime.wav` — empty placeholder
- `migrations/001_initial.sql` — comment stub
- `src/lib.rs` — Declares all 11 modules, exports `pub fn run()` with Tauri builder + notification/autostart/shell plugins
- `src/main.rs` — Calls `blinky_lib::run()`
- `src/state.rs` — stub
- `src/tray.rs` — stub
- `src/timer.rs` — stub
- `src/db.rs` — stub
- `src/analytics.rs` — stub
- `src/commands.rs` — stub
- `src/settings.rs` — stub
- `src/notifications.rs` — stub
- `src/audio.rs` — stub
- `src/autostart.rs` — stub
- `src/overlay.rs` — stub

## Deviations from Spec

- **Tailwind v4** (not v3): Uses `@import "tailwindcss"` in CSS instead of `@tailwind` directives. `tailwind.config.js` exists as a stub for spec compliance but isn't actively used.
- **Main window `visible: true`** for Block 0 testing: Spec says `visible: false`, but we need to verify the window appears. Later blocks (tray) will manage visibility.
- **`vite-env.d.ts` added**: Not in spec file tree but required for Vite type declarations (`/// <reference types="vite/client" />`).
- **Vite 6** instead of Vite 7: `vite@^7` does not exist on npm yet. Used `^6.0.0` which resolved to v6.4.1.
- **Icons are solid-blue RGBA PNGs**: Minimal valid placeholder icons generated via Python. They satisfy Tauri's RGBA requirement but should be replaced with real branding later.

## Acceptance Criteria Results

| Criteria | Result |
|----------|--------|
| `npm run tauri dev` launches without errors | PASS — compiled 579 crates, ran `target/debug/blinky` |
| A native window appears | PASS — app launched with main window |
| `cargo check` in `src-tauri/` succeeds | PASS — zero errors |
| `npx tsc --noEmit` succeeds | PASS — zero errors |
| All files in canonical file tree exist | PASS — verified all files present |
| Overlay window defined in `tauri.conf.json` | PASS — second window with alwaysOnTop, transparent, no decorations |

## Known Issues / TODOs for Next Blocks

- Icons are placeholder blue squares — replace with real Blinky branding when ready
- `chime.wav` is an empty file — Block 4 (audio) will need a real sound file
- Main window is `visible: true` — Block with tray implementation should change to `visible: false` and manage via tray
- `rusqlite` pinned at 0.31 and `rodio` at 0.19 — newer versions available but these are stable and match Tauri v2 ecosystem
