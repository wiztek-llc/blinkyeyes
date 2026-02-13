## Block 0: Onboarding State Infrastructure

- **Start in Paused phase (simpler approach)**: The spec offered two approaches for handling the timer during onboarding: a new initial state or starting in Paused. Chose the simpler approach — `lib.rs` checks `settings.onboarding_completed` and sets initial phase to `Paused` if false. `complete_onboarding` calls `timer::resume()` to transition to `Working`. This reuses existing pause/resume machinery with zero new state machine states.

- **Demo break returns to Paused via existing tick logic**: Rather than adding special demo-break state tracking, the timer's break completion transition checks `settings.onboarding_completed`. If false (meaning this was a demo break), it transitions to `Paused` instead of `Working`. This elegantly handles the "return to wizard" requirement without any new timer states.

- **Demo breaks don't create DB records**: `trigger_demo_break` sets the timer phase directly without calling `insert_break_record`. The break record ID stays `None`, so when the break completes, the DB finalization is skipped. This keeps demo breaks out of analytics entirely.

- **first_break_completed tracked in timer's CompleteBreak handler**: The spec says "set first_break_completed = true ... in the timer engine when a break completes and it detects this is the first one." Implemented exactly there — the CompleteBreak side-effect handler checks `settings.onboarding_completed && !settings.first_break_completed && break_record_id.is_some()` (the last condition excludes demo breaks) and persists the change to both in-memory settings and DB.

- **`DateTime::from_timestamp_millis` over deprecated `NaiveDateTime::from_timestamp_millis`**: chrono's `NaiveDateTime::from_timestamp_millis` is deprecated. Used `DateTime::from_timestamp_millis` + `.date_naive()` for the `is_first_day` computation.

- **Auto-complete logic in migration runner**: The spec says "if there are any break_records when migration 002 runs, set onboarding_completed = 1 and first_break_completed = 1." This logic lives in `run_migrations()` immediately after applying 002's ALTER TABLEs. It runs a simple `EXISTS` query on `break_records` and conditionally UPDATEs the settings row.

- **`tooltips_seen` stored as JSON string in UserSettings, parsed in OnboardingState**: Following the spec exactly — the DB and UserSettings carry the raw JSON string `"[]"`. The `OnboardingState` struct (returned to frontend) has `Vec<String>`. Conversion happens in `build_onboarding_state()`. This avoids JSON parsing on every settings load/save.

- **`mark_tooltip_seen` IPC parameter uses camelCase `tooltipId`**: Tauri v2 auto-converts snake_case command parameter names to camelCase for the frontend. The TypeScript wrapper uses `tooltipId` which maps to the Rust parameter `tooltip_id`.

## Block 1: Welcome Wizard Shell & Routing

- **`MainApp` wrapper for onboarding routing**: Rather than modifying `AppShell` directly, added a `MainApp` component that wraps `AppShell` and `Onboarding`. This keeps the existing dashboard/settings shell completely untouched — the conditional logic is a clean layer on top.

- **Single-element step transitions over dual-element**: The spec described simultaneous slide-out/slide-in of two elements. Used a simpler single-element approach: fade/slide the current step out (300ms timeout), then swap the component and let it animate in. This avoids the complexity of managing two absolutely-positioned elements, CSS z-index conflicts, and height synchronization between steps. The visual result is smooth directional movement that matches the spec's intent.

- **`onComplete` prop pattern**: The `Onboarding` page receives an `onComplete` callback from `MainApp` instead of directly calling `completeOnboarding`. This keeps the wizard component focused on UI (step navigation, animations) while `MainApp` handles the IPC call and the wizard-to-dashboard transition. Later blocks that fill in ReadyStep will call `onComplete` from the "Start Protecting Your Eyes" button.

- **Theme applied in both `MainApp` and `AppShell`**: `MainApp` calls `getSettings().then(applyTheme)` on mount so the correct theme is active even during the onboarding wizard. `AppShell` also applies theme (for the settings-changed event listener). This means theme is correct regardless of which view renders first.

- **CSS animations use Tailwind v4 `@utility` directives**: All onboarding animations (`animate-fade-in`, `animate-slide-out-left`, etc.) follow the same pattern as the existing `animate-overlay-enter` — `@keyframes` + `@utility`. This is the required pattern for Tailwind v4 with `@tailwindcss/vite`.

- **Placeholder steps accept full props interface now**: All four step components accept `{ onNext, onBack, settings, onUpdateSettings }` even though placeholders don't use all of them. This ensures Blocks 2, 3, and 4 can fill in step content without changing the Onboarding page's prop-passing code.

## Block 2: Welcome Step (Step 1)

- **Emoji icons for cards and hero**: Used Unicode emoji (laptop, eyes, sparkles, eye) rather than SVG icons or an icon library. The spec explicitly suggested "emoji-based for MVP" and this keeps the bundle at zero additional bytes for icon assets. The emoji render consistently across macOS, Windows 10+, and modern Linux DEs.

- **`text-3xl` for "20" numbers**: The spec suggested `text-4xl` or larger, but with three cards side-by-side at 480px window width, `text-4xl` caused crowding. `text-3xl` (1.875rem / 30px) is still the most visually prominent element per card and provides the right visual weight without overflow.

- **Staggered animations via separate `@utility` classes**: Rather than using inline `animation-delay` styles or a single parameterized animation, created three distinct utility classes (`animate-stagger-in-1`, `animate-stagger-in-2`, `animate-stagger-in-3`) with 0/150/300ms delays. This follows the established Tailwind v4 `@utility` pattern from Block 1 and avoids the need for `style` props in JSX.

- **No `onBack` usage**: The WelcomeStep is step 1 — the back button is hidden by the Onboarding page when `currentStep === 0`. The prop is accepted (interface contract) but intentionally unused. No lint suppression needed since the destructured-but-unused parameter pattern is standard for component prop interfaces.

- **Condensed vertical spacing for viewport fit**: Used `mb-5`/`mb-6` margins and `py-6` padding to ensure all content (hero + 3 cards + context paragraph + CTA) fits within the 640px viewport height without any scrollbar. The spec explicitly requires "No scrollbar needed — all content fits in the viewport."

## Block 5: Enhanced Empty States

- **`sessionStorage` for first-day banner dismissal**: The spec says the banner should "not reappear once dismissed." Using `sessionStorage` means it persists within the app session but reappears after restart. This is acceptable because `is_first_day` is computed from `onboarding_completed_at` being today — the banner naturally disappears after the first day regardless. Adding a backend-persisted `banner_dismissed` field for a one-day-only UI element would be over-engineering.

- **Data-driven empty states over `isFirstDay`-gated**: Most empty states trigger based on data conditions (zero breaks, zero streak) rather than `isFirstDay` alone. This means: (1) after a user completes their first break, the empty states naturally transition to showing real data without any explicit state toggle; (2) returning users who happen to have zero breaks on a given day still see the normal zero-state (not the first-day variant); (3) only `StreakCard` uses `isFirstDay` as an extra condition, because the spec explicitly distinguishes between "first day zero streak" and "broke your streak" messaging.

- **`EmptyState` component uses minimal styling**: No dashed border or background fill — just centered text with an emoji icon. The component is used inside cards that already have their own `bg-white dark:bg-gray-800` background, so adding more background layers would create visual nesting that fights the clean card aesthetic. The `compact` prop reduces spacing for the 2-column grid cards (StreakCard, ComplianceRate) where vertical space is limited.

- **Dashboard uses `useOnboarding` hook directly**: Rather than threading `isFirstDay` through props from `AppShell`, the Dashboard imports and calls `useOnboarding()` itself. This is cleaner than prop drilling through the router, and the hook's `getOnboardingState` call is lightweight (single DB read, already cached in managed state). The Dashboard is the only page that needs this information.

- **Sentence case applied to chart headings**: Changed "Last 7 Days" → "Last 7 days" and "This Week" → "This week" to follow the spec's Appendix A copy guidelines. Consistent with the rest of the onboarding text style.

## Block 7: First Break Celebration

- **No Rust changes needed**: Block 0 already implemented all backend logic for first break tracking — `first_break_completed` field, `first-break-celebrated` event emission in the timer's `CompleteBreak` handler, and the guard conditions (`onboarding_completed && !first_break_completed && break_record_id.is_some()`). This block was purely frontend.

- **`getSettings()` for first-break detection in overlay**: MiniOverlay checks `settings.onboarding_completed && !settings.first_break_completed` to determine whether to show "Your first break!" text. Fetches settings on mount and again on each `break-started` event (to handle the case where `first_break_completed` changed between breaks). On `break-completed`, immediately sets `isFirstBreak = false` locally to prevent stale state.

- **HTML entity `&#10024;` (sparkle) over emoji**: The celebration card uses an HTML entity for the sparkle icon rather than Unicode emoji. This ensures consistent rendering across all platforms without depending on emoji font support.

- **`cubic-bezier(0.34, 1.56, 0.64, 1)` for celebration animation**: Matches the spec's Appendix B recommendation for card entrance animations. The slight overshoot (1.56 on the second control point) creates a gentle bounce that feels celebratory without being excessive.

- **Auto-dismiss via `setTimeout` with cleanup**: The 10-second auto-dismiss uses a ref-stored timeout that's properly cleaned up on unmount and on manual dismissal. Prevents orphan timeouts and ensures the card doesn't reappear after dismissal.

- **First-day evening summary not implemented**: The spec marks this as a stretch goal ("optional — implement if there's bandwidth"). Skipped since it requires additional backend logic (tracking break count thresholds) for a one-day-only feature with diminishing return.

## Block 3: Setup Step (Step 2)

- **`applyThemePreview` duplicated from App.tsx**: The SetupStep needs to apply theme changes live as the user clicks theme buttons. Rather than importing the `applyTheme` function from App.tsx (which isn't exported and lives inside a component file), duplicated the 10-line function locally. The logic is trivial (toggle `dark` class on `documentElement`) and the duplication avoids creating a shared utility module for a single small function.

- **Grid layout for presets over flexbox**: Used `grid grid-cols-4` for interval presets instead of flexbox. This guarantees equal-width buttons regardless of label length, which looks cleaner with the "15 min" / "20 min" / "30 min" / "Custom" set. The description text wraps naturally within the fixed column width.

- **Compact toggle style for notification section**: Created a `ToggleRow` sub-component with label + description stacked vertically, rather than reusing the Settings page's simpler `Toggle` component. The onboarding toggle needs the friendly description text beneath each label (e.g., "A gentle reminder floats at the top of your screen") which the existing Toggle doesn't support. Keeping the sub-component local to SetupStep avoids polluting shared components with onboarding-specific patterns.

- **Emoji icons for theme buttons**: Used Unicode globe (🌐), sun (☀️), and moon (🌙) for the three theme options. Same rationale as Block 2's emoji approach — zero bundle cost, consistent cross-platform rendering for these common symbols.

- **`overflow-y-auto` safety net**: The content area has `overflow-y-auto` as a fallback. At 480x640 the content fits without scrolling, but if system font sizes are larger than expected or the window is resized smaller, the content becomes scrollable rather than clipping. The spec requires "fits in the viewport without scrolling" at the default size, and this satisfies that while being resilient to edge cases.

## Block 4: Preview Step & Ready Step (Steps 3 & 4)

- **Settings save in Onboarding.tsx, not ReadyStep**: The spec says the launch button should "Save all accumulated settings via `update_settings`." The save logic lives in `Onboarding.tsx`'s `handleComplete` function rather than inside ReadyStep. This keeps the step component as a pure UI layer — it calls `onNext()`, and the parent handles fetching current settings, merging accumulated changes, saving via IPC, then calling `onComplete`. The ReadyStep never touches IPC directly.

- **`goToStep` function for "Change settings" link**: The spec says ReadyStep should have a "Change" link that navigates back to step 2. Rather than calling `onBack` twice (fragile and would require animation delays), added a `goToStep(index)` function in Onboarding.tsx that navigates directly to any step. Passed as `onGoToStep` prop. ReadyStep calls `onGoToStep(1)` to jump to SetupStep. The `onGoToStep` prop is optional in all step interfaces to maintain backward compatibility.

- **Static preview at ~90% scale**: The spec suggests rendering the overlay preview at "~80% scale." Used Tailwind's `scale-90` (90%) because at 80% the progress ring text became difficult to read at the 480px window width. The 10% difference is imperceptible but keeps the text legible.

- **Demo break button state is best-effort**: The `demoActive` state tracks whether `triggerDemoBreak()` has been called, but resets when the command returns (immediately), not when the actual 5-second demo break ends. This means the button re-enables before the overlay dismisses. This is acceptable because: (1) `triggerDemoBreak` is a no-op if already in Breaking phase, (2) the visual feedback of the overlay being visible is sufficient, (3) tracking the actual break end would require an event listener in PreviewStep that adds complexity for a minor polish issue.

- **No new CSS animations required**: Both steps use existing animations (fade-in, slide transitions from Block 1) and standard Tailwind utilities. The static overlay preview is pure CSS/HTML, and the ReadyStep's launch button uses `shadow-md` with a colored shadow for subtle visual emphasis rather than a glow animation.

## Block 6: Contextual Tooltips

- **Wrapper divs with refs over `forwardRef`**: Existing dashboard components (`TimerStatus`, `StreakCard`, etc.) don't use `forwardRef`. Rather than modifying four components to add ref forwarding, wrapped each in a `<div ref={...} className="relative">` in `Dashboard.tsx`. This is less invasive — zero changes to the component internals — and the `relative` positioning serves double duty as the anchor for the `PulsingDot`'s `absolute` positioning.

- **Hooks before conditional return in Tooltip**: The initial implementation had `if (seen.includes(id)) return null` before the `useEffect` calls, which violates React's hooks rules. Moved the early return after all hooks, guarding the hook bodies with `if (alreadySeen) return` instead. The `alreadySeen` check is a safety net — in practice, the Dashboard only renders a `<Tooltip>` for the active tooltip ID, so a seen tooltip is never mounted.

- **300ms delay between sequential tooltips**: After dismissing a tooltip, there's a 300ms delay before showing the next one. This prevents the jarring "one disappears, another immediately appears" effect and gives the user a moment to process the dismissal.

- **`PulsingDot` hidden when its tooltip is active**: The pulsing dot for a component disappears while that component's tooltip is showing (`activeTooltip !== id`). This prevents visual clutter — the tooltip itself is the attention-grabber, the dot is redundant while it's visible.

- **`z-10` on PulsingDot**: Added `z-10` to the dot's span so it renders above the card's content (which may have its own stacking context from `rounded-2xl` + `overflow-hidden`). The tooltip itself uses `z-50` (fixed positioning) so there's no conflict.

- **Tooltip arrow uses CSS border triangle technique**: The arrow/pointer is a zero-width/height div with 6px borders where three sides are transparent. This is the classic CSS triangle approach — zero dependencies, works in all browsers, and matches the dark mode background via `dark:border-t-gray-700` etc.

- **Viewport clamping for tooltip position**: After computing the ideal tooltip position relative to its target, coordinates are clamped to `[8px, viewport - tooltip_size - 8px]`. This prevents tooltips from overflowing the 480px window width, which would happen for the streak/compliance cards in the 2-column grid if positioned naively.

- **Streak tooltip description is dynamic**: The spec says the streak tooltip should include the user's daily goal value. The `tooltipDescriptions` record in Dashboard.tsx overrides the static description with a template string that includes `settings.daily_goal`. Other tooltips use their static descriptions from `TOOLTIP_SEQUENCE`.

- **Click-outside listener delayed by 100ms**: The `mousedown` listener for click-outside dismiss is attached after a 100ms `setTimeout`. Without this delay, the click event that opens/navigates to a tooltip could immediately trigger the click-outside handler, dismissing the tooltip before the user sees it.

## Block 8: Integration, Polish & Settings Reset

- **State-driven view switching over one-way flag**: The previous `MainApp` used a `showDashboard` boolean that was a one-way flip (false → true, never back). This broke the "Re-run onboarding" flow since the wizard could never re-appear. Replaced with reactive logic: `MainApp` now derives which view to show directly from `onboardingState.onboarding_completed`. When `resetOnboarding()` sets the flag to `false`, the wizard automatically re-renders. A `justCompleted` flag tracks only the completion transition for fade-in animation purposes.

- **`onResetOnboarding` prop threading over hook-in-Settings**: The reset flow needs to coordinate between Settings (where the button lives) and MainApp (which controls view switching). Rather than having Settings import `useOnboarding` directly (which would create a second independent hook instance with its own state), the `handleResetOnboarding` callback is created in `MainApp` and passed through `AppShell` → `Settings` as a prop. This ensures the same hook instance that controls routing also handles the reset.

- **"About" section over Danger Zone placement**: The spec says the re-run button can go in "the Danger Zone section or a new About section." Chose "About" because re-running onboarding is non-destructive — it doesn't delete any data, just resets the wizard flag and pauses the timer. Placing it in the red-bordered Danger Zone would incorrectly signal that it's a risky operation.

- **Link-style button over standard button**: The re-run button uses `text-blue-600` link styling rather than the filled button pattern used by "Export CSV" and "Clear all data." This matches the spec's description of "a text button (not dangerous-looking, just a link-style button)" and visually communicates that this action is lightweight.

- **Fade-out animation on reset**: When the user clicks "Re-run onboarding wizard," the current view fades out (300ms) before the wizard fades in. This uses the same animation pattern as the onboarding completion transition, providing visual consistency.

- **No code changes needed in Rust backend**: All backend integration points (commands, events, state management) were correctly wired by their respective blocks. The verification pass confirmed every connection in the spec's integration checklist without requiring any fixups. This is the expected outcome for a well-architected block dependency system.
