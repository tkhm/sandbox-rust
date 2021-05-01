use std::io;
use std::cmp::Ordering;
use log::debug;

fn main() {
    env_logger::init();
    debug!("ferris_watch starting...");

    loop {
        println!("Please input something!");

        let secret_number = make_secret_number();
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("less"),
            Ordering::Greater => println!("greater"),
            Ordering::Equal => {
                println!("Got it!");
                break;
            }
        }
    }
}

fn make_secret_number() -> u32 {
    10
}
