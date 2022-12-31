use crate::helpers::read_file;
use regex::Regex;

type Operation = (usize, usize, usize);

fn parse_crate_line(l: &str, crates: &mut Vec<Vec<String>>) {
    let mut is_crate_item = false;
    l.chars().enumerate().for_each(|(i, c)| {
        if is_crate_item {
            let crate_idx = (i + 3) / 4;
            while crates.len() < crate_idx + 1 {
                crates.push(vec![]);
            }
            crates[crate_idx].push(c.to_string());
        }
        is_crate_item = c == '[';
    });
}

fn get_operation(l: &str) -> Operation {
    let re = Regex::new(r"\d+").unwrap();
    let ops = re
        .captures_iter(l)
        .map(|cap| cap[0].parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    (ops[0], ops[1], ops[2])
}

fn find_start_of_operations(input: core::str::Lines<'_>) -> usize {
    let mut index: usize = 0;
    input.enumerate().for_each(|(i, l)| {
        if l == "" {
            index = i;
        }
    });
    return index;
}

fn run_operation(op: Operation, crates: &mut Vec<Vec<String>>) {
    for _ in 0..op.0 {
        let krate = crates[op.1].pop();
        match krate {
            Some(ok) => crates[op.2].push(ok),
            None => (),
        }
    }
}

fn run_operation_updated(op: Operation, crates: &mut Vec<Vec<String>>) {
    let idx = crates[op.1].len() - op.0;
    let krates = crates[op.1][idx..].to_vec();
    crates[op.2] = [crates[op.2].clone(), krates].concat();
    crates[op.1] = crates[op.1][..idx].to_vec();
}

fn parser(contents: String) -> (Vec<Vec<String>>, Vec<Operation>) {
    let mut crates: Vec<Vec<String>> = Vec::new();
    let mut operations: Vec<Operation> = Vec::new();
    let start_of_operations_index = find_start_of_operations(contents.lines());

    // populate crates
    contents
        .lines()
        .take(start_of_operations_index - 1)
        .collect::<Vec<&str>>()
        .iter()
        .rev()
        .for_each(|l| parse_crate_line(l, &mut crates));

    // populate operations
    contents
        .lines()
        .skip(start_of_operations_index + 1)
        .for_each(|l| {
            operations.push(get_operation(l));
        });

    return (crates, operations);
}

fn parse_answer(crates: &Vec<Vec<String>>) -> String {
    crates
        .iter()
        .map(|c| c.last().unwrap_or(&"".to_owned()).to_owned())
        .collect()
}

pub fn solution() -> (String, String) {
    let contents = read_file("/inputs/day5.txt");
    let (crates, operations) = parser(contents);
    let mut crates_one = crates.clone();
    let mut crates_two = crates.clone();
    
    for op in operations {
        run_operation(op, &mut crates_one);
        run_operation_updated(op, &mut crates_two);
    }

    let result1: String = parse_answer(&crates_one);
    let result2: String = parse_answer(&crates_two);

    return (result1.to_string(), result2.to_string());
}

#[cfg(test)]
mod tests {
    use crate::day5;

    const TEST_INPUT: &str = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn parse_crate_line() {
        let mut crates: Vec<Vec<String>> = Vec::new();
        day5::parse_crate_line(TEST_INPUT.lines().collect::<Vec<&str>>()[3], &mut crates);
        day5::parse_crate_line(TEST_INPUT.lines().collect::<Vec<&str>>()[2], &mut crates);
        day5::parse_crate_line(TEST_INPUT.lines().collect::<Vec<&str>>()[1], &mut crates);
        assert!(crates[1].pop().unwrap() == "N");
        assert!(crates[2].pop().unwrap() == "D");
        assert!(crates[3].pop().unwrap() == "P");
    }

    #[test]
    fn reverse_iter() {
        let index = day5::find_start_of_operations(TEST_INPUT.lines());
        let crates: Vec<String> = TEST_INPUT
            .lines()
            .take(index - 1)
            .collect::<Vec<&str>>()
            .iter()
            .rev()
            .map(|l| l.to_owned().to_owned())
            .collect::<Vec<String>>();
        assert!(crates[0] == "[Z] [M] [P]");
    }

    #[test]
    fn parse_operations() {
        let operation_strings = &TEST_INPUT.lines().collect::<Vec<&str>>()[6..=7];
        let op_one = day5::get_operation(operation_strings[0]);
        let op_two = day5::get_operation(operation_strings[1]);
        assert!(op_one == (1, 2, 1));
        assert!(op_two == (3, 1, 3));
    }

    #[test]
    fn result1() {
        let (mut crates, operations) = day5::parser(TEST_INPUT.to_owned());

        for op in operations {
            day5::run_operation(op, &mut crates)
        };

        assert!(day5::parse_answer(&crates) == "CMZ".to_owned());
    }

    #[test]
    fn result2() {
        let (mut crates, operations) = day5::parser(TEST_INPUT.to_owned());

        for op in operations {
            day5::run_operation_updated(op, &mut crates)
        };

        assert!(day5::parse_answer(&crates) == "MCD".to_owned());
    }
}
