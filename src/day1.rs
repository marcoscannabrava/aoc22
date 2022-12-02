use crate::helpers::read_file;

pub fn day1_parser() -> Vec<Vec<u32>> {
    let contents = read_file("/inputs/day1.txt");

    let mut result: Vec<Vec<u32>> = Vec::new();
    let mut subarray = Vec::new();
    for l in contents.lines() {
        if l.is_empty() {
            result.push(subarray);
            subarray = Vec::new();
        } else {
            subarray.push(l.parse::<u32>().unwrap());
        }
    }
    result.push(subarray);

    return result
}

pub fn day1() -> (String, String) {
    let vector = day1_parser();
    let mut map_of_sums = vector.iter().map(|x| x.iter().sum::<u32>()).collect::<Vec<u32>>();
    map_of_sums.sort();
    let top_three = map_of_sums.into_iter().rev().take(3).collect::<Vec<u32>>();
    let max = top_three.iter().max();
    let sum = top_three.iter().sum::<u32>();
    
    return (max.unwrap().to_string(), sum.to_string())
}