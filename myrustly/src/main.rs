use log::info;
use std::{thread, time};

fn main() {
    env_logger::init();
    let timer_sec = time::Duration::from_secs(10);
    let one_sec = time::Duration::from_secs(1);
    let now = time::Instant::now();

    for n in (0..timer_sec.as_secs()).rev() {
        thread::sleep(one_sec);
        info!("countdown: {}", n);
        info!("{} secs past", now.elapsed().as_secs().to_string());
    }
}