use crate::helpers::read_file;

#[derive(Debug, PartialEq)]
enum OperationType {
    Multiplication,
    Addition,
}

#[derive(Debug, Default)]
struct MonkeyTest {
    divisible_by: u32,
    true_monkey: usize,
    false_monkey: usize,
}

#[derive(Debug, Default)]
struct Monkey {
    items: Vec<u32>,
    operation_type: Option<OperationType>,
    operation_number: Option<u32>,
    test: MonkeyTest,
}

type Jungle = Vec<Monkey>;

trait Inspector {
    fn inspect(&mut self, item: u32) -> u32;
}

trait Thrower {
    fn throw(&self, item: u32, monkeys: &mut Jungle);
}

impl Inspector for Monkey {
    fn inspect(&mut self, item: u32) -> u32 {
        match self.operation_type.as_mut().unwrap() {
            OperationType::Multiplication => match self.operation_number {
                Some(n) => n * item,
                None => item * item,
            },
            OperationType::Addition => match self.operation_number {
                Some(n) => n + item,
                None => item + item,
            },
        }
    }
}

impl Thrower for Monkey {
    fn throw(&self, item: u32, monkeys: &mut Jungle) {
        let throw_to: &mut Monkey = match item % self.test.divisible_by == 0 {
            true => &mut monkeys[self.test.true_monkey],
            false => &mut monkeys[self.test.false_monkey],
        };
        throw_to.items.push(item);
    }
}

fn parser(input: &str) -> Jungle {
    let mut monkeys: Jungle = Vec::new();
    for line in input.lines() {
        if line.starts_with("Monkey") {
            monkeys.push(Monkey::default());
        }
        if line.starts_with("  Starting items:") {
            let items: Vec<u32> = line
                .split(": ")
                .nth(1)
                .unwrap()
                .split(", ")
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            monkeys.last_mut().unwrap().items = items;
        }
        if line.starts_with("  Operation:") {
            let operation_symbol = line
                .split(": new = old ")
                .nth(1)
                .unwrap()
                .chars()
                .take(1)
                .collect::<String>();
            match operation_symbol.as_str() {
                "*" => {
                    monkeys.last_mut().unwrap().operation_type = Some(OperationType::Multiplication)
                }
                "+" => monkeys.last_mut().unwrap().operation_type = Some(OperationType::Addition),
                _ => panic!("Unknown operation symbol"),
            }
            let operation_number = line.split(" ").last().unwrap().parse::<u32>();
            monkeys.last_mut().unwrap().operation_number = match operation_number {
                Ok(n) => Some(n),
                Err(_) => None,
            }
        }
        if line.starts_with("  Test:") {
            let divisible_by_number = line.split(" ").last().unwrap().parse::<u32>().unwrap();
            monkeys.last_mut().unwrap().test.divisible_by = divisible_by_number;
        }
        if line.starts_with("    If true:") {
            let monkey = line.split(" ").last().unwrap().parse::<u32>().unwrap();
            monkeys.last_mut().unwrap().test.true_monkey = monkey as usize;
        }
        if line.starts_with("    If false:") {
            let monkey = line.split(" ").last().unwrap().parse::<u32>().unwrap();
            monkeys.last_mut().unwrap().test.false_monkey = monkey as usize;
        }
    }
    monkeys
}

pub fn solution() -> (String, String) {
    let contents = read_file("/inputs/day11.txt");

    let result1: usize = 0;
    let result2: usize = 0;

    return (result1.to_string(), result2.to_string());
}

#[cfg(test)]
mod tests {
    use crate::day11;

    const TEST_INPUT: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn parser() {
        let jungle = day11::parser(TEST_INPUT);

        assert_eq!(jungle.len(), 4);
        assert_eq!(jungle[0].items, vec![79, 98]);
        assert_eq!(jungle[0].operation_type, Some(day11::OperationType::Multiplication));
        assert_eq!(jungle[0].operation_number, Some(19));
        
        assert_eq!(jungle[2].operation_number, None);
        assert_eq!(jungle[2].test.divisible_by, 13);
        assert_eq!(jungle[2].test.true_monkey, 1);
        assert_eq!(jungle[2].test.false_monkey, 3);

        assert_eq!(jungle[3].items, vec![74]);
        assert_eq!(jungle[3].operation_number, Some(3));
    }
}
