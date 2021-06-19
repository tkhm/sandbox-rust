use log::debug;
use log::info;
use std::cmp::Ordering;
use std::io;

fn main() {
    env_logger::init();
    debug!("ferris_watch starting...でござる");
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    info!("area: {}", rect1.area());
    info!("area: {}", 'た');
    debug!("rect1: {:?}", &rect1);
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    debug!("home: {:?}", &home);
    debug!("loopback: {:?}", &loopback);
    let x = None;

    if x.is_some() {
        let y: IpAddr = x.unwrap();
        info!("hi {:?}", y);
    }

    let s1 = String::from("あいう");
    let len = s1.len();
    info!("{}", len);
}

/// aaa
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /// aaa
    /// 
    /// # Examples
    /// 
    /// ```
    /// let rect1 = main::Rectangle {
    ///   width: 30,
    ///   height: 50,
    /// };
    /// assert_eq!(1500, rect1.area());
    /// ```
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}
