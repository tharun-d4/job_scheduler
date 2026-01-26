INSERT INTO jobs 
(id, job_type, payload, status, priority, max_retries, created_at)
VALUES
(
  '019bfadc-28bb-781d-9d22-acf23fe50117',
  'send_email',
  '{
    "to": "to_email@mail.com",
    "from": "job_scheduler@mail.com",
    "subject": "This is a sample test",
    "body": "Yes this is just a sample api test"
  }',
  'pending',
  5,
  5,
  NOW() AT TIME ZONE 'UTC'
);
