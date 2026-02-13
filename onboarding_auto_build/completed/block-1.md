# Block 1: Welcome Wizard Shell & Routing

## Files Created
- `src/pages/Onboarding.tsx` — Multi-step wizard with step navigation, indicator dots, and slide animations
- `src/hooks/useOnboarding.ts` — Onboarding state management hook (fetches state, exposes completeOnboarding/markTooltipSeen/triggerDemoBreak/resetOnboarding)
- `src/components/onboarding/WelcomeStep.tsx` — Placeholder step 1 (content to be built in Block 2)
- `src/components/onboarding/SetupStep.tsx` — Placeholder step 2 (content to be built in Block 3)
- `src/components/onboarding/PreviewStep.tsx` — Placeholder step 3 (content to be built in Block 4)
- `src/components/onboarding/ReadyStep.tsx` — Placeholder step 4 (content to be built in Block 4)

## Files Modified
- `src/App.tsx` — Added conditional routing: loading → onboarding wizard (if not completed) → dashboard (if completed). Smooth fade transition between wizard and dashboard on completion. Overlay route bypass preserved.
- `src/assets/styles.css` — Added 5 CSS animation utilities: `animate-fade-in`, `animate-fade-out`, `animate-slide-in`, `animate-slide-out-left`, `animate-slide-out-right` using Tailwind v4 `@utility` directives.

## Deviations from Spec
- Step transitions use opacity + translateX fade/slide rather than a continuous slide-through. The spec described "current step slides left, new step slides in from right" which implies two elements animating simultaneously. Instead, the current step fades/slides out (300ms), then the new step component mounts with a slide-in. This is simpler (no dual-element orchestration) and produces a clean directional feel that matches the spec's intent.
- The `onComplete` prop on `Onboarding` is called from the last step's `onNext` callback, keeping the wizard decoupled from the completion logic. The actual `completeOnboarding` IPC call and dashboard transition live in `App.tsx`'s `MainApp` component.

## Acceptance Criteria Results
- [x] Fresh install shows the onboarding wizard, not the dashboard (conditional routing in MainApp checks `onboarding_completed`)
- [x] Returning user (onboarding already completed) sees the dashboard directly
- [x] Step indicator shows 4 steps with correct labels (Welcome, Setup, Preview, Ready)
- [x] Next/Back navigation works between all 4 steps
- [x] Back button is hidden on step 1 (WelcomeStep doesn't render a Back button)
- [x] Step transitions animate smoothly (slide direction matches navigation direction — left for forward, right for back)
- [x] The wizard is full-screen with no nav bar or other chrome
- [x] Loading state shows briefly before the wizard/dashboard appears ("Blinky" centered with fade-in)
- [x] `npx tsc --noEmit` passes — 0 type errors
- [x] `cargo check` passes — 3 pre-existing dead_code warnings only
- [x] `cargo test` passes — 32 tests, 0 failures
- [x] All existing tests still pass (no regressions)

## Known Issues / TODOs for Later Blocks
- Step components are placeholders — WelcomeStep content (Block 2), SetupStep content (Block 3), PreviewStep + ReadyStep content (Block 4) need to be implemented
- The `onUpdateSettings` prop is passed to all steps but only used by SetupStep (Block 3) and ReadyStep (Block 4)
- Theme application during onboarding: `MainApp` applies theme on mount, but SetupStep's live theme preview (Block 3) will need to call `applyTheme` directly when the user changes the theme selector
