use crate::helpers::read_file;
use std::char;
use std::collections::HashMap;

fn parser() -> Vec<(Vec<u32>, Vec<u32>)> {
    let contents = read_file("/inputs/day4.txt");
    // WIP
    return contents
        .lines()
        .map(|l| {
            let mut pairs = l
                .split(",")
                .map(|range| range.split("-").map(|n| {
                    n.parse::<u32>().unwrap()
                }).collect());
            return (pairs.nth(0).unwrap(), pairs.nth(1).unwrap())
        })
        .collect();
}

pub fn solution() -> (String, String) {
    let vector: Vec<(Vec<u32>, Vec<u32>)> = parser();
    let result1: u32 = 0;
    let result2: u32 = 0;

    println!("{:?}", vector);

    return (result1.to_string(), result2.to_string());
}
