# Block 2: Welcome Step (Step 1)

## Files Modified
- `src/components/onboarding/WelcomeStep.tsx` — Full implementation replacing placeholder
- `src/assets/styles.css` — Added staggered card entrance animations (`animate-stagger-in-1/2/3`)

## Files Created
None

## Deviations from Spec
- **Emoji icons instead of custom SVGs**: Used emoji characters (laptop, eyes, sparkles) for the three rule cards and the hero eye icon. The spec suggested "emoji-based for MVP" and this keeps the bundle small with zero additional assets.
- **`text-3xl` for "20" numbers instead of `text-4xl`**: The spec suggested `text-4xl` or larger for the "20" numbers, but at the 480px window width with three side-by-side cards, `text-3xl` provides better proportions without crowding. The numbers are still the most visually prominent elements in each card.
- **Slightly condensed vertical spacing**: Used tighter margins (`mb-5`, `mb-6`) to ensure all content fits comfortably in the 640px viewport height without scrolling, while preserving the generous whitespace feel.

## Acceptance Criteria Results
- [x] The 20-20-20 rule is clearly explained with the three-part breakdown
- [x] All text is warm and encouraging in tone
- [x] The "Get started" / Next button navigates to step 2
- [x] The three cards animate in with staggered timing (0ms, 150ms, 300ms delays)
- [x] The layout looks good at the 480x640 window size (no scrollbar needed)
- [x] Dark mode renders correctly (blue-50 cards → blue-950/30, all text has dark: variants)
- [x] No scrollbar needed — all content fits in the viewport
- [x] `npx tsc --noEmit` passes
- [x] `cargo check` passes
- [x] `cargo test` — 32 tests passed, 0 failed

## Known Issues / TODOs for Later Blocks
- None. This block is self-contained UI with no backend dependencies beyond what Block 0/1 already provides.
