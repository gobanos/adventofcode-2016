#[macro_use]
extern crate lazy_static;
extern crate regex;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    println!("DAY1:");
    println!(" - PROBLEM1: {}", day1::problem1::challenge());
    println!(" - PROBLEM2: {}", day1::problem2::challenge().unwrap());

    println!("DAY2:");
    println!(" - PROBLEM1: {}", day2::problem1::challenge());
    println!(" - PROBLEM2: {}", day2::problem2::challenge());

    println!("DAY3:");
    println!(" - PROBLEM1: {}", day3::problem1::challenge());
    println!(" - PROBLEM2: {}", day3::problem2::challenge());

    println!("DAY4:");
    println!(" - PROBLEM1: {}", day4::problem1::challenge());
}
