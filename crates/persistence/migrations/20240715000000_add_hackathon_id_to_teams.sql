ALTER TABLE teams
    ADD COLUMN IF NOT EXISTS hackathon_id TEXT;

UPDATE teams AS t
SET hackathon_id = s.hackathon_id
FROM (
    SELECT DISTINCT ON (team_id) team_id, hackathon_id
    FROM submissions
    ORDER BY team_id, created_at DESC
) AS s
WHERE t.id = s.team_id
  AND t.hackathon_id IS NULL;

ALTER TABLE teams
    ADD CONSTRAINT teams_hackathon_id_fkey
    FOREIGN KEY (hackathon_id) REFERENCES hackathons(id) ON DELETE CASCADE;

ALTER TABLE teams
    ALTER COLUMN hackathon_id SET NOT NULL;

CREATE INDEX IF NOT EXISTS teams_hackathon_id_idx ON teams(hackathon_id);
