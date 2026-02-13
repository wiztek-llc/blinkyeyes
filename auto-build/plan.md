# Blinky — Parallelizable Feature Implementation Spec

## How to Use This Document

**This is a blueprint for an LLM building Blinky from scratch, working block-by-block in sequence.** Each feature block tells you *exactly* what to build, *why* it exists, *how it connects* to everything else, and *how to verify it works* — but does NOT hand you the code. You write the code. You make the implementation decisions. This document is your product manager, architect, and QA team rolled into one.

**Rules for the implementing LLM:**
1. Work through blocks in order (Block 0 first, then sequentially or following the dependency graph).
2. Before starting a block, read the ENTIRE block — especially "Integration Contracts" and "Acceptance Criteria."
3. Every block's contracts are sacred. Types, event names, command signatures — these are the seams where blocks connect. Don't deviate.
4. If a block says "this must be non-blocking," that's a hard constraint, not a suggestion.
5. After finishing each block, run every acceptance criterion before moving on.

---

## What Is Blinky?

Blinky is a cross-platform desktop app (macOS, Windows, Linux) that implements the **20-20-20 rule**: every 20 minutes, it reminds you to look at something 20 feet away for 20 seconds. The reminder is non-intrusive — no modal dialogs, no focus stealing, no interrupting your flow. A gentle chime plays when the 20-second rest period ends. The app tracks every break event and provides analytics on your eye-rest habits over time.

### Core Experience (The User's Perspective)

You install Blinky. It appears in your system tray. You forget about it. Twenty minutes later, a small translucent pill slides down from the top of your screen: "👀 Look away — rest your eyes" with a 20-second countdown. You glance out the window. The pill counts down. A soft chime plays. The pill vanishes. You go back to work. This happens all day, quietly. When you're curious, you click the tray icon and see a dashboard: how many breaks you've taken today, your weekly streak, your compliance rate over time. You feel good about taking care of your eyes.

That's it. The app should feel like it barely exists.

### Why These Tech Choices

| Choice | Why |
|--------|-----|
| **Tauri v2** (Rust + Web) | Blinky is a *utility* — it runs 8+ hours/day in the background. It MUST be tiny in memory (~15MB, not Electron's ~150MB+), fast to launch, and produce small binaries (~3MB). Tauri gives us native OS integration (tray, notifications, autostart) at a fraction of Electron's cost. |
| **React + TypeScript** frontend | The settings panel and analytics dashboard are standard UI work. React is the pragmatic choice for a small app with a few interactive pages. |
| **SQLite** (via `rusqlite`, bundled) | Analytics need persistent storage. SQLite is embedded, zero-config, and handles Blinky's data volume (a few dozen rows per day) effortlessly. No server, no setup. |
| **Tailwind CSS** | Rapid styling for a small UI. No CSS architecture decisions needed. |
| **Vite** | Fast dev server, standard bundler for React+Tauri. |

---

## Architecture Overview

```
┌──────────────────────────────────────────────────────────────────────────┐
│                            TAURI SHELL                                   │
│                                                                          │
│   RUST BACKEND (runs natively, manages all state)                        │
│   ┌────────────┐ ┌──────────────┐ ┌────────────┐ ┌────────────────────┐ │
│   │System Tray │ │ Timer Engine │ │  Database   │ │  Notification +   │ │
│   │            │ │  (async loop │ │  (SQLite)   │ │  Audio            │ │
│   │• Icon      │ │   1s ticks)  │ │             │ │                   │ │
│   │• Menu      │ │              │ │• breaks     │ │• OS notifications │ │
│   │• Tooltip   │ │• Work phase  │ │• settings   │ │• Chime playback   │ │
│   │            │ │• Break phase │ │• daily cache │ │                   │ │
│   │            │ │• Pause/Skip  │ │             │ │                   │ │
│   └─────┬──────┘ └──────┬───────┘ └──────┬─────┘ └────────────────────┘ │
│         │               │                │                               │
│         └───────┬───────┴────────────────┘                               │
│                 │                                                        │
│          Tauri IPC (commands + events)                                    │
│                 │                                                        │
│   WEBVIEW FRONTEND (React, renders in native window)                     │
│   ┌─────────────────────────────┐  ┌──────────────────────────────────┐  │
│   │ Main Window                 │  │ Overlay Window                   │  │
│   │ • Dashboard (analytics)     │  │ • Translucent pill at top of     │  │
│   │ • Settings panel            │  │   screen during breaks           │  │
│   │ • Timer status display      │  │ • Does NOT steal focus           │  │
│   └─────────────────────────────┘  └──────────────────────────────────┘  │
│                                                                          │
│   ┌──────────────┐ ┌──────────────────┐                                  │
│   │  Autostart   │ │  Idle Detection  │                                  │
│   │  (OS-level)  │ │  (pause when AFK)│                                  │
│   └──────────────┘ └──────────────────┘                                  │
└──────────────────────────────────────────────────────────────────────────┘
```

**Data flow pattern:** The Rust backend owns ALL state. The frontend is a pure display/input layer. State changes flow like this:
- User action in frontend → Tauri command (IPC) → Rust mutates state → Rust emits event → Frontend updates
- Timer tick → Rust emits event → Frontend updates
- The frontend NEVER holds authoritative state. It subscribes and reflects.

---

## Project File Structure

This is the canonical file tree. Every feature block references these paths. Create this structure in Block 0.

```
blinky/
├── src-tauri/
│   ├── Cargo.toml                       # Rust dependencies
│   ├── tauri.conf.json                  # Tauri config (windows, bundle, permissions)
│   ├── capabilities/
│   │   └── default.json                 # Tauri v2 capability permissions
│   ├── icons/
│   │   ├── icon.ico                     # Windows app icon
│   │   ├── icon.icns                    # macOS app icon
│   │   ├── icon.png                     # Linux app icon + fallback
│   │   ├── tray-default.png             # Tray: normal state (22x22 + @2x)
│   │   ├── tray-active.png              # Tray: break in progress (22x22 + @2x)
│   │   └── tray-paused.png              # Tray: paused (22x22 + @2x)
│   ├── sounds/
│   │   └── chime.wav                    # Break-complete chime
│   ├── src/
│   │   ├── main.rs                      # Entry: builds app, registers everything
│   │   ├── lib.rs                       # Module declarations
│   │   ├── state.rs                     # Shared types + AppState struct
│   │   ├── tray.rs                      # System tray icon, menu, handlers
│   │   ├── timer.rs                     # Core timer loop + state machine
│   │   ├── db.rs                        # SQLite init, migrations, all CRUD
│   │   ├── analytics.rs                 # Query builders for dashboard data
│   │   ├── commands.rs                  # All #[tauri::command] functions
│   │   ├── settings.rs                  # Settings validation + application
│   │   ├── notifications.rs             # OS notification dispatch
│   │   ├── audio.rs                     # Sound playback
│   │   ├── autostart.rs                 # Launch-at-login
│   │   └── overlay.rs                   # Overlay window positioning/show/hide
│   └── migrations/
│       └── 001_initial.sql              # DB schema
├── src/
│   ├── main.tsx                         # React entry
│   ├── App.tsx                          # Router + layout shell
│   ├── pages/
│   │   ├── Dashboard.tsx                # Analytics page
│   │   └── Settings.tsx                 # Settings page
│   ├── components/
│   │   ├── TimerStatus.tsx              # Live countdown + controls
│   │   ├── StreakCard.tsx               # Current streak display
│   │   ├── DailyChart.tsx               # 7-day bar chart
│   │   ├── WeeklyHeatmap.tsx            # Weekly activity heatmap
│   │   ├── ComplianceRate.tsx           # Donut/percentage display
│   │   └── MiniOverlay.tsx              # Break reminder overlay UI
│   ├── hooks/
│   │   ├── useTimer.ts                  # Subscribe to timer events
│   │   ├── useSettings.ts              # Read/write settings via commands
│   │   └── useAnalytics.ts             # Fetch analytics summary
│   ├── lib/
│   │   ├── commands.ts                  # Typed invoke() wrappers for every command
│   │   └── types.ts                     # TypeScript mirrors of Rust types
│   └── assets/
│       └── styles.css                   # Tailwind directives + custom animations
├── index.html
├── package.json
├── tsconfig.json
├── vite.config.ts
└── tailwind.config.js
```

---

## Sacred Contracts (The Glue Between Blocks)

**These are the shared types, command signatures, and event names that every block must respect. This is how independently-built blocks snap together. Do not deviate from these.**

### Core Rust Types (defined in `state.rs`)

**`TimerPhase` enum:** `Working`, `Breaking`, `Paused`, `Suspended` — these are the only four states the timer can be in.

**`TimerState` struct:**
- `phase: TimerPhase` — current phase
- `seconds_remaining: u64` — countdown within current phase
- `phase_duration: u64` — total seconds in this phase (for progress bars)
- `phase_started_at: u64` — Unix timestamp ms when phase began
- `breaks_completed_today: u32` — running count for the day

**`UserSettings` struct:**
- `work_interval_minutes: u32` (default 20, range 1–120)
- `break_duration_seconds: u32` (default 20, range 5–300)
- `sound_enabled: bool` (default true)
- `sound_volume: f32` (default 0.7, range 0.0–1.0)
- `notification_enabled: bool` (default true)
- `overlay_enabled: bool` (default true)
- `launch_at_login: bool` (default false)
- `daily_goal: u32` (default 24 — roughly 8 hours × 3 per hour)
- `idle_pause_minutes: u32` (default 5, 0 = disabled)
- `theme: String` (default "system", options: "system"/"light"/"dark")

**`BreakRecord` struct:**
- `id: i64` — auto-increment
- `started_at: u64` — Unix timestamp ms
- `duration_seconds: u32` — actual rest time (may be less than configured if skipped early)
- `completed: bool` — user rested the full duration
- `skipped: bool` — user explicitly skipped
- `preceding_work_seconds: u32` — how long the work period was before this break

**`DailyStats` struct:**
- `date: String` — "YYYY-MM-DD"
- `breaks_completed: u32`
- `breaks_skipped: u32`
- `total_rest_seconds: u32`
- `longest_streak: u32` — consecutive completed breaks in a row that day
- `compliance_rate: f64` — completed / (completed + skipped)

**`AnalyticsSummary` struct:**
- `today: DailyStats`
- `last_7_days: Vec<DailyStats>` — exactly 7 entries, oldest first, zero-filled for empty days
- `last_30_days: Vec<DailyStats>` — exactly 30 entries, same rules
- `current_day_streak: u32` — consecutive days where daily goal was met
- `best_day_streak: u32` — all-time record
- `lifetime_breaks: u64`
- `lifetime_rest_seconds: u64`

**`AppState` (managed Tauri state):**
- `timer: Mutex<TimerState>`
- `settings: Mutex<UserSettings>`
- `db_path: String`

The DB connection itself is also managed as a separate `Mutex<Connection>` in Tauri state.

**All of these structs must derive `Serialize` and `Deserialize` (serde) for IPC transport. `TimerState`, `UserSettings`, and all analytics types also need `Clone`.**

**The TypeScript types in `types.ts` MUST be exact mirrors.** Tauri's IPC serializes Rust structs as JSON — any mismatch causes silent runtime failures.

### IPC Commands (Rust ↔ Frontend)

| Command | Args | Returns | Purpose |
|---------|------|---------|---------|
| `get_timer_state` | none | `TimerState` | Poll current timer status |
| `pause_timer` | none | `TimerState` | Pause the countdown |
| `resume_timer` | none | `TimerState` | Resume from pause |
| `skip_break` | none | `TimerState` | Skip current break, return to working |
| `reset_timer` | none | `TimerState` | Reset to full work interval |
| `get_settings` | none | `UserSettings` | Read current settings |
| `update_settings` | `settings: UserSettings` | `UserSettings` | Validate, persist, and apply new settings |
| `get_analytics_summary` | none | `AnalyticsSummary` | Full dashboard data |
| `get_break_history` | `limit: u32, offset: u32` | `Vec<BreakRecord>` | Paginated break log |
| `get_daily_stats_range` | `from: String, to: String` | `Vec<DailyStats>` | Stats for a date range (YYYY-MM-DD) |
| `export_data_csv` | none | `String` (file path) | Export all data, return path to CSV |
| `clear_all_data` | none | `bool` | Nuclear reset — delete everything |

### Events (Backend → Frontend, pushed via Tauri event system)

| Event Name | Payload Type | When It Fires |
|------------|-------------|---------------|
| `timer-tick` | `TimerState` | Every 1 second while Working or Breaking |
| `break-started` | `TimerState` | Work countdown hits 0, break begins |
| `break-completed` | `TimerState` | Break countdown hits 0, chime plays |
| `break-skipped` | `TimerState` | User clicked skip |
| `timer-paused` | `TimerState` | User paused |
| `timer-resumed` | `TimerState` | User resumed |
| `settings-changed` | `UserSettings` | Settings were updated |

The frontend `commands.ts` file should export typed wrapper functions around `invoke()` for every command, and the hooks should use `listen()` for events.

---

## Feature Block 0: Project Scaffolding & Build Pipeline

**Purpose:** Create the project from absolute zero. Install everything. Verify that `cargo tauri dev` opens a window.

**This block MUST be completed first. Every other block assumes this exists.**

### What To Do

1. **Initialize a Tauri v2 project** with the React + TypeScript template. Use `npm create tauri-app@latest`.
2. **Install frontend dependencies:** Tailwind CSS (via `@tailwindcss/vite` plugin), `react-router-dom` for page routing.
3. **Install Rust dependencies** in `Cargo.toml`:
    - `tauri` v2 with features: `tray-icon`, `image-png`
    - `tauri-plugin-notification` v2
    - `tauri-plugin-autostart` v2
    - `tauri-plugin-shell` v2
    - `serde` + `serde_json` (serialization)
    - `rusqlite` with `bundled` feature (embeds SQLite, no system dependency)
    - `rodio` (audio playback)
    - `chrono` with `serde` feature (date/time handling)
    - `dirs` (OS-standard directories)
    - `tokio` with `full` features (async runtime)
4. **Configure `tauri.conf.json`:**
    - Product name: "Blinky", identifier: "com.blinky.app"
    - Main window: 480×640, starts hidden (visible: false), centered, decorated
    - Overlay window: 360×80, starts hidden, NO decorations, transparent, always-on-top, skip-taskbar, focus: false, shadow: false. URL points to `/overlay` route.
    - Bundle config for all platforms with icon paths
5. **Set up Tauri v2 capabilities** in `capabilities/default.json` — grant permissions for: core defaults, window management (show/hide/close/focus/center), notifications, autostart, shell, events, tray.
6. **Configure Tailwind** in `vite.config.ts` using the Vite plugin approach.
7. **Create ALL placeholder files** from the file tree above. Every `.rs` file can just declare `// TODO` and every `.tsx` file can export an empty component. The point is that the module structure exists and compiles.
8. **Wire up `lib.rs`** to declare all modules. Wire up `main.rs` with a minimal Tauri builder that registers the notification and autostart plugins, and has an empty `.setup()` closure.
9. **Create `types.ts`** with all TypeScript type definitions mirroring the Rust types.
10. **Create `commands.ts`** with typed `invoke()` wrappers for every command (they'll fail at runtime until the Rust side is implemented, but the types are in place).

### Acceptance Criteria

- [ ] `npm run tauri dev` launches without errors
- [ ] A native window appears (content can be the default Tauri template)
- [ ] `cargo check` in `src-tauri/` succeeds with zero warnings about missing modules
- [ ] `npx tsc --noEmit` succeeds (TypeScript compilation clean)
- [ ] All files in the file tree exist (even if stubs)
- [ ] The overlay window is defined in `tauri.conf.json` (won't show yet, but configured)

---

## Feature Block 1: SQLite Database Layer

**Purpose:** Blinky needs to remember things across app restarts — break history, user settings, cached analytics. This block creates the entire persistence layer that every data-touching feature depends on.

**Depends on:** Block 0
**Depended on by:** Block 3 (writes break records), Block 5 (reads analytics), Block 6 (reads/writes settings)

### Database Location

Use the `dirs` crate to find the OS-standard application data directory. Create a `com.blinky.app` subdirectory. Store the DB file as `blinky.db` inside it. This means:
- macOS: `~/Library/Application Support/com.blinky.app/blinky.db`
- Windows: `C:\Users\<user>\AppData\Roaming\com.blinky.app\blinky.db`
- Linux: `~/.local/share/com.blinky.app/blinky.db`

Create the directory if it doesn't exist. This DB persists across app updates.

### Schema Design

Three tables + one metadata table:

**`settings` table** — Single-row pattern. The table has an `id` column with a CHECK constraint enforcing `id = 1`, so there's always exactly one row. Every user setting is its own typed column (not a JSON blob — we want SQL defaults and type safety). Seed the row with defaults on creation using `INSERT OR IGNORE`.

**`break_records` table** — One row per reminder event, regardless of outcome. Columns match the `BreakRecord` struct. Index on `started_at` for date-range queries. This is the source of truth for all analytics.

**`daily_stats_cache` table** — Denormalized daily aggregates, keyed by date string (YYYY-MM-DD). This is a *performance optimization* — the analytics engine could recompute everything from `break_records`, but caching avoids scanning thousands of rows for the dashboard. This table can be rebuilt from `break_records` at any time, so it's safe to drop.

**`_migrations` table** — Tracks which migration scripts have been applied. Simple `id + name + applied_at` pattern. Check this table before running each migration so they're idempotent.

### Implementation Requirements

**Initialization function (`init_db`):** Opens the DB, enables WAL journal mode (critical — WAL lets the timer write while the dashboard reads without blocking), enables foreign keys, runs pending migrations, returns the connection wrapped in a Mutex.

**Why WAL mode?** Blinky writes to the DB every ~20 minutes (break records) while the dashboard may read analytics at any time. Without WAL, reads and writes would block each other. WAL = Write-Ahead Logging — readers see a consistent snapshot while writers append to a log. This is a single pragma call but it matters.

**CRUD functions to implement:**
- `insert_break_record(...)` → returns the new row ID
- `update_break_completion(id, duration_seconds, completed)` → called when a break ends to finalize the record
- `get_break_records(limit, offset)` → paginated, newest first
- `load_settings()` → returns `UserSettings` from the single row
- `save_settings(settings)` → updates the single row
- `recompute_daily_stats(date)` → reads `break_records` for that date, computes aggregates, upserts into `daily_stats_cache`, returns `DailyStats`
- `get_daily_stats_range(from, to)` → reads from cache, always recomputes today's entry (it's in flux)
- `clear_all_data()` → deletes all break records, clears cache, resets settings to defaults
- `export_to_csv()` → writes all break records as CSV to the user's Downloads folder, returns the file path

**Edge case: date boundaries.** Break records store timestamps in Unix milliseconds. To query "all breaks on 2025-01-15," you need to convert "2025-01-15" to a start-of-day and end-of-day timestamp range. Use UTC consistently. Don't use the local timezone for storage — only for display.

### Testing

Write unit tests using an in-memory SQLite connection (`:memory:`). Test:
- Default settings load correctly after migration
- Save + load settings round-trips every field
- Insert + query break records works with pagination
- Daily stats computation is correct for mixed completed/skipped breaks
- Longest streak computation handles edge cases (all completed, all skipped, alternating)
- Clear all data actually resets everything
- Compliance rate handles the zero-division case (no breaks)

### Acceptance Criteria

- [ ] `cargo test` passes all DB tests
- [ ] DB file is created in the correct OS-specific location on first launch
- [ ] Settings default to the documented values
- [ ] After inserting 10 break records, `get_break_records(5, 0)` returns the 5 most recent
- [ ] `recompute_daily_stats` correctly calculates compliance rate and longest streak
- [ ] `clear_all_data` followed by `load_settings` returns defaults
- [ ] `export_to_csv` creates a valid CSV file in the Downloads directory

---

## Feature Block 2: System Tray

**Purpose:** Blinky is a tray-resident app. The tray icon is the primary interaction point — it shows status at a glance and provides quick actions. Most users will interact with Blinky exclusively through the tray.

**Depends on:** Block 0
**Depended on by:** Block 3 (timer calls `update_tray_status`), Block 10 (integration)

### Tray Icon Design

Three 22×22 PNG icons (also provide @2x at 44×44 for Retina/HiDPI):
- **`tray-default.png`** — A simple stylized eye icon in a neutral color (muted gray or blue). This is what users see 95% of the time.
- **`tray-active.png`** — Same eye icon but in green/glowing. Signals "break is happening right now."
- **`tray-paused.png`** — Eye icon with a small pause indicator, in gray. Signals "timer is paused."

Design constraints: must be recognizable at tiny sizes, should work on both light and dark OS taskbars, transparent background. On macOS, the system may auto-template tray icons (convert to monochrome) — test both modes.

For an MVP, you can generate simple programmatic icons or use basic emoji-to-PNG conversion. Polish later.

### Tray Menu Structure

```
┌─────────────────────────────┐
│ Next break in 18:32         │  ← Status line (disabled/non-clickable)
│─────────────────────────────│
│ Pause                       │  ← Toggles to "Resume" when paused
│ Skip Break                  │  ← Only meaningful during breaks, but always visible
│ Reset Timer                 │
│─────────────────────────────│
│ Open Dashboard              │  ← Shows/focuses main window on Dashboard page
│ Settings                    │  ← Shows/focuses main window on Settings page
│─────────────────────────────│
│ Quit Blinky                 │
└─────────────────────────────┘
```

### Tray Behaviors

**Left-click on tray icon:** Toggle main window visibility. If hidden → show and focus. If visible → hide.

**Tooltip:** Always shows current status. Updated every second by the timer engine:
- Working: "Blinky — Next break in MM:SS"
- Breaking: "Blinky — Look away! Xs remaining"
- Paused: "Blinky — Paused"
- Suspended: "Blinky — Suspended (idle)"

**Menu actions:** The tray module should NOT directly mutate timer state. Instead, it should emit events or call into the timer module's public API. This keeps the tray as a thin input layer.

**Dynamic menu text:** The "Pause" item should read "Resume" when the timer is paused. The status line should update with the current countdown. (Note: Tauri v2's menu system has some limitations on dynamic text — research the best approach. You may need to rebuild the menu on each update or use `set_text` on individual items.)

### The `update_tray_status` Function

This is the function the timer engine calls on every tick. It receives the current `TimerPhase` and `seconds_remaining`. It must:
1. Update the tooltip text
2. Swap the tray icon to match the phase
3. Update the status menu item text
4. Toggle "Pause"/"Resume" label

This function must be cheap — it's called every second. Don't do any I/O or heavy work inside it.

### Acceptance Criteria

- [ ] App launches with a visible tray icon (correct for the OS)
- [ ] Left-clicking the icon toggles the main window
- [ ] Right-clicking (or left-clicking on macOS, depending on config) shows the context menu
- [ ] "Quit Blinky" actually exits the process
- [ ] "Open Dashboard" shows and focuses the main window
- [ ] The tooltip text reflects the current timer state (manually test by checking tooltip)
- [ ] The icon changes when the phase changes (test by triggering a break)

---

## Feature Block 3: Timer Engine

**Purpose:** This is the brain of Blinky. An async background loop that ticks every second, manages the state machine, and orchestrates everything that happens when phases change.

**Depends on:** Block 0, Block 1 (DB writes)
**Depended on by:** Everything — this is the core

### State Machine

```
          ┌──────────────────────────────────────────────┐
          │                                              │
          ▼                                              │
    ┌──────────┐    seconds_remaining    ┌──────────┐   │
    │ WORKING  │───── hits 0 ──────────►│ BREAKING │   │
    │          │                         │          │   │
    │ counting │                         │ counting │   │
    │ down from│    ┌── user skips ──────│ down from│   │
    │ work     │    │                    │ break    │   │
    │ interval │    │                    │ duration │   │
    └──┬───┬───┘    │                    └────┬─────┘   │
       │   │        │                         │         │
       │   │        │    seconds_remaining     │         │
       │   │        │    hits 0 ──────────────┘─────────┘
       │   │        │    (break complete → chime → log → back to Working)
       │   │        │
       │   │        ▼
       │   │   log as skipped, back to Working
       │   │
  user │   │ user
pauses │   │ resumes
       │   │
       ▼   │
    ┌──────────┐
    │ PAUSED   │    (time frozen, seconds_remaining preserved)
    └──────────┘

    Any phase can also transition to SUSPENDED (idle detection),
    which behaves like Paused but is triggered automatically.
```

### Transition Details

**Working → Breaking:**
- Fire `break-started` event
- Send OS notification (if enabled in settings)
- Show overlay window (if enabled in settings)
- Insert a new `break_record` in the DB with `completed: false, skipped: false, duration_seconds: 0`
- Remember the record ID so we can update it when the break ends
- Switch tray icon to active

**Breaking → Working (break complete):**
- Play the chime sound (if enabled)
- Update the break record: `completed: true, duration_seconds: actual_break_duration`
- Increment `breaks_completed_today`
- Fire `break-completed` event
- Hide overlay window
- Recompute today's daily stats cache
- Switch tray icon to default
- Reset `seconds_remaining` to full work interval

**Breaking → Working (skip):**
- Update the break record: `skipped: true, duration_seconds: actual_seconds_elapsed`
- Fire `break-skipped` event
- Hide overlay
- Switch tray icon to default
- Reset work timer

**Any → Paused:**
- Freeze `seconds_remaining` at current value
- Fire `timer-paused` event
- Switch tray icon to paused

**Paused → Working/Breaking:**
- Resume with the preserved `seconds_remaining`
- Fire `timer-resumed` event
- Restore the appropriate tray icon

### Critical: Timer Accuracy

**Do NOT rely on counting `sleep(1 second)` calls.** Each loop iteration takes slightly more than 1 second due to lock acquisition, event emission, DB writes, etc. Over 20 minutes, this drift accumulates to noticeable inaccuracy.

**Solution:** Store `phase_started_at` as a wall-clock timestamp. On each tick, compute: `seconds_remaining = phase_duration - ((now - phase_started_at) / 1000)`. This way, even if a tick is delayed (system sleep, heavy load), the timer self-corrects.

This also handles the case where the laptop is closed during a work period — when it wakes up, the timer will see that the full work interval has elapsed and immediately trigger a break.

### The Timer Loop Structure

The timer runs as a `tauri::async_runtime::spawn` task started during app setup. It loops forever:

1. Read current state (lock the Mutex, clone what's needed, drop the lock)
2. Read relevant settings (lock, clone, drop)
3. Match on the current phase, handle transitions
4. Emit `timer-tick` event with current `TimerState`
5. Call `update_tray_status()`
6. `tokio::time::sleep(Duration::from_secs(1))`
7. Go to 1

**Lock discipline:** Hold Mutex locks for the minimum possible time. Lock, read/write, unlock. Never hold a lock across an await point or an I/O operation.

### Public API for Commands

The timer module exposes these functions that `commands.rs` calls:
- `pause(app_handle)` → `TimerState`
- `resume(app_handle)` → `TimerState`
- `skip_break(app_handle)` → `TimerState`
- `reset(app_handle)` → `TimerState`

Each function mutates the `AppState`, emits appropriate events, and returns the new state.

### Edge Cases to Handle

- **Settings change during a work period:** If the user changes `work_interval_minutes` from 20 to 10 while there's 15 minutes remaining, what happens? **Answer:** Don't retroactively change the current period. Apply the new interval on the NEXT cycle. But DO apply `break_duration_seconds` changes immediately if a break starts.
- **Settings change during a break:** If break duration changes mid-break, use the original duration for this break.
- **App starts up, previous session had breaks today:** Query the DB on startup to initialize `breaks_completed_today` correctly.
- **System time changes:** If the user changes their system clock, the wall-clock approach might cause the timer to fire immediately or wait too long. This is an acceptable edge case — don't over-engineer for it.
- **Multiple skips rapidly:** Debounce. If `skip_break` is called when not in `Breaking` phase, it's a no-op.

### Acceptance Criteria

- [ ] Timer starts counting down from `work_interval_minutes * 60` on app launch
- [ ] At 0, transitions to `Breaking` phase (observable via events or tray tooltip)
- [ ] Break counts down for `break_duration_seconds`
- [ ] At 0, transitions back to `Working` (observable via events or tray tooltip)
- [ ] `pause_timer` freezes the countdown; `resume_timer` continues from where it was
- [ ] `skip_break` during a break immediately returns to Working
- [ ] `reset_timer` restarts the work interval from full duration
- [ ] Timer does not drift by more than 2 seconds over a 20-minute period (test with short intervals)
- [ ] `breaks_completed_today` increments on break completion, not on skip
- [ ] A break record is written to the DB with correct `completed`/`skipped` flags

---

## Feature Block 4: Notifications & Audio

**Purpose:** Two sensory feedback channels. A notification nudges the user when a break starts. A chime tells them it's over (without them needing to watch the screen).

**Depends on:** Block 0
**Depended on by:** Block 3 (timer calls both)

### OS Notifications

Use `tauri-plugin-notification`. The notification for a break starting should have:
- **Title:** Something friendly and brief, like "Time for a break! 👀"
- **Body:** "Look at something 20 feet away for 20 seconds."

The break-complete notification (optional, secondary to the chime):
- **Title:** "Break complete ✓"
- **Body:** "Nice work! Your eyes will thank you."

**Non-intrusiveness guarantee:** OS notifications are inherently non-intrusive on all three platforms — they appear in a notification area, don't steal focus, and auto-dismiss. This is exactly what we want.

**Permission handling:** On macOS, the app needs notification permission. Tauri's plugin handles the request flow, but you should handle the case where the user denied permission gracefully (just skip sending, don't crash or nag).

### Audio Playback

Use the `rodio` crate. The chime sound file (`chime.wav`) should be:
- Format: WAV, 44.1kHz, 16-bit, mono
- Duration: 0.5–1.0 seconds
- Character: Soft and pleasant. A single gentle bell, marimba note, or soft ding. NOT a harsh alarm, beep, or buzzer. Think "meditation app" not "alarm clock."
- Normalized to -6dB to leave headroom for volume adjustment.

**Embed the file at compile time** using `include_bytes!()`. This avoids file-not-found issues in bundled apps where the working directory isn't what you'd expect.

**Spawn audio on a separate thread.** The rodio `OutputStream` blocks while playing, and you absolutely cannot block the timer loop. `std::thread::spawn` a short-lived thread that creates the stream, plays the sound, and exits.

**Volume control:** The `sound_volume` setting (0.0–1.0) should be applied to the rodio `Sink` before appending the audio source.

**Sound sourcing for MVP:** You can generate a simple chime programmatically (a sine wave at ~880Hz with exponential decay, rendered to WAV), download a CC0-licensed chime from freesound.org, or create one in Audacity. The file just needs to exist and sound nice.

### Acceptance Criteria

- [ ] When a break starts, an OS notification appears
- [ ] The notification does NOT bring any window to the foreground
- [ ] When a break completes, a chime sound plays
- [ ] The chime volume changes when `sound_volume` setting is adjusted
- [ ] Setting `sound_enabled: false` prevents the chime from playing
- [ ] Setting `notification_enabled: false` prevents the notification from firing
- [ ] Audio playback does not block or delay the timer loop

---

## Feature Block 5: Analytics Engine

**Purpose:** Transform raw break records into the `AnalyticsSummary` that powers the dashboard. This is pure computation — reads from DB, returns a struct.

**Depends on:** Block 1 (database)
**Depended on by:** Block 6 (the `get_analytics_summary` command calls this)

### What `build_analytics_summary` Must Do

This is a single function that the command layer calls. It returns `AnalyticsSummary`. Here's the logic:

1. **Today's stats:** Always recompute from `break_records` (today is still in progress, cache may be stale). Write result to `daily_stats_cache`.

2. **Last 7 days:** Query `daily_stats_cache` for the date range [today - 6 days, today]. **Fill in missing days with zero-value `DailyStats`.** The frontend expects exactly 7 entries. If the user didn't use Blinky on Wednesday, Wednesday still appears in the array with all zeros.

3. **Last 30 days:** Same as above but 30 entries.

4. **Current day streak:** Walk backwards from today through `daily_stats_cache`. Count consecutive days where `breaks_completed >= daily_goal`. Stop at the first day that doesn't meet the goal. Special case: if today hasn't met the goal *yet*, don't break the streak (the day isn't over). If today has zero breaks but it's 9 AM, that shouldn't reset a 15-day streak.

5. **Best day streak:** Scan all `daily_stats_cache` entries to find the longest run of consecutive days meeting the goal. This can use a simple linear scan.

6. **Lifetime totals:** `SELECT COUNT(*), SUM(duration_seconds) FROM break_records WHERE completed = 1`. Handle the case where the table is empty (return 0, 0).

### The Daily Stats Recomputation Logic

`recompute_daily_stats(date_string)`:
1. Convert the date string to a Unix timestamp range for that day (start of day to end of day, UTC)
2. Query all `break_records` within that range
3. Count completed, count skipped, sum duration_seconds
4. Compute longest streak: walk through the day's breaks in order, count consecutive `completed && !skipped` runs
5. Compute compliance rate: `completed / (completed + skipped)`, handle divide-by-zero → 0.0
6. Upsert into `daily_stats_cache`

### Edge Cases

- Brand new install: zero break records → all stats are zero, 7-day and 30-day arrays are all zero-filled entries
- User hasn't opened the app in a week: the missing days should appear as zeros in the 7-day view, not be omitted
- Midnight boundary: a break that starts at 11:59 PM and ends at 12:00 AM — assign it to the day it started
- Streak calculation across DST changes: use UTC everywhere to avoid ambiguity

### Acceptance Criteria

- [ ] With zero data, `build_analytics_summary` returns a valid struct with all zeros
- [ ] `last_7_days` always has exactly 7 elements
- [ ] `last_30_days` always has exactly 30 elements
- [ ] After 3 completed breaks and 1 skipped break today, `today.compliance_rate ≈ 0.75`
- [ ] Streak computation correctly handles gaps (day with zero breaks resets streak)
- [ ] Lifetime totals only count completed breaks, not skipped ones
- [ ] The function completes in <50ms even with 1000+ break records (test with synthetic data)

---

## Feature Block 6: IPC Commands & Settings Management

**Purpose:** Bridge the Rust backend and React frontend. Every frontend interaction goes through these commands. Also handles settings validation and application.

**Depends on:** Block 0, Block 1
**Depended on by:** Block 8 (frontend calls these), Block 10 (registered in main.rs)

### Command Implementations

Implement every command listed in the Sacred Contracts section. Each one is a `#[tauri::command]` function. Most are thin wrappers:
- `get_timer_state` → lock Mutex, clone, return
- `pause_timer` / `resume_timer` / `skip_break` / `reset_timer` → call the corresponding function in `timer.rs`
- `get_settings` → lock Mutex, clone, return
- `update_settings` → validate, save to DB, update in-memory state, emit `settings-changed` event
- `get_analytics_summary` → call `analytics::build_analytics_summary`
- `get_break_history` → call `db::get_break_records`
- `get_daily_stats_range` → call `db::get_daily_stats_range`
- `export_data_csv` → call `db::export_to_csv`
- `clear_all_data` → call `db::clear_all_data`, reset in-memory settings

### Settings Validation

`update_settings` MUST validate before persisting. Reject with a descriptive error string if:
- `work_interval_minutes` is outside 1–120
- `break_duration_seconds` is outside 5–300
- `sound_volume` is outside 0.0–1.0
- `daily_goal` is outside 1–100
- `theme` is not one of "system", "light", "dark"

### Settings Side Effects

When certain settings change, additional actions are needed:
- `launch_at_login` changes → call `autostart::set_autostart(app, new_value)`
- `work_interval_minutes` changes → the timer should use the new interval for the NEXT work cycle (not the current one)
- `theme` changes → emit a `settings-changed` event so the frontend can update its theme

### Registration in main.rs

All commands must be registered in `tauri::generate_handler![]`. The DB connection and `AppState` must be `.manage()`d so commands can access them via `State<>`.

### Acceptance Criteria

- [ ] Every command in the contract table is callable from the frontend
- [ ] `get_timer_state` returns a valid `TimerState`
- [ ] `update_settings` with valid data persists and returns the new settings
- [ ] `update_settings` with `work_interval_minutes: 0` returns an error string
- [ ] `get_analytics_summary` returns data that matches what's in the DB
- [ ] `export_data_csv` creates a file and returns its path
- [ ] `clear_all_data` resets everything and subsequent `get_settings` returns defaults
- [ ] Settings survive app restart (close and reopen → settings are the same)

---

## Feature Block 7: Non-Intrusive Overlay Window

**Purpose:** The signature UX element. A small, translucent pill that slides down from the top of the screen during breaks. It's the thing that makes Blinky feel *gentle* rather than *nagging*.

**Depends on:** Block 0
**Depended on by:** Block 3 (timer shows/hides it), Block 8 (frontend renders it)

### The Non-Intrusiveness Contract

This is the most important UX requirement in the entire app. Every design decision in this block serves one goal: **the overlay must not interrupt the user's workflow.** Here's how:

| Requirement | How To Achieve It |
|-------------|-------------------|
| Doesn't steal focus | Tauri window config: `focus: false`. NEVER call `set_focus()` on the overlay. |
| Doesn't block clicks on other apps | The outer container uses `pointer-events: none`. Only the tiny "skip" button is clickable. |
| Doesn't obscure important content | It's small (360×80px), positioned at the top center of the screen, and translucent. |
| Auto-dismisses | Hides when break timer reaches 0 or when user skips. No manual dismissal needed. |
| Smooth appearance | Fades in with a slide-down animation. No jarring pop. |
| Doesn't appear in the taskbar/dock | Tauri window config: `skipTaskbar: true`. |

### Overlay Window (Tauri Side)

The overlay is a second Tauri window, configured in `tauri.conf.json`:
- Decorations: false (no title bar)
- Transparent: true (the background is see-through)
- Always on top: true (floats above all other windows)
- Focus: false (does NOT steal focus when shown)
- Resizable: false
- URL: points to `/overlay` route in the React app

`overlay.rs` exposes two functions:
- `show_overlay(app_handle)` — positions the overlay at top-center of the primary monitor (accounting for macOS menu bar height), then calls `window.show()` WITHOUT `set_focus()`
- `hide_overlay(app_handle)` — calls `window.hide()`

### Overlay UI (React Side)

The `/overlay` route renders the `MiniOverlay` component. This component:
- Listens to `timer-tick` events to get `seconds_remaining`
- Shows when `phase == "Breaking"`, hides otherwise
- Displays:
    - An eye emoji or icon
    - "Look away — rest your eyes" text
    - A large countdown number (e.g., "14s")
    - A small circular progress indicator (SVG ring that depletes as time passes)
    - A subtle "skip" text button
- Uses a translucent dark background (e.g., `bg-gray-900/80` with `backdrop-blur`)
- Has a slide-down-and-fade-in entrance animation
- The skip button calls the `skipBreak` command

### Design Details

- The overlay should look like a floating pill/capsule shape — rounded corners (large border-radius), not a rectangular box
- Text should be white on the dark translucent background
- The countdown number should use tabular/monospace figures so it doesn't jump around as digits change
- The progress ring provides visual feedback without requiring the user to read the number
- The "skip" button should be de-emphasized (small, low-contrast) — we want users to take the break, not skip it. But it must exist for when they're in a meeting or critical moment.

### Positioning

The overlay should be centered horizontally on the primary monitor, near the top. On macOS, account for the ~24px menu bar. On Windows and Linux, position at y=8 or similar small offset from the top edge.

**Don't position it at y=0** — on some systems, this can interfere with window manager hot corners or be partially hidden under panels.

### Acceptance Criteria

- [ ] During a break, the overlay appears at the top center of the screen
- [ ] The overlay does NOT steal focus — you can keep typing in your editor without clicking away
- [ ] The countdown matches the timer's `seconds_remaining`
- [ ] The progress ring animates smoothly
- [ ] Clicking "skip" hides the overlay and resets the timer
- [ ] When the break completes naturally, the overlay auto-hides
- [ ] The overlay has a translucent background (you can see your desktop through it)
- [ ] The entrance animation is smooth (no jarring pop-in)
- [ ] The overlay does NOT appear in the taskbar/dock/alt-tab list

---

## Feature Block 8: Frontend UI (Dashboard + Settings)

**Purpose:** The main window with two pages — an analytics dashboard showing eye-rest habits, and a settings panel. This is what users see when they click the tray icon.

**Depends on:** Block 0 (scaffolding), Block 6 (commands)
**Can be developed against mock data** if the backend isn't ready yet.

### App Shell

The main window has a simple top nav bar with the Blinky name/logo and two nav links: Dashboard and Settings. Use `react-router-dom` for routing. The nav should highlight the active page.

The shell should respect the theme setting (light/dark/system). Use Tailwind's dark mode with the `class` strategy, and toggle the `dark` class on the root element based on the setting. For "system", listen to `prefers-color-scheme` media query.

### Dashboard Page

The dashboard is the default/home page. It shows, from top to bottom:

1. **Timer Status (hero section):** The most prominent element. Shows:
    - Current phase in plain language ("Next break in..." / "Look away!" / "Paused")
    - Big countdown numbers (MM:SS for working, Xs for breaking)
    - A thin progress bar showing how far through the current phase
    - Pause/Resume button
    - Skip button (only during breaks)
    - "X breaks completed today" text below

2. **Key Metrics (2-column grid):**
    - **Streak card:** Shows current day streak (🔥 3-day streak!), best streak, and today's progress toward the daily goal (e.g., "8 of 24 breaks")
    - **Compliance rate:** Today's completion percentage as a large number, with completed vs skipped counts underneath

3. **7-Day Bar Chart:** A simple bar chart showing breaks per day for the last 7 days. Each bar has two segments — green for completed, orange for skipped. Day names (Mon, Tue, etc.) on the x-axis.

4. **Weekly Heatmap:** A 7-column grid (one per day) where each cell's color intensity represents how many breaks were taken. Light = few, dark/vibrant = many. Think GitHub contribution graph but for one week.

5. **Lifetime Stats:** A subtle footer line: "1,247 lifetime breaks · 415 minutes of rest"

### Settings Page

A clean form with sections:

**Timer section:**
- Work interval slider (5–60 min, step 5, shows current value)
- Break duration slider (10–120 sec, step 5, shows current value)
- Daily break goal number input

**Notifications section:**
- System notifications toggle
- Overlay reminder toggle
- Completion sound toggle
- Volume slider (only visible when sound is enabled)

**System section:**
- Launch at login toggle
- Auto-pause when idle slider (0–15 min, 0 = off)
- Theme selector (system/light/dark dropdown)

**Danger zone (at the bottom, separated):**
- "Export data as CSV" button
- "Clear all data" button (with confirmation dialog)

**Settings should auto-save** — no "Save" button. Each change immediately calls `update_settings`. Use a small "Saving..." indicator to give feedback. Debounce rapid changes (e.g., dragging a slider).

### React Hooks

**`useTimer`:** Fetches initial `TimerState` via command on mount, then subscribes to `timer-tick` events for real-time updates. Returns `TimerState | null`.

**`useAnalytics`:** Fetches `AnalyticsSummary` via command on mount and after `break-completed` events. Returns `{ data, loading, error, refresh }`.

**`useSettings`:** Fetches settings via command on mount. Returns settings + a save function that calls `update_settings` and handles errors.

### Charting Approach

For the bar chart and heatmap, **do NOT pull in a heavy charting library** like Chart.js or Recharts. The visualizations are simple enough to build with plain HTML/CSS (div heights for bars, background colors for heatmap cells) or simple SVGs. Keep the bundle small.

### Visual Design Direction

Blinky should feel calm and minimal — it's a health/wellness utility, not a productivity dashboard.
- Lots of whitespace
- Rounded corners on cards (border-radius: 16px style)
- Muted colors — soft greens for completed, soft oranges for skipped, soft blues for accents
- No aggressive gradients or shadows
- The timer countdown should be the visual anchor — large and prominent
- Typography: system font stack, use size/weight contrast instead of multiple font families

### Acceptance Criteria

- [ ] Main window opens to the Dashboard page
- [ ] Navigation between Dashboard and Settings works
- [ ] Timer countdown updates every second in real-time
- [ ] Pause/Resume/Skip buttons work and the UI reflects the state change immediately
- [ ] Analytics charts render correctly with real data (and gracefully with zero data)
- [ ] Settings changes are reflected immediately (no page reload needed)
- [ ] Theme switching works (light/dark/system)
- [ ] "Export CSV" triggers a file save
- [ ] "Clear all data" resets analytics to zero after confirmation
- [ ] The app looks good at the default 480×640 window size
- [ ] The app is usable (not broken) if the window is resized

---

## Feature Block 9: Autostart & Idle Detection

**Purpose:** Quality-of-life system integrations. Launch-at-login so users don't forget to start Blinky. Idle detection so the timer pauses when you walk away (you don't need a "rest your eyes" reminder if you're not looking at a screen).

**Depends on:** Block 0
**Depended on by:** Block 6 (settings change triggers autostart toggle)

### Launch at Login

Use `tauri-plugin-autostart`. It handles the OS-specific mechanics:
- macOS: LaunchAgent plist
- Windows: Registry key in `HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Run`
- Linux: `.desktop` file in `~/.config/autostart/`

Expose two functions: `set_autostart(app, enabled: bool)` and `is_autostart_enabled(app) -> bool`. The settings command calls `set_autostart` whenever `launch_at_login` changes.

### Idle Detection (Lower Priority — Can Be Deferred)

**Concept:** Check system idle time (seconds since last keyboard/mouse input) every 30 seconds. If idle time exceeds `idle_pause_minutes * 60`, transition the timer to `Suspended`. When activity resumes (idle time drops below threshold), transition back to `Working` with a fresh interval.

**Why this matters:** If you go to lunch for an hour, you don't want to come back to find that Blinky fired 3 reminders while you weren't there, and your "compliance rate" dropped because you "skipped" them. Idle detection pauses the cycle, preserving accurate analytics.

**Platform-specific APIs:**
- macOS: `CGEventSourceSecondsSinceLastEventType` from CoreGraphics
- Windows: `GetLastInputInfo` from `winuser.h`
- Linux: `XScreenSaverQueryInfo` from X11, or D-Bus `org.freedesktop.ScreenSaver`

**This requires native FFI and is the most platform-specific code in the app.** For an MVP, it's acceptable to ship with `idle_pause_minutes` defaulting to 0 (disabled) and implement this later. The timer state machine already has the `Suspended` phase — the only missing piece is the detection trigger.

**If implementing:** Create a periodic check (every 30 seconds) inside the timer loop or as a separate spawned task. When idle time exceeds the threshold and the timer is in `Working` phase, set phase to `Suspended`. When idle time drops below the threshold and phase is `Suspended`, set phase to `Working` with a fresh interval.

### Acceptance Criteria

- [ ] Toggling `launch_at_login: true` in settings causes the app to launch on next OS restart
- [ ] Toggling `launch_at_login: false` removes the autostart entry
- [ ] `is_autostart_enabled` reflects the actual OS-level state
- [ ] (If idle detection is implemented) Leaving the computer idle for longer than the threshold pauses the timer
- [ ] (If idle detection is implemented) Returning to the computer resumes the timer with a fresh work interval

---

## Feature Block 10: Integration & Assembly

**Purpose:** Wire all the blocks together. This is the final pass where you ensure every integration point connects, the app launches cleanly, and the full user experience works end-to-end.

**Depends on:** All other blocks (or at least their interfaces)

### Integration Checklist

Go through each of these and verify the connection exists and works:

| Connection | What to verify |
|------------|---------------|
| `main.rs` manages `AppState` | `AppState` struct is created with correct defaults, `.manage()`d |
| `main.rs` manages DB connection | `init_db()` called, connection `.manage()`d |
| `main.rs` registers all commands | Every command from the contract table is in `generate_handler![]` |
| `main.rs` registers plugins | `notification` and `autostart` plugins both registered |
| `main.rs` calls `create_tray()` in setup | Tray appears on launch |
| `main.rs` calls `start_timer_loop()` in setup | Timer begins counting on launch |
| Timer → DB | `insert_break_record` called on break start, `update_break_completion` on break end |
| Timer → Notifications | `send_break_notification` called when entering `Breaking` phase |
| Timer → Audio | `play_break_complete_sound` called when break timer reaches 0 |
| Timer → Tray | `update_tray_status` called every tick |
| Timer → Overlay | `show_overlay` called on break start, `hide_overlay` on break end/skip |
| Timer → Events | All 6 event types fire at the correct times |
| Settings → Autostart | Changing `launch_at_login` calls `set_autostart` |
| Settings → DB | `save_settings` called in `update_settings` command |
| Frontend → Commands | Every `invoke()` wrapper calls the correct Rust command |
| Frontend → Events | `useTimer` hook receives `timer-tick` events |
| Frontend → Events | `useAnalytics` hook refreshes on `break-completed` |
| Overlay → Events | `MiniOverlay` listens to `timer-tick` and `break-completed` |

### End-to-End Test Scenarios

Run each of these manually. They exercise the full stack.

| Scenario | Steps | Expected Result |
|----------|-------|-----------------|
| **Fresh install** | Launch for the first time | DB created, defaults loaded, timer at 20:00, tray icon visible |
| **Full break cycle** | Set work interval to 1 min (for testing). Wait 60s. | Notification fires, overlay appears, 20s countdown, chime plays, overlay hides, timer resets to 1:00 |
| **Skip break** | Trigger a break, click "skip" in overlay or dashboard | Break logged as skipped, timer resets, overlay hides |
| **Pause mid-work** | Click pause at 12:30 remaining | Timer freezes at 12:30, tray shows "Paused" |
| **Resume** | Click resume | Timer continues from 12:30, counting down |
| **Settings persist** | Change interval to 15 min, quit, relaunch | Timer starts at 15:00, settings page shows 15 min |
| **Analytics accuracy** | Complete 5 breaks, skip 2, open dashboard | Shows 5 completed, 2 skipped, compliance rate ≈ 71.4% |
| **Tray icon changes** | Observe during work → break → pause | Icon transitions: default → active → paused |
| **Overlay non-intrusion** | Have a text editor focused, break triggers | Overlay appears but text editor retains focus; you can keep typing |
| **Export CSV** | Click "Export data as CSV" in settings | File appears in Downloads, contents match DB |
| **Clear data** | Click "Clear all data", confirm | Dashboard shows all zeros, settings reset |
| **Window toggle** | Click tray icon repeatedly | Window shows/hides alternately |
| **Dark mode** | Set theme to "dark" | Entire UI switches to dark colors |
| **Empty state** | Clear all data, open dashboard | Charts show zero states gracefully (no errors, no blank page) |

### Build & Distribution

```bash
# Development
npm run tauri dev

# Production build
npm run tauri build
```

Verify the production build produces:
- macOS: `.dmg` installer
- Windows: NSIS `.exe` installer
- Linux: `.AppImage` and/or `.deb`

Test the production build separately from the dev build — some issues (like asset paths, sound file embedding, DB location) only surface in production.

### Performance Checklist

- [ ] Memory usage <30MB during normal operation (check Activity Monitor / Task Manager)
- [ ] CPU usage ~0% when the timer is just ticking (no busy loops)
- [ ] App launches in <2 seconds
- [ ] Main window opens in <500ms when clicking the tray icon
- [ ] No visible lag when switching between Dashboard and Settings pages
- [ ] Analytics summary computes in <100ms with 1000+ break records

---

## Parallelization Map

```
                       ┌──────────┐
                       │ Block 0  │  MUST BE FIRST
                       │ Scaffold │
                       └────┬─────┘
                            │
        ┌───────┬───────┬───┼───────┬───────┬────────┐
        │       │       │   │       │       │        │
   ┌────▼──┐ ┌─▼────┐ ┌▼───▼─┐ ┌──▼───┐ ┌─▼─────┐ ┌▼──────┐
   │Blk 1  │ │Blk 2 │ │Blk 4 │ │Blk 7 │ │Blk 8  │ │Blk 9  │
   │  DB   │ │ Tray │ │Notif │ │Overlayⁿ│ │ UI   │ │ Auto  │
   └──┬────┘ └──────┘ │Audio │ └───────┘ └───────┘ │ start │
      │               └──────┘                      └───────┘
      │
   ┌──┼──────────┐
   │  │          │
┌──▼──▼─┐  ┌────▼───┐
│ Blk 3 │  │ Blk 5  │
│ Timer │  │Analytcs│
└───────┘  └────────┘
      │          │
   ┌──▼──────────▼──┐
   │    Block 6     │
   │   Commands     │
   └───────┬────────┘
           │
     ┌─────▼──────┐
     │  Block 10  │  MUST BE LAST
     │ Integration│
     └────────────┘
```

**Maximum parallelism: 6 blocks** after Block 0 is done. Blocks 1, 2, 4, 7, 8, and 9 have no dependencies on each other.

**Recommended sequential order** (if working linearly as described):
0 → 1 → 3 → 4 → 2 → 7 → 5 → 6 → 8 → 9 → 10

This ordering front-loads the core loop (DB + Timer) and defers UI until data flows are established. But you can reorder as long as you respect the dependency arrows.

---

## Appendix A: Development Shortcuts

- **Shorten the timer for testing:** Allow work interval to be set as low as 1 minute in settings validation (or bypass validation in debug mode). Testing a 20-minute timer is miserable. Consider adding a `#[cfg(debug_assertions)]` flag that uses 10-second work intervals and 5-second breaks.
- **Seed test data:** Write a small utility function that inserts N random break records spanning the last 30 days. Call it from a debug-only command. This lets you test the analytics dashboard with realistic data without waiting 30 days.
- **Log everything in dev:** The timer loop, DB writes, event emissions — sprinkle `println!()` or `log::debug!()` during development. Remove or gate behind debug before shipping.

## Appendix B: Future Features (Out of Scope — Do Not Build)

Documenting these so they don't creep in, but the architecture should not *prevent* them:

1. Focus mode integration (calendar-aware pausing)
2. Multi-monitor overlay positioning
3. Team/social accountability features
4. Custom sound upload
5. Pomodoro mode (alternative timer pattern)
6. Screen dimming instead of overlay
7. Mobile companion notifications
8. OS widgets (macOS widget center, Windows widgets)

## Appendix C: Error Handling Philosophy

Follow these rules in every block:

1. **Never panic in production.** Use `unwrap_or_default()`, `if let`, or proper `Result` chains. A crash in a background utility that runs all day is unacceptable. If something fails, degrade — don't die.
2. **Degrade gracefully.** If the notification permission is denied, the timer still works. If the DB is somehow locked, the timer still ticks. If the sound system has no output device, skip the chime.
3. **Commands return `Result<T, String>`.** The String error is a human-readable description. The frontend can choose to show it or log it.
4. **Don't show error dialogs for non-critical failures.** Log them. The user doesn't need to know that a background analytics cache update failed.
5. **Do validate user input aggressively.** Settings validation is the one place where errors should be surfaced to the user (the `update_settings` command returns an error string).

## Appendix D: Platform Gotchas

**macOS:**
- Tray icons may be auto-templated (converted to monochrome). Test with both light and dark menu bars.
- Notification permission must be granted. First notification triggers the OS permission dialog.
- The overlay window should not appear below the ~24px menu bar.
- `.app` bundle must be signed for distribution (not needed for dev builds).

**Windows:**
- Tray icon is in the system tray (bottom-right). Windows may hide it in the overflow area by default — the user has to drag it out.
- The overlay window with `always_on_top` should work fine. Test that it doesn't interfere with fullscreen apps.
- The autostart registry key goes in `HKCU` (per-user, no admin needed).

**Linux:**
- Tray behavior varies wildly by desktop environment. GNOME has historically been hostile to tray icons (requires extensions). KDE, XFCE, and others support them natively.
- Notifications via `libnotify` — behavior varies by notification daemon.
- The transparent overlay window requires a compositing window manager. Under X11 without compositing, the transparent background may render as opaque black. Wayland handles transparency differently. This is an acceptable limitation.
- AppImage is the most portable distribution format.