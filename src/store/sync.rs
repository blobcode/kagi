// ignore for now
// run save on the db every 500ms.

use std::{
    fs, thread,
    time::{Duration, Instant},
};

// sync to file every 500ms
pub fn run() {
    let scheduler = thread::spawn(|| {
        let wait_time = Duration::from_millis(500);

        for _ in 0..5 {
            let start = Instant::now();

            let thread = thread::spawn(sync);

            thread.join().unwrap();

            let runtime = start.elapsed();

            if let Some(remaining) = wait_time.checked_sub(runtime) {
                thread::sleep(remaining);
            }
        }
    });

    scheduler.join().unwrap();
}

fn sync() {
    thread::sleep(Duration::from_millis(100))
}
