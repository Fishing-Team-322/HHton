ALTER TABLE submissions
    ADD COLUMN IF NOT EXISTS provider TEXT,
    ADD COLUMN IF NOT EXISTS repository TEXT,
    ADD COLUMN IF NOT EXISTS commit_sha TEXT,
    ADD COLUMN IF NOT EXISTS is_private BOOLEAN NOT NULL DEFAULT FALSE,
    ADD COLUMN IF NOT EXISTS proof_status TEXT,
    ADD COLUMN IF NOT EXISTS metadata_json JSONB;

UPDATE submissions
SET
    provider = COALESCE(NULLIF(provider, ''), 'unknown'),
    repository = COALESCE(NULLIF(repository, ''), 'legacy/' || id),
    commit_sha = COALESCE(NULLIF(commit_sha, ''), 'legacy')
WHERE provider IS NULL
   OR provider = ''
   OR repository IS NULL
   OR repository = ''
   OR commit_sha IS NULL
   OR commit_sha = '';

ALTER TABLE submissions
    ALTER COLUMN provider SET NOT NULL,
    ALTER COLUMN repository SET NOT NULL,
    ALTER COLUMN commit_sha SET NOT NULL;
