/*
--- Day 5: Supply Stacks ---
The expedition can depart as soon as the final supplies have been unloaded from the ships. Supplies are stored in stacks of marked crates, but because the needed supplies are buried under many other crates, the crates need to be rearranged.

The ship has a giant cargo crane capable of moving crates between stacks. To ensure none of the crates get crushed or fall over, the crane operator will rearrange them in a series of carefully-planned steps. After the crates are rearranged, the desired crates will be at the top of each stack.

The Elves don't want to interrupt the crane operator during this delicate procedure, but they forgot to ask her which crate will end up where, and they want to be ready to unload them as soon as possible so they can embark.

They do, however, have a drawing of the starting stacks of crates and the rearrangement procedure (your puzzle input). For example:

    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
In this example, there are three stacks of crates. Stack 1 contains two crates: crate Z is on the bottom, and crate N is on top. Stack 2 contains three crates; from bottom to top, they are crates M, C, and D. Finally, stack 3 contains a single crate, P.

Then, the rearrangement procedure is given. In each step of the procedure, a quantity of crates is moved from one stack to a different stack. In the first step of the above rearrangement procedure, one crate is moved from stack 2 to stack 1, resulting in this configuration:

[D]
[N] [C]
[Z] [M] [P]
 1   2   3
In the second step, three crates are moved from stack 1 to stack 3. Crates are moved one at a time, so the first crate to be moved (D) ends up below the second and third crates:

        [Z]
        [N]
    [C] [D]
    [M] [P]
 1   2   3
Then, both crates are moved from stack 2 to stack 1. Again, because crates are moved one at a time, crate C ends up below crate M:

        [Z]
        [N]
[M]     [D]
[C]     [P]
 1   2   3
Finally, one crate is moved from stack 1 to stack 2:

        [Z]
        [N]
        [D]
[C] [M] [P]
 1   2   3
The Elves just need to know which crate will end up on top of each stack; in this example, the top crates are C in stack 1, M in stack 2, and Z in stack 3, so you should combine these together and give the Elves the message CMZ.

After the rearrangement procedure completes, what crate ends up on top of each stack?

Your puzzle answer was SPFMVDTZT.

--- Part Two ---
As you watch the crane operator expertly rearrange the crates, you notice the process isn't following your prediction.

Some mud was covering the writing on the side of the crane, and you quickly wipe it away. The crane isn't a CrateMover 9000 - it's a CrateMover 9001.

The CrateMover 9001 is notable for many new and exciting features: air conditioning, leather seats, an extra cup holder, and the ability to pick up and move multiple crates at once.

Again considering the example above, the crates begin in the same configuration:

    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 
Moving a single crate from stack 2 to stack 1 behaves the same as before:

[D]        
[N] [C]    
[Z] [M] [P]
 1   2   3 
However, the action of moving three crates from stack 1 to stack 3 means that those three moved crates stay in the same order, resulting in this new configuration:

        [D]
        [N]
    [C] [Z]
    [M] [P]
 1   2   3
Next, as both crates are moved from stack 2 to stack 1, they retain their order as well:

        [D]
        [N]
[C]     [Z]
[M]     [P]
 1   2   3
Finally, a single crate is still moved from stack 1 to stack 2, but now it's crate C that gets moved:

        [D]
        [N]
        [Z]
[M] [C] [P]
 1   2   3
In this example, the CrateMover 9001 has put the crates in a totally different order: MCD.

Before the rearrangement process finishes, update your simulation so that the Elves know where they should stand to be ready to unload the final supplies. After the rearrangement procedure completes, what crate ends up on top of each stack?
*/
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

fn parse_answer_one(crates: &Vec<Vec<String>>) -> String {
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

    let result1: String = parse_answer_one(&crates_one);
    let result2: String = parse_answer_one(&crates_two);

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

        assert!(day5::parse_answer_one(&crates) == "CMZ".to_owned());
    }

    #[test]
    fn result2() {
        let (mut crates, operations) = day5::parser(TEST_INPUT.to_owned());

        for op in operations {
            day5::run_operation_updated(op, &mut crates)
        };

        assert!(day5::parse_answer_one(&crates) == "MCD".to_owned());
    }
}
