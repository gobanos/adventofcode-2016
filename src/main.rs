#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate crypto;

use std::thread;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    let day1_p1_thread = thread::spawn(day1::problem1::challenge);
    let day1_p2_thread = thread::spawn(day1::problem2::challenge);

    let day2_p1_thread = thread::spawn(day2::problem1::challenge);
    let day2_p2_thread = thread::spawn(day2::problem2::challenge);

    let day3_p1_thread = thread::spawn(day3::problem1::challenge);
    let day3_p2_thread = thread::spawn(day3::problem2::challenge);

    let day4_p1_thread = thread::spawn(day4::problem1::challenge);
    let day4_p2_thread = thread::spawn(day4::problem2::challenge);

    let day5_p1_thread = thread::spawn(day5::problem1::challenge);
    let day5_p2_thread = thread::spawn(day5::problem2::challenge);

    let day6_p1_thread = thread::spawn(day6::problem1::challenge);
    let day6_p2_thread = thread::spawn(day6::problem2::challenge);

    let day7_p1_thread = thread::spawn(day7::problem1::challenge);
    let day7_p2_thread = thread::spawn(day7::problem2::challenge);

    println!("DAY1:");
    println!(" - PROBLEM1: {}", day1_p1_thread.join().unwrap());
    println!(" - PROBLEM2: {}", day1_p2_thread.join().unwrap());

    println!("DAY2:");
    println!(" - PROBLEM1: {}", day2_p1_thread.join().unwrap());
    println!(" - PROBLEM2: {}", day2_p2_thread.join().unwrap());

    println!("DAY3:");
    println!(" - PROBLEM1: {}", day3_p1_thread.join().unwrap());
    println!(" - PROBLEM2: {}", day3_p2_thread.join().unwrap());

    println!("DAY4:");
    println!(" - PROBLEM1: {}", day4_p1_thread.join().unwrap());
    println!(" - PROBLEM2: {}", day4_p2_thread.join().unwrap());

    println!("DAY5:");
    println!(" - PROBLEM1: {}", day5_p1_thread.join().unwrap());
    println!(" - PROBLEM2: {}", day5_p2_thread.join().unwrap());

    println!("DAY6:");
    println!(" - PROBLEM1: {}", day6_p1_thread.join().unwrap());
    println!(" - PROBLEM2: {}", day6_p2_thread.join().unwrap());

    println!("DAY7:");
    println!(" - PROBLEM1: {}", day7_p1_thread.join().unwrap());
    println!(" - PROBLEM2: {}", day7_p2_thread.join().unwrap());
}
