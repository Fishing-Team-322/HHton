-- Rating participant and team profiles with event outcome history.
CREATE TABLE IF NOT EXISTS rating_participant_profiles (
    participant_id TEXT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    team_id TEXT REFERENCES teams(id) ON DELETE SET NULL,
    rating DOUBLE PRECISION NOT NULL DEFAULT 0,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS rating_team_profiles (
    team_id TEXT PRIMARY KEY REFERENCES teams(id) ON DELETE CASCADE,
    rating DOUBLE PRECISION NOT NULL DEFAULT 0,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS rating_event_outcomes (
    id UUID PRIMARY KEY,
    participant_id TEXT REFERENCES rating_participant_profiles(participant_id) ON DELETE CASCADE,
    team_id TEXT REFERENCES rating_team_profiles(team_id) ON DELETE CASCADE,
    position INTEGER NOT NULL CHECK (position >= 1),
    total_participants INTEGER NOT NULL CHECK (total_participants > 0),
    scale SMALLINT NOT NULL CHECK (scale BETWEEN 1 AND 4),
    finished_at DATE NOT NULL,
    team_size INTEGER NOT NULL CHECK (team_size > 0),
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS rating_event_outcomes_participant_idx
    ON rating_event_outcomes(participant_id, finished_at DESC);

CREATE INDEX IF NOT EXISTS rating_event_outcomes_team_idx
    ON rating_event_outcomes(team_id, finished_at DESC);
