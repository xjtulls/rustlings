use std::{sync::{Arc, Mutex}, thread, time::Duration};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // Wrap JobStatus in a Mutex and then in an Arc for thread-safe mutable access
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status); // Clone the Arc to share ownership
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // Lock the mutex before updating the shared value
            let mut status = status_shared.lock().unwrap();
            status.jobs_done += 1;
        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Lock the mutex before reading the value
    let final_status = status.lock().unwrap();
    println!("Jobs done: {}", final_status.jobs_done);
}
