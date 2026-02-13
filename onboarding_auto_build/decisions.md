## Block 0: Onboarding State Infrastructure

- **Start in Paused phase (simpler approach)**: The spec offered two approaches for handling the timer during onboarding: a new initial state or starting in Paused. Chose the simpler approach — `lib.rs` checks `settings.onboarding_completed` and sets initial phase to `Paused` if false. `complete_onboarding` calls `timer::resume()` to transition to `Working`. This reuses existing pause/resume machinery with zero new state machine states.

- **Demo break returns to Paused via existing tick logic**: Rather than adding special demo-break state tracking, the timer's break completion transition checks `settings.onboarding_completed`. If false (meaning this was a demo break), it transitions to `Paused` instead of `Working`. This elegantly handles the "return to wizard" requirement without any new timer states.

- **Demo breaks don't create DB records**: `trigger_demo_break` sets the timer phase directly without calling `insert_break_record`. The break record ID stays `None`, so when the break completes, the DB finalization is skipped. This keeps demo breaks out of analytics entirely.

- **first_break_completed tracked in timer's CompleteBreak handler**: The spec says "set first_break_completed = true ... in the timer engine when a break completes and it detects this is the first one." Implemented exactly there — the CompleteBreak side-effect handler checks `settings.onboarding_completed && !settings.first_break_completed && break_record_id.is_some()` (the last condition excludes demo breaks) and persists the change to both in-memory settings and DB.

- **`DateTime::from_timestamp_millis` over deprecated `NaiveDateTime::from_timestamp_millis`**: chrono's `NaiveDateTime::from_timestamp_millis` is deprecated. Used `DateTime::from_timestamp_millis` + `.date_naive()` for the `is_first_day` computation.

- **Auto-complete logic in migration runner**: The spec says "if there are any break_records when migration 002 runs, set onboarding_completed = 1 and first_break_completed = 1." This logic lives in `run_migrations()` immediately after applying 002's ALTER TABLEs. It runs a simple `EXISTS` query on `break_records` and conditionally UPDATEs the settings row.

- **`tooltips_seen` stored as JSON string in UserSettings, parsed in OnboardingState**: Following the spec exactly — the DB and UserSettings carry the raw JSON string `"[]"`. The `OnboardingState` struct (returned to frontend) has `Vec<String>`. Conversion happens in `build_onboarding_state()`. This avoids JSON parsing on every settings load/save.

- **`mark_tooltip_seen` IPC parameter uses camelCase `tooltipId`**: Tauri v2 auto-converts snake_case command parameter names to camelCase for the frontend. The TypeScript wrapper uses `tooltipId` which maps to the Rust parameter `tooltip_id`.
