use crate::helpers::read_file;
use std::collections::HashSet;

fn find_first_n_distinct(s: &str, n: usize) -> usize {
    for i in n..s.chars().collect::<Vec<char>>().len() {
        let set: HashSet<char> = HashSet::from_iter(s.chars().skip(i - n).take(n));
        if set.len() == n {
            return i;
        }
    }
    return 0;
}

pub fn solution() -> (String, String) {
    let contents = read_file("/inputs/day6.txt");
    let result1: usize = find_first_n_distinct(&contents, 4);
    let result2: usize = find_first_n_distinct(&contents, 14);

    return (result1.to_string(), result2.to_string());
}

#[cfg(test)]
mod tests {
    use crate::day6;

    const TEST_INPUT: &[(&str, u8); 4] = &[
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    ];

    #[test]
    fn find_first_n_distinct() {
        for (s, expected) in TEST_INPUT.iter() {
            println!("string: {:?}", s);
            println!("index: {:?}", day6::find_first_n_distinct(s, 4));
            assert_eq!(day6::find_first_n_distinct(s, 4), *expected as usize);
        }
    }
}
