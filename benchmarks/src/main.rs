use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use tokio::time::Instant;

#[derive(Debug, serde::Deserialize)]
pub struct JobStats {
    pub pending: i64,
    pub running: i64,
    pub completed: i64,
    pub failed: i64,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("args: {:?}", args);

    let total_jobs: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(10_000);
    let concurrency: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(100);

    benchmark_script(total_jobs, concurrency).await;
}

async fn benchmark_script(total_jobs: usize, concurrency: usize) {
    println!("=== Job Scheduler Benchmark ===");
    println!("total_jobs to be submitted: {}", total_jobs);
    println!("concurrent jobs to be submitted: {}", concurrency);

    let client = reqwest::Client::new();
    let success = Arc::new(AtomicU64::new(0));
    let error = Arc::new(AtomicU64::new(0));

    let mut handles = Vec::with_capacity(concurrency);

    println!("Submitting jobs...");
    let start = Instant::now();
    for i in 0..total_jobs {
        let client = client.clone();
        let success = success.clone();
        let error = error.clone();

        let handle = tokio::spawn(async move {
            let response = client
                .post("http://127.0.0.1:8000/jobs")
                .json(&serde_json::json!({
                    "job_type": "send_email",
                    "payload": {
                        "to": format!("user{}@mail.com", i),
                        "from": "job_scheduler@mail.com",
                        "subject": "Benchmark",
                        "body": "Load test"
                    },
                    "priority": 10,
                    "max_retries": 5,
                }))
                .send()
                .await;

            match response {
                Ok(resp) => {
                    if resp.status() == reqwest::StatusCode::CREATED {
                        success.fetch_add(1, Ordering::Relaxed);
                    }
                }
                Err(e) => {
                    eprintln!("error when submitting job: {:?}", e);
                    error.fetch_add(1, Ordering::Relaxed);
                }
            }
        });
        handles.push(handle);

        if handles.len() >= concurrency {
            for h in handles.drain(..) {
                h.await.ok();
            }
        }
    }
    for h in handles {
        h.await.ok();
    }
    println!("Submitted {} email jobs", total_jobs);

    let end = start.elapsed();
    println!("== Submission results ==");
    println!("Duration: {:.2}sec", end.as_secs_f64());
    println!("Successful: {:?}", success.load(Ordering::Relaxed));
    println!("Errors: {:?}", error.load(Ordering::Relaxed));
    println!(
        "Rate: {:.2} jobs/sec",
        total_jobs as f64 / end.as_secs_f64()
    );
}
