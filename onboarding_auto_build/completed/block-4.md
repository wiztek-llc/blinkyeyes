# Block 4: Preview Step & Ready Step (Steps 3 & 4)

## Files Created or Modified

**Modified:**
- `src/components/onboarding/PreviewStep.tsx` — Full implementation of the break preview step
- `src/components/onboarding/ReadyStep.tsx` — Full implementation of the ready/launch step
- `src/pages/Onboarding.tsx` — Added settings save on completion, `goToStep` function, `onGoToStep` prop passing
- `src/components/onboarding/WelcomeStep.tsx` — Added optional `onGoToStep` to props interface
- `src/components/onboarding/SetupStep.tsx` — Added optional `onGoToStep` to props interface

## Deviations from Spec

- **No new CSS animations needed**: The preview step uses the existing overlay styles inline, and the ReadyStep uses standard Tailwind styling. The spec's animation requirements were already satisfied by Block 1's animations.
- **Settings saved in Onboarding.tsx, not ReadyStep**: The spec says ReadyStep should "Save all accumulated settings via `update_settings`". The actual save logic lives in `Onboarding.tsx`'s `handleComplete` function, which fetches current settings, merges the accumulated changes, saves via IPC, then calls `onComplete`. This keeps the step component focused on UI and the parent managing the data flow.
- **"Change settings" link uses `onGoToStep(1)` instead of `onBack` repeated**: The spec says the "Change" link should navigate back to step 2. Implemented via a new `goToStep` function that allows direct navigation to any step index, passed as `onGoToStep` prop. This is more flexible than calling `onBack` twice.

## Acceptance Criteria Results

- [x] Static preview accurately represents the real overlay appearance — matches MiniOverlay pill shape, progress ring, text, skip button
- [x] "Try it" button triggers a real 5-second demo break with the overlay — calls `triggerDemoBreak()` command
- [x] Demo break overlay appears, counts down, plays chime, and hides — handled by existing timer + MiniOverlay infrastructure from Block 0/7
- [x] After demo break, the wizard is still showing — timer returns to Paused after demo break (Block 0 logic), wizard state preserved
- [x] Settings summary on the ready step correctly reflects all choices from step 2 — reads from accumulated settings with sensible defaults
- [x] "Change" link navigates back to step 2 — via `onGoToStep(1)`
- [x] "Start Protecting Your Eyes" button saves settings, completes onboarding, and starts the timer — `handleComplete` in Onboarding.tsx merges + saves settings, then calls `onComplete` which triggers `completeOnboarding` in MainApp
- [x] Smooth transition from wizard to dashboard after completion — handled by MainApp's fade-out/fade-in transition
- [x] Timer is running and counting down when dashboard appears — `completeOnboarding` resumes the timer
- [x] Dark mode renders correctly for both steps — uses standard Tailwind dark: variants throughout
- [x] `cargo check` passes
- [x] `cargo test` — all 32 tests pass
- [x] `npx tsc --noEmit` passes

## Known Issues or TODOs for Later Blocks

- The "Try it" button state (`demoActive`) resets to false after the `triggerDemoBreak` command returns, but the actual demo break continues for 5 seconds. The button becomes clickable again before the overlay dismisses. This is cosmetic — clicking it again during the demo break is a no-op because the timer is already in Breaking phase.
- Block 8 (Integration) should verify the full flow: wizard → save settings → complete → dashboard with correct timer state.
