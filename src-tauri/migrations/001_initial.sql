-- Blinky initial schema

CREATE TABLE IF NOT EXISTS _migrations (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    applied_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS settings (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    work_interval_minutes INTEGER NOT NULL DEFAULT 20,
    break_duration_seconds INTEGER NOT NULL DEFAULT 20,
    sound_enabled INTEGER NOT NULL DEFAULT 1,
    sound_volume REAL NOT NULL DEFAULT 0.7,
    notification_enabled INTEGER NOT NULL DEFAULT 1,
    overlay_enabled INTEGER NOT NULL DEFAULT 1,
    launch_at_login INTEGER NOT NULL DEFAULT 0,
    daily_goal INTEGER NOT NULL DEFAULT 24,
    idle_pause_minutes INTEGER NOT NULL DEFAULT 5,
    theme TEXT NOT NULL DEFAULT 'system'
);

INSERT OR IGNORE INTO settings (id) VALUES (1);

CREATE TABLE IF NOT EXISTS break_records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    started_at INTEGER NOT NULL,
    duration_seconds INTEGER NOT NULL DEFAULT 0,
    completed INTEGER NOT NULL DEFAULT 0,
    skipped INTEGER NOT NULL DEFAULT 0,
    preceding_work_seconds INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_break_records_started_at ON break_records(started_at);

CREATE TABLE IF NOT EXISTS daily_stats_cache (
    date TEXT PRIMARY KEY,
    breaks_completed INTEGER NOT NULL DEFAULT 0,
    breaks_skipped INTEGER NOT NULL DEFAULT 0,
    total_rest_seconds INTEGER NOT NULL DEFAULT 0,
    longest_streak INTEGER NOT NULL DEFAULT 0,
    compliance_rate REAL NOT NULL DEFAULT 0.0
);
