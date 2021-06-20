use std::io;
use std::cmp::Ordering;
use log::debug;

fn main() {
    env_logger::init();
    debug!("ferris_watch starting...");

    loop {
        println!("Please input something!");

        let seed = 20;
        let secret_number = make_secret_number(seed);
        println!("{}", seed);

        let seedchar = String::from("x");
        let secret_string = make_secret_string(&seedchar);
        println!("{}", seedchar);
        println!("{}", secret_string);

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let word = first_word(&guess);
        println!("first word is: {}", word);
        let user = build_user(String::from("xxx@example.com"), word.to_string());

        println!("{}", user.username);

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

fn make_secret_number(seed: u32) -> u32 {
    println!("{}", seed);

    let x = seed;
    x
}

fn make_secret_string(seed: &String) -> String {
    println!("{}", seed);

    let x = seed;
    x.to_string()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

/// # Examples
/// 
/// ```
/// let five = 5;
/// assert_eq!(6, main_copy::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
