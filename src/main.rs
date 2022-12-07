#![feature(iter_next_chunk)]
#![feature(iter_array_chunks)]
#![feature(iter_collect_into)]

extern crate core;

use std::collections::HashMap;
use std::env;
use std::sync::Mutex;

mod days;
mod io;

use crate::days::day01::day01;

pub static DAYS: Mutex<Option<HashMap<&str, fn() -> ()>>> = Mutex::new(None);

fn main() {
    *DAYS.lock().unwrap() = Some(HashMap::from([
        ("day01", day01 as fn() -> ()),
        ("day07", day07 as fn() -> ()),
    ]));

    let args: Vec<String> = env::args().collect();
    let day = &args[1];

    DAYS.lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .get(day.as_str())
        .unwrap()();
}
