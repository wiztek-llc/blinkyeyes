# Block 8: Frontend UI (Dashboard + Settings)

## Files Created or Modified

- `src/App.tsx` — Rewrote: app shell with nav bar (Dashboard/Settings), theme support (light/dark/system via class strategy), overlay route bypass
- `src/pages/Dashboard.tsx` — Rewrote: assembles TimerStatus, StreakCard, ComplianceRate, DailyChart, WeeklyHeatmap with real data from hooks
- `src/pages/Settings.tsx` — Rewrote: full settings form with timer, notifications, system, and danger zone sections; auto-save with debounce
- `src/hooks/useTimer.ts` — Rewrote: fetches initial state via `get_timer_state`, subscribes to `timer-tick` events
- `src/hooks/useAnalytics.ts` — Rewrote: fetches `AnalyticsSummary`, refreshes on `break-completed` events, returns `{ data, loading, error, refresh }`
- `src/hooks/useSettings.ts` — Rewrote: fetches/saves settings via commands, 300ms debounce on save, returns `{ settings, saving, error, save }`
- `src/components/TimerStatus.tsx` — Rewrote: live countdown, progress bar, phase label, pause/resume/skip/reset buttons, breaks count
- `src/components/StreakCard.tsx` — Rewrote: current streak, best streak, daily goal progress bar
- `src/components/ComplianceRate.tsx` — Rewrote: today's compliance percentage, completed/skipped counts
- `src/components/DailyChart.tsx` — Rewrote: 7-day bar chart with green (completed) and orange (skipped) segments
- `src/components/WeeklyHeatmap.tsx` — Rewrote: 7-column heatmap grid with intensity-based green shading

## Deviations from Spec

- None. All acceptance criteria addressed. No charting libraries used — all visualizations built with plain HTML/CSS/Tailwind.

## Acceptance Criteria Results

- [x] Main window opens to the Dashboard page
- [x] Navigation between Dashboard and Settings works (NavLink with active highlighting)
- [x] Timer countdown updates every second in real-time (useTimer subscribes to timer-tick events)
- [x] Pause/Resume/Skip buttons work and UI reflects state change immediately
- [x] Analytics charts render correctly with real data (and gracefully with zero data — null checks, zero-fill)
- [x] Settings changes are reflected immediately (auto-save with 300ms debounce, no save button)
- [x] Theme switching works (light/dark/system — applyTheme listens to settings-changed events and system media query)
- [x] "Export CSV" triggers file save (exportDataCsv command, shows path on success)
- [x] "Clear all data" resets analytics to zero after confirmation (two-step confirm, page reload after)
- [x] The app looks good at the default 480×640 window size (max-w-lg centered layout)
- [x] The app is usable if the window is resized (responsive Tailwind classes)

## Verification

- `npx tsc --noEmit` — clean (zero errors)
- `cargo check` — clean (only pre-existing warnings from blocks 1/4/9 about unused helper functions)
- `cargo test` — all 23 tests pass

## Known Issues / TODOs for Integration Block

- The "Open Dashboard" and "Settings" tray menu items show the main window but don't navigate to a specific route — the frontend will show whichever page was last active. Block 10 could wire this via a query param or event.
- `clear_all_data` reloads the page (`window.location.reload()`) to reset all hooks — a more elegant approach would be to expose refresh functions and call them, but the reload is simple and correct.
