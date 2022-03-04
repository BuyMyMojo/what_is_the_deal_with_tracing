use std::thread::sleep;
use std::time;
use rand::Rng;
use chrono::{Utc};
use tracing::{Level, instrument, info, span, event};
use tracing_subscriber;

#[instrument]
async fn testasync(numb: u64, created: String) {

    let mut rng = rand::thread_rng();
    let sleep_time = rng.gen_range(0..500);
    let fo = time::Duration::from_millis(sleep_time);
    sleep(fo);
    event!(Level::INFO, time_slept_in_ms = sleep_time, "Done test task!");
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let span = span!(Level::TRACE, "main func");

    const TASK_COUNT: u64 = 70;

    for numb in 0..TASK_COUNT {
        tokio::spawn(testasync(numb, Utc::now().to_rfc3339()));
    }
    event!(Level::INFO, "Done spawning {} tasks", TASK_COUNT);

    let fo = time::Duration::from_secs(10);
    sleep(fo);
    event!(Level::INFO, "Done final sleep");

    event!(Level::ERROR, "Closing now!");

}
