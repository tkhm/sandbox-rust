use log::info;
use std::{thread, time};

pub fn xxx(min: u64) {
    env_logger::init();
    let timer_sec = time::Duration::from_secs(min * 60);
    let now = time::Instant::now();

    info!("Start! {}:00", min);
    let pause_sec = time::Duration::from_secs(1);
    for _n in (0..timer_sec.as_secs()).rev() {
        thread::sleep(pause_sec);
        let past_secs = now.elapsed().as_secs();
        // display unit
        if past_secs % 10 == 0 {
            let remaining_sec = timer_sec.as_secs() - past_secs;
            let display_min = remaining_sec / 60;
            let display_sec = remaining_sec % 60;
            // TODO: 0 sec will be 2:0 but it should be 2:00
            info!("{}:{}", display_min, display_sec);
        }
    }
    info!("Time is up! ({}:00)", min);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn checkx() {
        xxx(2);
        assert_eq!("", "");
    }
}
