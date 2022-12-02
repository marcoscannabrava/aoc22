pub mod day1;
pub mod helpers;

use day1::day1;

fn main() {
    println!("Hello, AOC 2022!");

    let day1_result = day1();
    println!("Day 1, Part 1 Result: {}", day1_result.0);
    println!("Day 1, Part 2 Result: {}", day1_result.1);
    println!("---");
}
