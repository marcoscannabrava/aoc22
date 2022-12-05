pub mod day1;
pub mod day2;
pub mod day3;
pub mod helpers;

fn main() {
    println!("Hello, AOC 2022!");
    println!("---");

    let day1_result = day1::solution();
    println!("Day 1, Part 1 Result: {}", day1_result.0);
    println!("Day 1, Part 2 Result: {}", day1_result.1);
    println!("---");

    let day2_result = day2::solution();
    println!("Day 2, Part 1 Result: {}", day2_result.0);
    println!("Day 2, Part 2 Result: {}", day2_result.1);
    println!("---");

    let day3_result = day3::solution();
    println!("Day 3, Part 1 Result: {}", day3_result.0);
    println!("Day 3, Part 2 Result: {}", day3_result.1);
    println!("---");
}
