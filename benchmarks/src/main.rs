use std::{
    process::Command,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

use tokio::time::Instant;

#[derive(Debug, serde::Deserialize)]
pub struct JobStats {
    pub pending: i64,
    pub running: i64,
    pub completed: i64,
    pub failed: i64,
}

#[tokio::main]
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

    let build_server = Command::new("cargo")
        .args(["build", "--release", "--bin", "server"])
        .output()
        .expect("failed to compile server");

    println!("build_server: {:?}", build_server);

    let server_process = Command::new("./target/release/server")
        .spawn()
        .expect("failed to spawn a server process");
    println!("Started Server (pid: {:?})", server_process.id());

    let build_worker = Command::new("cargo")
        .args(["build", "release", "--bin", "worker"])
        .output()
        .expect("failed to compile worker");
    println!("build_worker: {:?}", build_worker);

    let supervisor_process = Command::new("cargo")
        .args(["run", "--release", "--bin", "worker_supervisor"])
        .spawn()
        .expect("failed to start worker supervisor");

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

    println!("Waiting for the workers to process the jobs");
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        let stats = client
            .get("http://127.0.0.1:8000/jobs/stats")
            .send()
            .await
            .unwrap()
            .json::<JobStats>()
            .await
            .unwrap();

        if stats.pending == 0 && stats.running == 0 {
            println!("final stats: {:?}", stats);
            break;
        }
    }

    let end = start.elapsed();
    println!("== Processing results ==");
    println!("Processing time: {:.2}sec", end.as_secs_f64());
    println!(
        "Processing Rate: {:.2} jobs/sec",
        success.load(Ordering::Relaxed) as f64 / end.as_secs_f64()
    );

    // kill the server, worker supervisor processes by sending SIGTERM
    Command::new("kill")
        .args([
            "-TERM",
            &server_process.id().to_string(),
            &supervisor_process.id().to_string(),
        ])
        .output()
        .expect("Killing server process");
}
