use crate::helpers::read_file;
use std::ops::RangeInclusive;

type Pair = (RangeInclusive<u32>, RangeInclusive<u32>);

fn parse_range(tuple: (&str, &str)) -> RangeInclusive<u32> {
    let (l, r) = tuple;
    l.parse().unwrap()..=r.parse().unwrap()
}

fn parser() -> Vec<Pair> {
    let contents = read_file("/inputs/day4.txt");
    contents
        .lines()
        .map(|l| {
            let (r1, r2) = l.split_once(",").unwrap();
            let (t1, t2) = (r1.split_once("-").unwrap(), r2.split_once("-").unwrap());
            (parse_range(t1), parse_range(t2))
        })
        .collect()
}

fn one_contains_another(ranges: &Pair) -> bool {
    let (r1, r2) = ranges;
    r1.contains(&r2.start()) && r1.contains(&r2.end())
        || r2.contains(&r1.start()) && r2.contains(&r1.end())
}

fn overlaps(ranges: &Pair) -> bool {
    let (r1, r2) = ranges;
    r1.contains(r2.start())
        || r1.contains(r2.end())
        || r2.contains(r1.start())
        || r2.contains(r1.end())
}

pub fn solution() -> (String, String) {
    let vector: Vec<Pair> = parser();
    let result1: u32 = vector
        .iter()
        .filter(|pair| one_contains_another(pair))
        .count()
        .try_into()
        .unwrap();
    let result2: u32 = vector
        .iter()
        .filter(|pair| overlaps(pair))
        .count()
        .try_into()
        .unwrap();

    return (result1.to_string(), result2.to_string());
}
