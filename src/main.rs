extern crate regex;
extern crate chrono;

use std::env;
use std::env::Args;
use chrono::prelude::*;
use regex::Regex;

fn main() {
    let args = env::args();

    let res = get_res(args);

    println!("{}", res);
}

fn get_res(args: Args) -> String {
    if args.len() == 2 {
        let input_time = args.last();
        if let Some(time) = input_time {
            let re = Regex::new(r"^\d{0,10}$").unwrap();
            if re.is_match(&time) {
                let time: i64 = match time.trim().parse() {
                    Ok(num) => num,
                    Err(_) => 0,
                };
                return Local.timestamp(time, 0).naive_local().to_string();
            } else {
                if let Ok(time) = Local.datetime_from_str(&time, "%Y-%m-%d %H:%M:%S") {
                    return time.timestamp().to_string();
                }
            }
        }
    }

    let now = Local::now();
    now.timestamp().to_string() + "\n" + &now.format("%Y-%m-%d %H:%M:%S").to_string()
}
