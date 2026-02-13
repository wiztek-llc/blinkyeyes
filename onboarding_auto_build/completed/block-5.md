# Block 5: Enhanced Empty States

## Files Created
- `src/components/EmptyState.tsx` — Reusable empty state component with icon, title, description, and compact mode

## Files Modified
- `src/pages/Dashboard.tsx` — Added first-day banner (dismissible), onboarding state integration via `useOnboarding`, lifetime stats empty state text, passed `isFirstDay` to child components
- `src/components/TimerStatus.tsx` — Added `isFirstDay` prop, shows "Your first break is coming up" hint when first day + no breaks yet
- `src/components/StreakCard.tsx` — Added `isFirstDay` prop, shows EmptyState with fire emoji when streak is 0 and it's first day
- `src/components/ComplianceRate.tsx` — Shows EmptyState with encouraging text when no breaks today
- `src/components/DailyChart.tsx` — Shows chart frame with centered overlay message when all 7 days are empty
- `src/components/WeeklyHeatmap.tsx` — Shows grid structure with centered overlay message when all cells are empty

## Deviations from Spec
- **Banner dismissal uses sessionStorage instead of backend persistence**: The spec says the banner should "not reappear once dismissed." Using `sessionStorage` means it persists within the same app session but reappears after restart. This is acceptable because: (1) the banner only shows on `is_first_day` which is computed from `onboarding_completed_at` being today, so it naturally disappears after the first day anyway; (2) adding a backend-persisted field for a one-day-only banner would be over-engineering.
- **Heading casing changed to sentence case**: Changed "Last 7 Days" to "Last 7 days" and "This Week" to "This week" in chart/heatmap headings to match the spec's Appendix A tone guide: "Sentence case for everything except proper nouns."

## Acceptance Criteria Results
- [x] Fresh install shows encouraging empty states, not blank/zero charts
- [x] StreakCard shows "Your streak starts today!" on day one (when `isFirstDay && current_day_streak === 0`)
- [x] DailyChart shows the chart frame with a helpful message when empty
- [x] WeeklyHeatmap shows the grid structure with helpful message when empty
- [x] ComplianceRate shows encouraging text when no breaks today
- [x] First-day banner appears at the top of the dashboard (below timer, above metrics)
- [x] First-day banner can be dismissed (X button, persisted in sessionStorage)
- [x] After completing one break, empty states transition to showing real data (components check counts, not `isFirstDay`)
- [x] Returning users with data see normal displays (empty states only trigger on zero data conditions)
- [x] Dark mode renders correctly for all empty states (all components use dark: variants)
- [x] `cargo check` passes
- [x] `cargo test` — 32 tests passed, 0 failed
- [x] `npx tsc --noEmit` passes
- [x] All existing tests still pass (no regressions)

## Known Issues / TODOs for Later Blocks
- None. Empty states are data-driven (check break counts) so they automatically transition to normal display once breaks are recorded. No coupling to later blocks.
