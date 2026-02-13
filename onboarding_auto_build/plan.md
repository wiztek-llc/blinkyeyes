# Blinky Onboarding — Full Implementation Spec

## How to Use This Document

**This is a blueprint for an LLM adding a world-class onboarding experience to an existing, fully functional Blinky app.** The app already works — timer, dashboard, settings, overlay, tray, analytics — everything described in `auto-build/plan.md` is built and operational. Your job is to layer onboarding on top without breaking anything.

**Key difference from the original build spec:** You are modifying an existing codebase, not building from scratch. Every change must be additive. Existing users who have already been using Blinky should never see the onboarding wizard — it's first-run only. The app must remain fully functional for returning users at all times.

**Rules for the implementing LLM:**
1. Read `auto-build/plan.md` and `auto-build/decisions.md` first to understand the existing architecture.
2. Work through blocks in order, following the dependency graph.
3. Never modify `001_initial.sql`. New DB changes go in `002_onboarding.sql`.
4. Test that the existing app still works after every block — don't just test the new feature.
5. Match the existing code style exactly. Look at how existing `.tsx` components, hooks, and Rust modules are written and follow the same patterns.

---

## What Is Blinky's Onboarding Problem?

Right now, Blinky has **zero first-time user experience.** Here's what happens when someone installs the app:

1. App launches. Timer immediately starts counting down from 20:00.
2. Dashboard shows empty charts and "0 breaks completed."
3. User has no idea what the 20-20-20 rule is, what the overlay will look like, or what any of the analytics mean.
4. 20 minutes later, a translucent pill appears at the top of their screen with no prior warning. They might think it's malware.

This is terrible. We need the opposite of this.

### What Great Onboarding Looks Like for Blinky

The ideal first-run experience:

1. **Welcome** — A warm screen that explains what Blinky does and why eye rest matters. The user feels informed, not confused.
2. **Quick setup** — The user picks their preferred work interval, notification style, and theme. Takes 15 seconds. The app feels personalized immediately.
3. **Preview** — The user sees exactly what a break reminder looks like before one surprises them. They can trigger a demo break. No surprises.
4. **Launch** — A satisfying "start" moment. The timer begins. The user knows exactly what to expect.
5. **Guided first session** — The dashboard has helpful empty states instead of blank charts. Contextual tooltips explain each metric on first visit. The first real break has extra encouragement.
6. **Celebration** — After the first completed break, a small celebration moment reinforces the habit.

The tone throughout: **warm, friendly, encouraging.** Like a gentle wellness coach, not a clinical health tool. "Your eyes will thank you" energy.

---

## Architecture Overview

### New Files

```
src/
├── pages/
│   └── Onboarding.tsx              # Multi-step onboarding wizard
├── components/
│   ├── onboarding/
│   │   ├── WelcomeStep.tsx         # Step 1: Welcome & 20-20-20 explanation
│   │   ├── SetupStep.tsx           # Step 2: Quick settings customization
│   │   ├── PreviewStep.tsx         # Step 3: Break preview & demo
│   │   └── ReadyStep.tsx           # Step 4: Summary & launch
│   ├── Tooltip.tsx                 # Reusable contextual tooltip component
│   └── EmptyState.tsx              # Reusable empty state component
├── hooks/
│   └── useOnboarding.ts            # Onboarding state management hook
src-tauri/
├── migrations/
│   └── 002_onboarding.sql          # New DB columns for onboarding state
├── src/
│   └── onboarding.rs               # Onboarding commands (Rust side)
```

### Modified Files

```
src-tauri/src/lib.rs                # Add onboarding module declaration
src-tauri/src/state.rs              # Add onboarding fields to UserSettings
src-tauri/src/db.rs                 # Run migration 002, load/save new fields
src-tauri/src/commands.rs           # Register new onboarding commands
src-tauri/src/timer.rs              # Respect onboarding state (don't auto-start)
src-tauri/src/main.rs               # Register new commands in generate_handler![]
src/lib/types.ts                    # Add onboarding TypeScript types
src/lib/commands.ts                 # Add onboarding command wrappers
src/App.tsx                         # Conditional routing based on onboarding state
src/pages/Dashboard.tsx             # Enhanced empty states, first-visit tooltips
src/components/TimerStatus.tsx      # Empty state for pre-first-break
src/components/StreakCard.tsx        # Encouraging first-day state
src/components/ComplianceRate.tsx    # Helpful empty state
src/components/DailyChart.tsx       # Guided empty state
src/components/WeeklyHeatmap.tsx    # Guided empty state
src/components/MiniOverlay.tsx      # Enhanced first-break experience
src/assets/styles.css               # New animations for onboarding
```

### New Contracts

**New fields on `UserSettings`:**

| Field | Type | Default | Purpose |
|-------|------|---------|---------|
| `onboarding_completed` | `bool` | `false` | Whether the user has finished the onboarding wizard |
| `onboarding_completed_at` | `Option<u64>` | `None` | Timestamp when onboarding was completed (for first-day logic) |
| `tooltips_seen` | `String` | `"[]"` | JSON array of tooltip IDs the user has dismissed |
| `first_break_completed` | `bool` | `false` | Whether the user has completed their very first break ever |

**New IPC Commands:**

| Command | Args | Returns | Purpose |
|---------|------|---------|---------|
| `get_onboarding_state` | none | `OnboardingState` | Get current onboarding progress |
| `complete_onboarding` | none | `OnboardingState` | Mark onboarding as finished, start timer |
| `mark_tooltip_seen` | `tooltip_id: String` | `Vec<String>` | Record a dismissed tooltip, return updated list |
| `trigger_demo_break` | none | `bool` | Start a short 5-second demo break for the preview step |
| `reset_onboarding` | none | `bool` | Reset onboarding state (for testing / "re-run onboarding" in settings) |

**New TypeScript Type:**

```typescript
interface OnboardingState {
  onboarding_completed: boolean;
  onboarding_completed_at: number | null;
  tooltips_seen: string[];
  first_break_completed: boolean;
  is_first_day: boolean; // computed: true if onboarding_completed_at is today
}
```

**New Events:**

| Event Name | Payload | When |
|------------|---------|------|
| `onboarding-completed` | `OnboardingState` | User finishes the wizard |
| `first-break-celebrated` | `null` | First-ever break was completed |

---

## Feature Block 0: Onboarding State Infrastructure

**Purpose:** Add the database schema, Rust types, IPC commands, and timer modifications needed to support onboarding. This is pure backend plumbing — no UI.

**Depends on:** Nothing (but requires the existing Blinky app to be fully built)
**Depended on by:** All other blocks

### Database Migration (002_onboarding.sql)

Add four columns to the `settings` table:

```sql
ALTER TABLE settings ADD COLUMN onboarding_completed INTEGER NOT NULL DEFAULT 0;
ALTER TABLE settings ADD COLUMN onboarding_completed_at INTEGER DEFAULT NULL;
ALTER TABLE settings ADD COLUMN tooltips_seen TEXT NOT NULL DEFAULT '[]';
ALTER TABLE settings ADD COLUMN first_break_completed INTEGER NOT NULL DEFAULT 0;
```

**Migration idempotency:** The migration runner (in `db.rs`) already tracks applied migrations via the `_migrations` table. Add `002_onboarding` as a new entry. Follow the exact same pattern as `001_initial.sql`.

**Important: Handle existing users.** If someone has been using Blinky before onboarding was added, they already have a settings row. The `ALTER TABLE ADD COLUMN ... DEFAULT` ensures existing rows get the default values. For these users, we should **auto-complete onboarding** — they don't need the wizard. Detection: if there are any `break_records` in the database when migration 002 runs, set `onboarding_completed = 1` and `first_break_completed = 1` on the existing settings row.

### Rust Type Changes

**In `state.rs`, add to `UserSettings`:**
```rust
pub onboarding_completed: bool,           // default false
pub onboarding_completed_at: Option<u64>, // default None
pub tooltips_seen: String,                // default "[]" (JSON array)
pub first_break_completed: bool,          // default false
```

These fields are serialized over IPC along with all other settings. The frontend will see them.

**New struct `OnboardingState`** (in `state.rs` or `onboarding.rs`):
```rust
#[derive(Serialize, Deserialize, Clone)]
pub struct OnboardingState {
    pub onboarding_completed: bool,
    pub onboarding_completed_at: Option<u64>,
    pub tooltips_seen: Vec<String>,
    pub first_break_completed: bool,
    pub is_first_day: bool,
}
```

Note: `tooltips_seen` is stored as a JSON string in the DB/UserSettings but exposed as `Vec<String>` in `OnboardingState` for frontend convenience. The conversion happens in the command layer.

### New Rust Module: `onboarding.rs`

This module contains the business logic for onboarding state management:

- `build_onboarding_state(settings: &UserSettings) -> OnboardingState` — Converts the raw settings fields into the computed `OnboardingState`. Parses `tooltips_seen` JSON string into `Vec<String>`. Computes `is_first_day` by checking if `onboarding_completed_at` falls on today's date (UTC).

- `complete_onboarding(settings: &mut UserSettings)` — Sets `onboarding_completed = true`, `onboarding_completed_at = now()`. Does NOT start the timer — that's the caller's responsibility.

- `mark_tooltip_seen(settings: &mut UserSettings, tooltip_id: &str) -> Vec<String>` — Parses the existing `tooltips_seen` JSON, adds the new ID if not already present, serializes back to JSON string, updates the settings. Returns the updated list.

- `reset_onboarding(settings: &mut UserSettings)` — Resets `onboarding_completed = false`, `onboarding_completed_at = None`, `tooltips_seen = "[]"`, `first_break_completed = false`.

### Timer Modification

**Critical change:** The timer currently starts immediately on app launch (in the `.setup()` closure of `main.rs`, the timer loop begins in `Working` phase). With onboarding, the timer should start in a **new initial state** if onboarding is not completed.

**Approach:** Add a check at the start of the timer loop. If `onboarding_completed == false`, the timer sits in `Paused` phase and emits tick events with a special indicator. It doesn't count down. Once `complete_onboarding` is called, the command handler sets `onboarding_completed = true` AND transitions the timer to `Working` phase. The timer loop picks this up on the next tick and starts counting normally.

**Why not just skip spawning the timer?** Because the timer loop also handles tray updates and idle detection. It's simpler to have it running but dormant than to spawn it conditionally.

**Alternative approach (simpler):** Start the timer in `Paused` phase if `!onboarding_completed`, and have `complete_onboarding` call `resume_timer` internally. This reuses existing pause/resume logic with zero new state machine states.

### IPC Commands (in `commands.rs`)

```rust
#[tauri::command]
fn get_onboarding_state(...) -> OnboardingState { ... }

#[tauri::command]
fn complete_onboarding(...) -> OnboardingState { ... }

#[tauri::command]
fn mark_tooltip_seen(tooltip_id: String, ...) -> Vec<String> { ... }

#[tauri::command]
fn trigger_demo_break(...) -> bool { ... }

#[tauri::command]
fn reset_onboarding(...) -> bool { ... }
```

**`trigger_demo_break`:** Temporarily transitions the timer to `Breaking` phase with a 5-second duration. When the demo break completes, the timer returns to `Paused` (not `Working`) since onboarding isn't done yet. The overlay should show during the demo break so the user can see it. This command should be a no-op if onboarding is already completed.

**`reset_onboarding`:** Resets all onboarding state. Useful for testing and for the "Re-run onboarding" option in settings. Should also pause the timer.

### TypeScript Types (in `types.ts`)

Add the `OnboardingState` interface matching the Rust struct. Add command wrappers in `commands.ts` for all five new commands.

### Registration

Add `mod onboarding;` to `lib.rs`. Add all five new commands to the `generate_handler![]` macro in `main.rs`.

### Acceptance Criteria

- [ ] Migration 002 runs successfully on a fresh database
- [ ] Migration 002 runs successfully on an existing database with break records (auto-completes onboarding)
- [ ] `get_onboarding_state` returns `onboarding_completed: false` on fresh install
- [ ] `get_onboarding_state` returns `onboarding_completed: true` on existing install with data
- [ ] `complete_onboarding` sets the flag and starts the timer
- [ ] `mark_tooltip_seen("streak")` adds "streak" to the seen list
- [ ] `trigger_demo_break` starts a 5-second break and returns to paused when done
- [ ] `reset_onboarding` clears all onboarding state and pauses the timer
- [ ] Timer does NOT count down on a fresh install before `complete_onboarding` is called
- [ ] Timer works normally for existing users (onboarding auto-completed by migration)
- [ ] `cargo check`, `cargo test`, `npx tsc --noEmit` all pass
- [ ] All existing tests still pass (no regressions)

---

## Feature Block 1: Welcome Wizard Shell & Routing

**Purpose:** Create the multi-step wizard UI framework and the conditional routing that shows the wizard on first launch. No step content yet — just the navigation skeleton.

**Depends on:** Block 0
**Depended on by:** Blocks 2, 3, 4 (step content)

### Conditional Routing (App.tsx)

On app mount, fetch `OnboardingState` via the `get_onboarding_state` command. Based on the result:
- If `onboarding_completed == false` → render the `Onboarding` page (full-screen, no nav bar)
- If `onboarding_completed == true` → render the normal app (Dashboard/Settings with nav bar)

**Loading state:** While the onboarding state is being fetched, show a minimal loading screen (the Blinky name centered on screen, maybe with a subtle fade-in). This prevents a flash of the wrong UI.

**Transition:** When onboarding completes, the wizard should smoothly transition to the dashboard. Don't use a hard page navigation — use a state change that swaps the rendered content with a fade/slide animation.

### Onboarding Page (pages/Onboarding.tsx)

A full-screen page with no nav bar, no chrome — just the wizard content centered on screen. The page manages:

- **Current step** (0-3): Stored in React state
- **Step navigation**: Next/Back buttons, with step indicator dots
- **Step transitions**: Smooth slide animations between steps (CSS transitions)
- **Settings accumulator**: A local state object that collects settings choices across steps, applied all at once when onboarding completes

The page renders four steps:
1. `WelcomeStep` — Welcome & 20-20-20 explanation
2. `SetupStep` — Quick settings customization
3. `PreviewStep` — Break preview & demo
4. `ReadyStep` — Summary & launch

Each step component receives:
- `onNext: () => void` — Advance to next step
- `onBack: () => void` — Go to previous step
- `settings: Partial<UserSettings>` — Accumulated settings so far
- `onUpdateSettings: (partial: Partial<UserSettings>) => void` — Update accumulated settings

### Step Indicator

A horizontal row of 4 dots at the top (or bottom) of the wizard. The current step's dot is filled/highlighted, completed steps are solid, future steps are hollow. Include step labels below the dots:
1. Welcome
2. Setup
3. Preview
4. Ready

The indicator should be minimal — don't let it compete with the step content.

### useOnboarding Hook (hooks/useOnboarding.ts)

Manages onboarding state for the entire app:

```typescript
function useOnboarding() {
  // Fetches OnboardingState on mount
  // Returns: { state, loading, completeOnboarding, markTooltipSeen, triggerDemoBreak, resetOnboarding }
  // completeOnboarding: calls the command, updates local state, triggers transition
  // Listens for onboarding-completed event to stay in sync
}
```

### Animation Details

Step transitions should feel smooth and directional:
- Going forward (Next): current step slides left, new step slides in from right
- Going back (Back): current step slides right, new step slides in from left
- Use CSS `transform: translateX()` with `transition` — no animation libraries needed
- Duration: 300ms ease-out

The overall wizard should fade in when the app loads (don't just pop into view).

### Visual Design

The onboarding wizard should feel distinct from the main app — it's a special moment, not just another page:
- Generous padding and whitespace
- Larger typography than the main app (the welcome heading should be big)
- Centered layout (content shouldn't stretch to fill the 480px width)
- Soft background — either the app's normal background or a very subtle gradient
- Step content should be vertically centered in the available space

### Acceptance Criteria

- [ ] Fresh install shows the onboarding wizard, not the dashboard
- [ ] Returning user (onboarding already completed) sees the dashboard directly
- [ ] Step indicator shows 4 steps with correct labels
- [ ] Next/Back navigation works between all 4 steps
- [ ] Back button is hidden on step 1
- [ ] Step transitions animate smoothly (slide direction matches navigation direction)
- [ ] The wizard is full-screen with no nav bar or other chrome
- [ ] Loading state shows briefly before the wizard/dashboard appears
- [ ] `npx tsc --noEmit` passes

---

## Feature Block 2: Welcome Step (Step 1)

**Purpose:** The user's first impression of Blinky. Explain what the app does, why eye rest matters, and make the user feel good about taking care of their eyes.

**Depends on:** Block 1
**Depended on by:** Block 6 (integration)

### Content & Layout

The welcome step has three sections, stacked vertically and centered:

**1. Hero / Branding**
- The Blinky name/logo, prominent but not overwhelming
- A tagline: "Gentle reminders to rest your eyes" or similar
- Optionally a simple eye icon or illustration (can be emoji-based for MVP: a large, friendly eye icon)

**2. The 20-20-20 Rule Explanation**

A visually appealing breakdown of the rule using three cards or an infographic-style layout:

```
Every 20 minutes    Look 20 feet away    For 20 seconds
     [icon]              [icon]               [icon]
  Work timer        Distance vision        Quick rest
```

The three "20s" should be visually prominent — they're the hook. Use icons or emoji to make each one scannable. Keep text minimal.

Below the three cards, a brief sentence of context:
> "Eye strain from screens is one of the most common health complaints for people who work at computers. The 20-20-20 rule is recommended by ophthalmologists as a simple, proven way to reduce it."

**3. Call to Action**
- A warm welcome message: "Let's set up Blinky to work the way you want."
- The "Get Started" / "Next" button, prominent and inviting

### Tone Guidelines

- Warm, not clinical: "Your eyes will thank you" not "Reduce ocular strain"
- Brief, not lecture-y: 2-3 sentences of context max, not a medical paper
- Encouraging: The user should feel good about caring for their eye health
- No guilt or fear: Don't use scary statistics about eye damage. Keep it positive.

### Animation

The three "20" cards should animate in with a staggered entrance:
- Card 1 fades in at 0ms
- Card 2 fades in at 150ms
- Card 3 fades in at 300ms

This creates a pleasant "one-two-three" reveal that draws the eye across the rule.

### Design Details

- The "20" numbers should be large and bold (text-4xl or larger)
- Cards should have soft backgrounds (e.g., `bg-blue-50 dark:bg-blue-950/30`)
- The explanatory text should be in a muted/secondary color
- Plenty of vertical spacing between sections
- The overall feel should be calm and welcoming — think wellness app, not productivity tool

### Acceptance Criteria

- [ ] The 20-20-20 rule is clearly explained with the three-part breakdown
- [ ] All text is warm and encouraging in tone
- [ ] The "Get Started" / Next button navigates to step 2
- [ ] The three cards animate in with staggered timing
- [ ] The layout looks good at the 480x640 window size
- [ ] Dark mode renders correctly
- [ ] No scrollbar needed — all content fits in the viewport

---

## Feature Block 3: Setup Step (Step 2)

**Purpose:** Let the user quickly personalize Blinky before their first session. This should take 10-15 seconds — just enough to feel ownership, not so much that it feels like a chore.

**Depends on:** Block 1
**Depended on by:** Block 6 (integration)

### Settings to Expose

Only the most impactful settings. The full settings page exists for fine-tuning later.

**1. Work Interval** — How often should Blinky remind you?

Present as 3-4 preset buttons (not a slider — presets are faster and less overwhelming):
- **15 min** — "Frequent breaks"
- **20 min** — "Recommended" (default, visually highlighted)
- **30 min** — "Fewer interruptions"
- **Custom** — Expands a small number input if selected

Each preset should have a brief descriptor so the user understands the tradeoff.

**2. Notifications** — How should Blinky get your attention?

Three toggles, presented as a checklist with friendly descriptions:
- **Screen overlay** (default: on) — "A gentle reminder floats at the top of your screen"
- **System notification** (default: on) — "A notification appears in your system tray"
- **Completion sound** (default: on) — "A soft chime plays when the break is over"

The user should be able to toggle each independently. At least one should remain on (show a gentle warning if all three are turned off: "Blinky won't be able to remind you with all notifications disabled").

**3. Theme** — Light, dark, or match your system?

Three visual option buttons showing mini-previews:
- **System** (default) — Shows a half-light/half-dark icon
- **Light** — Shows a sun icon
- **Dark** — Shows a moon icon

The theme should apply immediately when selected (live preview). The app background should change as the user clicks between options.

### Layout

Stack the three setting groups vertically with clear section headings. Each group should be compact:
- Section heading (e.g., "How often?")
- The controls
- Brief helper text if needed

Don't use a form style with labels on the left — use a card-based layout where each setting group is its own visual card.

### Settings Accumulation

The setup step doesn't save settings immediately. It accumulates choices in the parent wizard's state. Settings are saved all at once when onboarding completes (step 4). The only exception is theme — that should apply live for immediate visual feedback (but revert if the user navigates back and changes it).

### Acceptance Criteria

- [ ] Work interval presets work, with "20 min" highlighted as recommended
- [ ] Custom interval input appears when "Custom" is selected
- [ ] All three notification toggles work independently
- [ ] Warning appears if all notifications are disabled
- [ ] Theme selector applies the chosen theme live
- [ ] Settings choices persist when navigating back and forth between steps
- [ ] The layout fits in the viewport without scrolling
- [ ] Next and Back buttons work correctly
- [ ] Dark mode renders correctly

---

## Feature Block 4: Preview Step & Ready Step (Steps 3 & 4)

**Purpose:** Step 3 shows the user what a break looks like before one surprises them. Step 4 is the satisfying "launch" moment. These are grouped in one block because they're both small.

**Depends on:** Block 1, Block 0 (trigger_demo_break command)
**Depended on by:** Block 6 (integration)

### Preview Step (Step 3)

**Content:**

A section titled "Here's what a break looks like" with:

1. **Static preview** — A visual mockup of the break overlay, rendered inline in the wizard (not as an actual overlay window). Show the pill shape with the eye icon, "Look away — rest your eyes" text, a countdown number, and the progress ring. This should look exactly like the real overlay but embedded in the page.

2. **"Try it" button** — A button labeled "Try a break now" that triggers the actual overlay. When clicked:
   - Call `trigger_demo_break` (5-second demo break)
   - The real overlay window appears at the top of the screen
   - The countdown runs for 5 seconds
   - The chime plays at the end (if sound is enabled in the accumulated settings)
   - The overlay hides

3. **Explanatory text** — Brief copy below the preview:
   > "When it's time for a break, this gentle reminder will appear at the top of your screen. It won't steal your focus or interrupt what you're doing — you'll see a 20-second countdown, and when it's done, a soft chime lets you know."

4. **Reassurance** — Address the #1 concern:
   > "Don't worry — you can always skip a break if you're in the middle of something important."

**Design:**

The static preview should be rendered at ~80% scale and centered. Give it a subtle shadow or border to distinguish it from the page background. The "Try it" button should be inviting but secondary to the "Next" navigation.

### Ready Step (Step 4)

**Content:**

The final step before Blinky starts working. Three sections:

**1. Settings Summary**

A compact summary of what the user chose:
- Work interval: "Break every **20 minutes**"
- Notifications: "Overlay + Sound" (list the enabled ones)
- Theme: "Dark mode"

This is a confirmation — the user should feel confident about their choices. Include a "Change" link that navigates back to step 2.

**2. Encouragement**

A warm message that makes the "start" moment feel significant:
> "You're all set! Blinky will quietly remind you to rest your eyes throughout the day. Small habits make a big difference."

Or similar — the goal is to make the user feel good about this health habit before they start.

**3. Launch Button**

A large, prominent button: **"Start Protecting Your Eyes"**

When clicked:
1. Save all accumulated settings via `update_settings`
2. Call `complete_onboarding`
3. Transition smoothly to the dashboard

The button should feel like a satisfying "go" moment — give it a slightly different style from regular buttons (larger, maybe a subtle gradient or glow, or a filled primary color).

### Transition to Dashboard

After the launch button is clicked, the wizard should fade out and the dashboard should fade in. Don't use a hard navigation — animate the transition. The dashboard should already have the correct timer state (running, with the chosen work interval).

### Acceptance Criteria

- [ ] Static preview accurately represents the real overlay appearance
- [ ] "Try it" button triggers a real 5-second demo break with the overlay
- [ ] Demo break overlay appears, counts down, plays chime, and hides
- [ ] After demo break, the wizard is still showing (user returns to the preview step)
- [ ] Settings summary on the ready step correctly reflects all choices from step 2
- [ ] "Change" link navigates back to step 2
- [ ] "Start Protecting Your Eyes" button saves settings, completes onboarding, and starts the timer
- [ ] Smooth transition from wizard to dashboard after completion
- [ ] Timer is running and counting down when dashboard appears
- [ ] Dark mode renders correctly for both steps

---

## Feature Block 5: Enhanced Empty States

**Purpose:** Transform the dashboard from confusing-when-empty to encouraging-when-new. A first-time user should feel guided, not lost.

**Depends on:** Block 0 (onboarding state for first-day detection)
**Depended on by:** Block 6 (integration)

### Philosophy

Empty states are an onboarding surface. Instead of showing blank charts and zeros, each component should show:
1. What this metric **will** show once they have data
2. Gentle encouragement to take their first break
3. A sense of anticipation rather than absence

### EmptyState Component (components/EmptyState.tsx)

A reusable component for empty state displays:
```typescript
interface EmptyStateProps {
  icon: string;           // Emoji or icon name
  title: string;          // e.g., "Your streak starts today"
  description: string;    // e.g., "Complete breaks consistently to build a streak"
  compact?: boolean;      // For smaller card contexts
}
```

Renders a centered layout with the icon, title in medium weight, and description in muted color. Soft background (e.g., dashed border or very light fill). Should feel light and inviting, not like an error state.

### Component-Specific Empty States

**TimerStatus.tsx — No changes needed for empty state.** The timer always shows the countdown, even on day one. But if it's the first day and no breaks have been completed yet, add a subtle line below the timer: "Your first break is coming up — you'll see a gentle reminder when it's time."

**StreakCard.tsx:**
- When `current_day_streak == 0` AND it's the first day: Show "Your streak starts today!" with a flame icon and "Complete breaks every day to build a streak. Day one begins now."
- When `current_day_streak == 0` AND it's NOT the first day: Show the normal zero state (this is an existing user who broke their streak)

**ComplianceRate.tsx:**
- When no breaks today: Show the compliance circle at 0% with encouraging text: "Take your first break to see your completion rate here."

**DailyChart.tsx:**
- When all 7 days are zero: Show the chart frame (axis labels, day names) with a centered message overlay: "Your 7-day history will appear here as you use Blinky."
- When some days have data but today is zero: Show the chart normally with today's bar empty

**WeeklyHeatmap.tsx:**
- When all cells are zero: Show the grid structure with the lightest shade everywhere and a centered message: "Each day you use Blinky, this grid fills in. Aim for a solid wall of green!"

**Lifetime Stats (in Dashboard.tsx):**
- When `lifetime_breaks == 0`: "Start your lifetime stats with your first eye break."
- When it's the first day: Show the stats normally even if small ("1 lifetime break" is still a stat worth celebrating)

### First-Day Awareness

The dashboard should know whether it's the user's first day. Use the `is_first_day` field from `OnboardingState` (computed from `onboarding_completed_at`).

On the first day, add a subtle banner at the top of the dashboard (below the timer, above the metrics grid):
> "Welcome to your first day with Blinky! Each break you take gets tracked here."

The banner should have a dismiss button (X) and not reappear once dismissed. Use a subtle style — soft blue/green background, not an attention-grabbing alert.

### Acceptance Criteria

- [ ] Fresh install shows encouraging empty states, not blank/zero charts
- [ ] StreakCard shows "Your streak starts today!" on day one
- [ ] DailyChart shows the chart frame with a helpful message when empty
- [ ] WeeklyHeatmap shows the grid structure with helpful message when empty
- [ ] ComplianceRate shows encouraging text when no breaks today
- [ ] First-day banner appears at the top of the dashboard
- [ ] First-day banner can be dismissed
- [ ] After completing one break, empty states transition to showing real data
- [ ] Returning users with data see normal displays (empty states don't appear)
- [ ] Dark mode renders correctly for all empty states

---

## Feature Block 6: Contextual Tooltips

**Purpose:** On first visit to the dashboard, gently explain what each metric means. One-time-show tooltips that help users understand the analytics without reading documentation.

**Depends on:** Block 0 (tooltip tracking), Block 5 (empty states should be in place)
**Depended on by:** Block 8 (integration)

### Tooltip Component (components/Tooltip.tsx)

A reusable tooltip component that:
- Appears as a floating card near the target element
- Has an arrow/pointer indicating what it's explaining
- Contains a title, description text, and a "Got it" dismiss button
- Animates in (fade + slight scale-up from 0.95 to 1.0)
- Only shows once per tooltip ID (tracked via `mark_tooltip_seen`)
- Closes on "Got it" click or click-outside

```typescript
interface TooltipProps {
  id: string;              // Unique ID for tracking (e.g., "streak-card")
  title: string;
  description: string;
  position: 'top' | 'bottom' | 'left' | 'right';
  targetRef: RefObject<HTMLElement>;  // Element to attach to
  seen: string[];          // List of already-seen tooltip IDs
  onDismiss: (id: string) => void;
}
```

**Design:**
- White background (dark: dark gray) with subtle shadow
- Small arrow/triangle pointing to the target element
- Title in semibold, description in muted color
- "Got it" button is small and right-aligned
- Max width ~250px
- Z-index above everything else in the main window

### Tooltip Sequencing

Tooltips should NOT all appear at once — that's overwhelming. They appear one at a time, in sequence:

1. **First tooltip (timer):** Appears 2 seconds after the dashboard loads for the first time. Target: TimerStatus component.
   - ID: `"timer"`
   - Title: "Your Work Timer"
   - Description: "When this reaches zero, you'll get a gentle reminder to look away for 20 seconds. You can pause or skip anytime."

2. **Second tooltip (streak):** Appears after the first is dismissed. Target: StreakCard component.
   - ID: `"streak"`
   - Title: "Build a Streak"
   - Description: "Complete your daily break goal every day to keep your streak going. Your goal is set to {daily_goal} breaks per day."

3. **Third tooltip (compliance):** Appears after the second is dismissed. Target: ComplianceRate component.
   - ID: `"compliance"`
   - Title: "Completion Rate"
   - Description: "This shows what percentage of break reminders you completed instead of skipping. Don't stress about 100% — any breaks are good for your eyes!"

4. **Fourth tooltip (chart):** Appears after the third is dismissed. Target: DailyChart component.
   - ID: `"chart"`
   - Title: "Your History"
   - Description: "This chart tracks your daily breaks over the past week. Green means completed, orange means skipped."

### Pulsing Indicator

Before a tooltip is shown/dismissed, its target component should have a subtle pulsing dot (a small colored circle with a CSS pulse animation) in its top-right corner. This draws attention to the component without being obnoxious.

The pulse stops once the tooltip for that component has been seen.

### Tooltip State Management

Use the `useOnboarding` hook's `markTooltipSeen` function. The `tooltips_seen` array from `OnboardingState` determines which tooltips to skip.

The tooltip orchestration logic lives in Dashboard.tsx:
1. On mount, check which tooltips haven't been seen
2. Start showing them in sequence (with the 2-second initial delay)
3. When a tooltip is dismissed, check if there's a next one to show
4. If all tooltips have been seen, no indicators or tooltips appear

### Acceptance Criteria

- [ ] First dashboard visit shows tooltips one at a time in sequence
- [ ] Each tooltip points to the correct component
- [ ] "Got it" dismisses the current tooltip and shows the next one
- [ ] Dismissed tooltips don't reappear (persisted via `mark_tooltip_seen`)
- [ ] Pulsing dots appear on components with unseen tooltips
- [ ] Pulsing dots disappear after the tooltip is dismissed
- [ ] Returning users with all tooltips seen don't see any tooltips or dots
- [ ] Tooltips position correctly and don't overflow the window
- [ ] Click-outside also dismisses a tooltip
- [ ] Dark mode renders correctly

---

## Feature Block 7: First Break Celebration

**Purpose:** The first completed break is a milestone. Celebrate it to reinforce the habit and make the user feel good about their choice to use Blinky.

**Depends on:** Block 0 (first_break_completed tracking)
**Depended on by:** Block 8 (integration)

### Enhanced First Break Overlay

When the user's very first break starts (detected by `first_break_completed == false` in settings), the overlay should have slightly different text:

Instead of: "Look away — rest your eyes"
Show: "Your first break! Look at something far away..."

The rest of the overlay (countdown, progress ring, skip button) stays the same. This is a tiny detail but it acknowledges the milestone.

### Post-First-Break Celebration

When the first break completes (not skipped):

1. **Set `first_break_completed = true`** in settings (via a new internal call, not a user-facing command). This should happen in the timer engine when a break completes and it detects this is the first one.

2. **Emit `first-break-celebrated` event** so the frontend can react.

3. **Dashboard celebration card** — When the dashboard receives the `first-break-celebrated` event (or detects `first_break_completed` just became true), show a temporary celebration card at the top of the dashboard:
   - Appears with a bounce-in animation
   - Content: "You took your first break!" as a heading
   - Supporting text: "That's 20 seconds of rest your eyes needed. Keep it up — each break makes a difference."
   - A small celebratory icon/emoji (a party hat, sparkles, or simple checkmark with a glow)
   - Auto-dismisses after 10 seconds, or dismissible via X button
   - Shows only once (keyed on `first_break_completed` transition from false to true)

**No confetti or heavy animations.** Blinky's personality is calm and gentle. A celebration should feel like a warm smile, not a party. A subtle scale-up + fade-in animation on the card is enough.

### First-Day Evening Summary (Stretch Goal)

If it's late in the user's first day (say, 5+ breaks completed) and they open the dashboard, show a small summary at the top:

> "Great first day! You've rested your eyes {X} times for a total of {Y} minutes."

This is optional — implement it if the earlier celebration is done and there's bandwidth. Skip if not.

### Acceptance Criteria

- [ ] First break overlay shows "Your first break!" text instead of the standard message
- [ ] After first break completes, celebration card appears on the dashboard
- [ ] Celebration card has a gentle animation (no confetti or heavy effects)
- [ ] Celebration card auto-dismisses after 10 seconds
- [ ] Celebration card can be dismissed manually
- [ ] Second break and onwards show the standard overlay text
- [ ] `first_break_completed` is persisted — celebration doesn't re-trigger on app restart
- [ ] Skipping the first break does NOT trigger the celebration (only completion does)
- [ ] The `first-break-celebrated` event fires exactly once

---

## Feature Block 8: Integration, Polish & Settings Reset

**Purpose:** Wire everything together, add the "Re-run onboarding" option in settings, and verify the complete first-run experience end-to-end.

**Depends on:** All other blocks
**This should be the LAST block implemented**

### Settings Page Addition

Add a new item to the Settings page, in the "Danger zone" section (or a new "About" section at the bottom):

**"Re-run onboarding wizard"** — A text button (not dangerous-looking, just a link-style button) that:
1. Calls `reset_onboarding`
2. Transitions the UI back to the onboarding wizard
3. The wizard starts from step 1

This is useful for:
- Users who want to re-experience the onboarding
- Testing
- Users who want to see the 20-20-20 explanation again

### Full Integration Checklist

Go through each connection and verify it works:

| Connection | What to verify |
|------------|---------------|
| App.tsx routing | Fresh install → wizard. Returning user → dashboard. |
| Wizard → Settings | Settings accumulated during wizard are saved on completion |
| Wizard → Timer | Timer starts after "Start Protecting Your Eyes" is clicked |
| Demo break | "Try it" in preview step triggers real overlay for 5 seconds |
| Timer respects onboarding | Timer paused when onboarding not complete |
| Empty states | All dashboard components show helpful text when empty |
| Tooltips sequence | First dashboard visit shows tooltips one-at-a-time |
| Tooltip persistence | Dismissed tooltips stay dismissed across app restarts |
| First break overlay | First break shows "Your first break!" text |
| First break celebration | Celebration card appears on dashboard after first completed break |
| Theme live preview | Theme choice in wizard applies immediately |
| Existing user migration | User with existing break data skips onboarding entirely |
| Reset onboarding | "Re-run onboarding" in settings restarts the wizard |
| Tray interaction during onboarding | Tray icon visible but timer shows paused |

### End-to-End Test Scenarios

| Scenario | Steps | Expected Result |
|----------|-------|-----------------|
| **Brand new user** | Install and launch | Wizard appears, not dashboard |
| **Complete onboarding** | Go through all 4 wizard steps | Settings saved, timer starts, dashboard shows |
| **Demo break during onboarding** | Click "Try it" in step 3 | Overlay appears, 5s countdown, chime, overlay hides, wizard still showing |
| **Skip demo break** | Don't click "Try it", just click Next | Works fine, no break triggered |
| **First real break** | Wait for timer to count down (set to 1 min for testing) | Overlay shows "Your first break!", celebration after |
| **Skip first break** | Skip the first break | No celebration, standard overlay text on next break, celebration waits for completion |
| **Dashboard empty states** | Look at dashboard before any breaks | All components show helpful empty states, first-day banner visible |
| **Tooltip sequence** | Look at dashboard, follow tooltip sequence | 4 tooltips appear one-by-one, dots disappear as dismissed |
| **Existing user upgrade** | Launch app with existing break data | Dashboard appears directly, no wizard, no tooltips |
| **Re-run onboarding** | Click "Re-run onboarding" in settings | Wizard appears from step 1, timer pauses |
| **Theme during onboarding** | Switch theme in step 2 | Background changes immediately |
| **All notifications off warning** | Toggle all notifications off in step 2 | Warning message appears |
| **Back navigation** | Go to step 3, then back to step 2 | Settings choices preserved |
| **Dark mode throughout** | Set theme to dark in step 2, complete onboarding | All subsequent screens in dark mode |

### Performance

- [ ] Onboarding state check on app load completes in <50ms (shouldn't feel slow)
- [ ] Wizard step transitions are smooth (no dropped frames)
- [ ] Tooltip positioning doesn't cause layout shifts
- [ ] No memory leaks from event listeners in onboarding hooks

### Polish Checklist

- [ ] All text is grammatically correct with no typos
- [ ] All text follows the warm, friendly tone
- [ ] All animations are smooth (300ms ease-out for slides, 200ms for fades)
- [ ] Nothing overflows the 480x640 window
- [ ] All interactive elements have hover/active states
- [ ] Focus management is correct (keyboard navigation works through the wizard)
- [ ] The app looks cohesive — onboarding doesn't feel like a separate app bolted on

### Acceptance Criteria

- [ ] The complete first-run experience works end-to-end without errors
- [ ] All 14 integration checklist items verified
- [ ] All 14 E2E test scenarios pass
- [ ] "Re-run onboarding" in settings works correctly
- [ ] All existing app functionality still works for returning users
- [ ] `cargo check`, `cargo test`, `npx tsc --noEmit` all pass
- [ ] No regressions in existing tests

---

## Dependency Graph

```
                    ┌──────────┐
                    │ Block 0  │  MUST BE FIRST
                    │ Infra    │  (DB, types, commands, timer mod)
                    └────┬─────┘
                         │
           ┌─────────────┼─────────────┐
           │             │             │
      ┌────▼──┐    ┌────▼──┐    ┌────▼──┐
      │Blk 1  │    │Blk 5  │    │Blk 7  │
      │Wizard │    │Empty  │    │First  │
      │Shell  │    │States │    │Break  │
      └──┬────┘    └───────┘    │Celeb. │
         │                      └───────┘
   ┌─────┼─────┐
   │     │     │
┌──▼─┐ ┌▼───┐ ┌▼────┐
│Blk │ │Blk │ │Blk  │
│ 2  │ │ 3  │ │ 4   │
│Welc│ │Set │ │Prev │
│ome │ │ up │ │iew+ │
└────┘ └────┘ │Ready│
              └─────┘
         │
    ┌────▼──┐
    │Blk 6  │    (after Blk 0 + Blk 5)
    │Tooltip│
    │System │
    └───────┘
         │
    ┌────▼──┐
    │Blk 8  │  MUST BE LAST
    │Integr.│
    └───────┘
```

**Maximum parallelism: 3 blocks** after Block 0 is done — Blocks 1, 5, and 7 are independent.
After Block 1 is done — Blocks 2, 3, and 4 are independent of each other.
Block 6 depends on Blocks 0 and 5.
Block 8 depends on everything.

**Recommended sequential order:**
0 → 1 → 5 → 7 → 2 → 3 → 4 → 6 → 8

This ordering builds the infrastructure first, then the independent UI pieces, then wires everything together.

---

## Appendix A: Copy & Tone Guide

All user-facing text in the onboarding should follow these guidelines:

**Voice:** Warm, friendly wellness coach. Not clinical, not corporate, not cutesy.

**Good examples:**
- "Your eyes will thank you."
- "Small habits make a big difference."
- "A gentle reminder will appear at the top of your screen."
- "Don't stress about 100% — any breaks are good for your eyes!"

**Bad examples:**
- "Reduce ocular strain by 37% with regular screen breaks." (too clinical)
- "You NEED to take breaks or your eyes WILL suffer!" (fear-based)
- "Yay! Let's make your eyeballs super happy! " (too cutesy)
- "Configure your notification preferences." (too corporate)

**Capitalization:** Sentence case for everything except proper nouns. "Start protecting your eyes" not "Start Protecting Your Eyes" (the exception being the launch button, which uses title case for emphasis).

**Numbers:** Use digits, not words. "Every 20 minutes" not "every twenty minutes."

## Appendix B: Animation Reference

All animations should be CSS-only (no JavaScript animation loops, no animation libraries):

| Animation | Duration | Easing | CSS Property |
|-----------|----------|--------|--------------|
| Step slide | 300ms | ease-out | transform: translateX() |
| Fade in | 200ms | ease-in | opacity |
| Card entrance (celebration) | 400ms | cubic-bezier(0.34, 1.56, 0.64, 1) | transform: scale() + opacity |
| Tooltip appear | 200ms | ease-out | transform: scale() + opacity |
| Staggered cards | 150ms delay between | ease-out | opacity + transform: translateY(10px) |
| Pulse dot | 2s infinite | ease-in-out | transform: scale() + opacity |
| Progress indicator fill | 300ms | ease-out | width |

## Appendix C: Testing Shortcuts

- **Short timer for testing:** Set work interval to 1 minute in the setup step to test the break cycle quickly.
- **Reset onboarding:** Use the "Re-run onboarding" button in settings, or call `reset_onboarding` from the console/dev tools.
- **Skip to specific step:** During development, you can temporarily hardcode `initialStep` in Onboarding.tsx.
- **Test existing user flow:** Insert a fake break record before running migration 002 — the migration should auto-complete onboarding.
