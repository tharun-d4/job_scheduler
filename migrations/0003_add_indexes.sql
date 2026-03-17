CREATE INDEX claim_jobs_idx
ON jobs (status, priority DESC, created_at ASC, run_at ASC)
WHERE status = 'pending' AND attempts < max_retries;


CREATE INDEX job_status_idx
ON jobs (status);
