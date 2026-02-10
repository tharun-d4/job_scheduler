CREATE TABLE failed_jobs (
  id UUID PRIMARY KEY,
  job_type VARCHAR(100) NOT NULL,
  payload JSONB NOT NULL,
  priority SMALLINT NOT NULL,
  max_retries SMALLINT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL,
  started_at TIMESTAMPTZ NOT NULL,
  failed_at TIMESTAMPTZ NOT NULL,
  worker_id UUID NOT NULL,
  attempts SMALLINT NOT NULL,
  error_message TEXT NOT NULL,
  result JSONB
);
