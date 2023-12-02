use std::env;

mod exercises;

fn main() {
    let days = [exercises::day1::go];
    let mut args = env::args();
    let day = match args.nth(1) {
        Some(x) => x.parse().unwrap(),
        None => days.len(),
    };
    days[day - 1]();
}
