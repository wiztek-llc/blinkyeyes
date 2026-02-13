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
