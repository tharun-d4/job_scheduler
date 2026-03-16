use std::process::Command;

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
    println!("Begin benchmarking...");

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

    let req_client = reqwest::Client::new();

    const TOTAL_JOBS: u32 = 10_000;
    let start = Instant::now();
    let mut success = 0;
    let mut error = 0;
    for _ in 0..TOTAL_JOBS {
        let response = req_client
            .post("http://127.0.0.1:8000/jobs")
            .json(&serde_json::json!({
                "job_type": "send_email",
                "payload": {
                    "to": "to_email@mail.com",
                    "from": "job_scheduler@mail.com",
                    "subject": "This is a sample load test / benchmark",
                    "body": "Yes this is just a sample load test / benchmark"
                },
                "priority": 10,
                "max_retries": 5,
            }))
            .send()
            .await;
        if let Ok(resp) = response
            && resp.status() == reqwest::StatusCode::CREATED
        {
            success += 1;
        } else {
            error += 1;
        }
    }
    println!("Submitted {} email jobs", TOTAL_JOBS);

    let end = start.elapsed();
    println!("== Submission results ==");
    println!("Duration: {:.2}sec", end.as_secs_f64());
    println!("Successful: {success}");
    println!("Errors: {error}");
    println!(
        "Rate: {:.2} jobs/sec",
        TOTAL_JOBS as f64 / end.as_secs_f64()
    );

    println!("Waiting for the workers to process the jobs");
    loop {
        let stats = req_client
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
        TOTAL_JOBS as f64 / end.as_secs_f64()
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
