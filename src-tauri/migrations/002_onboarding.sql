-- Onboarding state columns on the settings table

ALTER TABLE settings ADD COLUMN onboarding_completed INTEGER NOT NULL DEFAULT 0;
ALTER TABLE settings ADD COLUMN onboarding_completed_at INTEGER DEFAULT NULL;
ALTER TABLE settings ADD COLUMN tooltips_seen TEXT NOT NULL DEFAULT '[]';
ALTER TABLE settings ADD COLUMN first_break_completed INTEGER NOT NULL DEFAULT 0;
