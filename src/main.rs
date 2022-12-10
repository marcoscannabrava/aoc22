pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
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
}
