ALTER TABLE hackathons ADD COLUMN IF NOT EXISTS description TEXT;
ALTER TABLE hackathons ADD COLUMN IF NOT EXISTS start_time TIMESTAMPTZ;
ALTER TABLE hackathons ADD COLUMN IF NOT EXISTS end_time TIMESTAMPTZ;

UPDATE hackathons
SET
    description = COALESCE(description, 'Description pending'),
    start_time = COALESCE(start_time, registration_deadline),
    end_time = COALESCE(end_time, submission_deadline)
WHERE
    description IS NULL OR start_time IS NULL OR end_time IS NULL;

ALTER TABLE hackathons
    ALTER COLUMN description SET NOT NULL,
    ALTER COLUMN start_time SET NOT NULL,
    ALTER COLUMN end_time SET NOT NULL;

ALTER TABLE hackathons
    ADD CONSTRAINT hackathons_schedule_order CHECK (
        registration_deadline <= start_time
        AND start_time <= end_time
        AND end_time <= submission_deadline
    );
