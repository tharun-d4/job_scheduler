use std::process::{Child, Command};

use shared::config::load_supervisor_config;

fn main() {
    let config = load_supervisor_config("./config").expect("Config Error");

    let mut workers = Vec::with_capacity(config.workers as usize);
    for _ in 0..config.workers {
        workers.push(spawn_worker());
    }

    loop {
        for worker in workers.iter_mut() {
            if let Ok(Some(status)) = worker.try_wait() {
                println!("Worker exited with status: {status}");

                *worker = spawn_worker();
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(config.poll_interval as u64));
    }
}

fn spawn_worker() -> Child {
    let child = Command::new("./target/debug/worker")
        .spawn()
        .expect("Failed to spawn worker process");
    println!("Spawned Worker PID: {:?}", child.id());
    child
}
