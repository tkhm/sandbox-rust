mod lib;
use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Missing required argument(s)");
        exit(1);
    }

    let pomo_type = &args[1];
    match pomo_type.as_str() {
        "pomo" => lib::xxx(25),
        "break" => lib::xxx(5),
        _ => lib::xxx(0)
    }
}
