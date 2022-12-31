use crate::helpers::read_file;
use std::collections::HashMap;

fn parser() -> Vec<(String, String)> {
    let contents = read_file("/inputs/day2.txt");

    let mut result: Vec<(String, String)> = Vec::new();
    for l in contents.lines() {
        let letters = l.split_whitespace().collect::<Vec<&str>>();
        result.push((letters[0].to_owned(), letters[1].to_owned()));
    }

    return result;
}

pub fn solution() -> (String, String) {
    // TODO: understand closures better to set global constant HashMaps and avoid duplicating `rules`
    let rules: HashMap<&str, Vec<&str>> = HashMap::from([
        ("win", vec!["CX", "AY", "BZ"]),
        ("draw", vec!["AX", "BY", "CZ"]),
        ("loss", vec!["BX", "CY", "AZ"]),
    ]);
    let second_rules: HashMap<&str, &str> =
        HashMap::from([("X", "loss"), ("Y", "draw"), ("Z", "win")]);
    let vector: Vec<(String, String)> = parser();

    fn calculate_points(code: &str) -> i32 {
        let strategy_points: HashMap<&str, i32> = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
        let match_points: HashMap<&str, i32> = HashMap::from([("win", 6), ("draw", 3)]);
        let rules: HashMap<&str, Vec<&str>> = HashMap::from([
            ("win", vec!["CX", "AY", "BZ"]),
            ("draw", vec!["AX", "BY", "CZ"]),
            ("loss", vec!["BX", "CY", "AZ"]),
        ]);
        let mut result: i32 = 0;
        result += strategy_points
            .get(&code.chars().last().unwrap().to_string() as &str)
            .unwrap();
        if rules.get("win").unwrap().contains(&code) {
            result += match_points.get("win").unwrap();
        } else if rules.get("draw").unwrap().contains(&code) {
            result += match_points.get("draw").unwrap();
        }
        return result;
    }

    let mut result1: i32 = 0;
    for (s1, s2) in &vector {
        let code = [s1.to_owned(), s2.to_owned()].join("");
        result1 += calculate_points(&code);
    }

    let mut result2: i32 = 0;
    for (s1, s2) in &vector {
        let code = rules
            .get(second_rules.get(&s2 as &str).unwrap())
            .unwrap()
            .iter()
            .find(|&&x| x.starts_with(s1))
            .unwrap();
        result2 += calculate_points(&code);
    }

    return (result1.to_string(), result2.to_string());
}
