use crate::helpers::read_file;

fn parser() -> Vec<Pair> {
    let contents = read_file("/inputs/day6.txt");
    contents
        .lines()
        .map(|l| {
            let (r1, r2) = l.split_once(",").unwrap();
            let (t1, t2) = (r1.split_once("-").unwrap(), r2.split_once("-").unwrap());
            (parse_range(t1), parse_range(t2))
        })
        .collect()
}


pub fn solution() -> (String, String) {
    let vector: Vec<Pair> = parser();
    let result1: u32 = 0;
    let result2: u32 = 0;

    return (result1.to_string(), result2.to_string());
}
