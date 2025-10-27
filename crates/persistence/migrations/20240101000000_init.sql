-- Users participating in hackathons.
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    display_name TEXT NOT NULL,
    email TEXT NOT NULL
);

-- Teams formed by users.
CREATE TABLE IF NOT EXISTS teams (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS team_members (
    team_id TEXT NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    PRIMARY KEY (team_id, user_id)
);

-- Hackathon events coordinated on the platform.
CREATE TABLE IF NOT EXISTS hackathons (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    registration_deadline TIMESTAMPTZ NOT NULL,
    submission_deadline TIMESTAMPTZ NOT NULL,
    team_size_limit INTEGER NOT NULL CHECK (team_size_limit > 0)
);

-- Submissions from teams for specific hackathons.
CREATE TABLE IF NOT EXISTS submissions (
    id TEXT PRIMARY KEY,
    team_id TEXT NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    hackathon_id TEXT NOT NULL REFERENCES hackathons(id) ON DELETE CASCADE,
    summary TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS submissions_team_id_idx ON submissions(team_id);
CREATE INDEX IF NOT EXISTS submissions_hackathon_id_idx ON submissions(hackathon_id);
