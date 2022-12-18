pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod helpers;

fn main() {
    println!("Hello, AOC 2022!");
    println!("---");

    let day1_result = day1::solution();
    println!("Day 1, Part 1 Answer: {}", day1_result.0);
    println!("Day 1, Part 2 Answer: {}", day1_result.1);
    println!("---");

    let day2_result = day2::solution();
    println!("Day 2, Part 1 Answer: {}", day2_result.0);
    println!("Day 2, Part 2 Answer: {}", day2_result.1);
    println!("---");

    let day3_result = day3::solution();
    println!("Day 3, Part 1 Answer: {}", day3_result.0);
    println!("Day 3, Part 2 Answer: {}", day3_result.1);
    println!("---");

    let day4_result = day4::solution();
    println!("Day 4, Part 1 Answer: {}", day4_result.0);
    println!("Day 4, Part 2 Answer: {}", day4_result.1);
    println!("---");

    let day5_result = day5::solution();
    println!("Day 5, Part 1 Answer: {}", day5_result.0);
    println!("Day 5, Part 2 Answer: {}", day5_result.1);
    println!("---");

    let day6_result = day6::solution();
    println!("Day 6, Part 1 Answer: {}", day6_result.0);
    println!("Day 6, Part 2 Answer: {}", day6_result.1);
    println!("---");

    let day7_result = day7::solution();
    println!("Day 7, Part 1 Answer: {}", day7_result.0);
    println!("Day 7, Part 2 Answer: {}", day7_result.1);
    println!("---");
}
