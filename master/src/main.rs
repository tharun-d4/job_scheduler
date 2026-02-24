use std::process::{Child, Command};

fn spawn_worker() -> Child {
    let child = Command::new("./target/debug/worker")
        .spawn()
        .expect("Failed to spawn worker process");
    println!("Spawned Worker PID: {:?}", child.id());
    child
}

fn main() {
    const WORKERS: u8 = 2;

    let mut workers = Vec::new();
    for _ in 0..WORKERS {
        workers.push(spawn_worker());
    }

    loop {
        for worker in workers.iter_mut() {
            if let Ok(Some(status)) = worker.try_wait() {
                println!("Worker exited with status: {status}");

                *worker = spawn_worker();
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
