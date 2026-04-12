CREATE TYPE run_mode_type AS ENUM ('immediate', 'scheduled', 'recurring', 'workflow');

ALTER TABLE jobs
ADD COLUMN run_mode run_mode_type NOT NULL DEFAULT 'immediate',
ADD COLUMN cron_expression TEXT;

