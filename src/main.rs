mod day1;
mod day2;

fn main() {
    println!("DAY1:");
    println!(" - PROBLEM1: {}", day1::problem1::challenge());
    println!(" - PROBLEM2: {}", day1::problem2::challenge().unwrap());

    println!("DAY2:");
    println!(" - PROBLEM1: {}", day2::problem1::challenge());
}
