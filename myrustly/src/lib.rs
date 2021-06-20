use log::info;
use std::{thread, time};

pub fn xxx(time: u64) {
    env_logger::init();
    let timer_sec = time::Duration::from_secs(time);
    let one_sec = time::Duration::from_secs(1);
    let now = time::Instant::now();

    for n in (0..timer_sec.as_secs()).rev() {
        thread::sleep(one_sec);
        info!("countdown: {}", n);
        info!("{} secs past", now.elapsed().as_secs().to_string());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn checkx() {
        xxx(10);
        assert_eq!("", "");
    }
}
