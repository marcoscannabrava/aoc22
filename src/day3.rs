use crate::helpers::read_file;
use std::char;
use std::collections::HashMap;

fn parser() -> Vec<String> {
    let contents = read_file("/inputs/day3.txt");
    return contents.lines().map(|x| x.to_owned()).collect();
}

pub fn solution() -> (String, String) {
    let vector: Vec<String> = parser();
    let mut alphabet = (10..36)
        .map(|i| char::from_digit(i, 36).unwrap().to_string())
        .collect::<Vec<String>>()
        .join("");
    alphabet += &alphabet.to_uppercase();
    let item_points = alphabet
        .chars()
        .enumerate()
        .map(|(i, v)| (v, i as u32 + 1))
        .collect::<HashMap<char, u32>>();

    let mut result1: u32 = 0;
    let result2: u32 = 0;
    'rucksacks: for rucksack in &vector {
        let mut half_one: HashMap<char, u32> = HashMap::new();
        for (i, item) in rucksack.chars().enumerate() {
            // build hash map from first half
            if i < rucksack.len() / 2 {
                half_one.entry(item).and_modify(|c| *c += 1).or_insert(1);
            } else {
                // check duplicates in second half
                if half_one.get(&item).unwrap_or(&0) > &0 {
                    // add up priorities
                    result1 += item_points.get(&item).unwrap_or(&0);
                    continue 'rucksacks;
                };
            }
        }
    }

    return (result1.to_string(), result2.to_string());
}
