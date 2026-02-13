# Block 1: SQLite Database Layer

## Files Created or Modified

### Created / Fully Implemented
- `src-tauri/src/state.rs` — All sacred contract types: TimerPhase, TimerState, UserSettings (with Default impl), BreakRecord, DailyStats (with DailyStats::zero helper), AnalyticsSummary, AppState, DbConnection
- `src-tauri/src/db.rs` — Full database layer: init_db, init_db_conn (for testing), run_migrations, insert_break_record, update_break_completion, get_break_records, load_settings, save_settings, recompute_daily_stats, get_daily_stats_range, count_breaks_today, clear_all_data, export_to_csv. Includes 11 unit tests.
- `src-tauri/migrations/001_initial.sql` — Complete schema: _migrations, settings (single-row with CHECK id=1), break_records (indexed on started_at), daily_stats_cache

### Modified (to match sacred contracts)
- `src/lib/types.ts` — Updated all TypeScript types to exactly mirror Rust types (was out of sync from Block 0)
- `src/lib/commands.ts` — Updated all command wrappers to match the sacred contract table (was out of sync from Block 0)
- `src/hooks/useSettings.ts` — Updated default settings to match new UserSettings type

## Deviations from Spec

- **types.ts and commands.ts were out of sync with sacred contracts from Block 0.** Fixed them as part of this block since the DB types must mirror exactly. The old types.ts had different field names (e.g., `remaining_secs` instead of `seconds_remaining`, `work_duration_mins` instead of `work_interval_minutes`) and old commands.ts had non-contract commands (`start_timer`, `complete_break`, `play_sound`, `get_weekly_stats`).
- **Added `count_breaks_today()` function** — not explicitly listed in spec's CRUD list but required by spec in Block 3 ("Query the DB on startup to initialize `breaks_completed_today` correctly").
- **Added `init_db_conn()` convenience function** — separate from `init_db()` to allow testing with in-memory connections without the Mutex wrapper.
- **Used `rusqlite::Error::InvalidParameterName` for custom errors** — rusqlite doesn't have a straightforward custom error variant; this was the pragmatic choice for wrapping chrono parse errors and filesystem errors.

## Acceptance Criteria Results

| Criteria | Result |
|----------|--------|
| `cargo test` passes all DB tests | PASS — 11/11 tests pass |
| DB file is created in correct OS-specific location on first launch | PASS — `get_db_dir()` uses `dirs::data_dir()` + `com.blinky.app/blinky.db` (verified path logic; actual file creation happens at runtime via `init_db`) |
| Settings default to documented values | PASS — `test_default_settings` verifies all 10 fields match `UserSettings::default()` |
| After inserting 10 break records, `get_break_records(5, 0)` returns the 5 most recent | PASS — `test_insert_and_query_break_records` verifies pagination and ordering |
| `recompute_daily_stats` correctly calculates compliance rate and longest streak | PASS — `test_daily_stats_computation` verifies 3 completed + 1 skipped → 75% rate, streak = 3 |
| `clear_all_data` followed by `load_settings` returns defaults | PASS — `test_clear_all_data_resets_everything` verifies |
| `export_to_csv` creates a valid CSV file in Downloads directory | PASS — `test_export_to_csv` verifies file creation, header, and content |
| `cargo check` — no errors | PASS — only dead-code warnings (expected, functions unused until later blocks) |
| `npx tsc --noEmit` — no type errors | PASS |

## Known Issues / TODOs for Later Blocks

- Dead-code warnings for all db.rs and state.rs public items — will resolve as Block 3 (timer), Block 5 (analytics), and Block 6 (commands) start consuming them.
- The `export_to_csv` test creates and deletes a real file in the Downloads directory — this is fine for dev testing but may need sandboxing in CI.
- `DbConnection` wrapper struct exists for Tauri's `.manage()` — Block 6 (commands) and Block 10 (integration) will use it to pass the connection to command handlers.
