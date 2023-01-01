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
    idx: usize,
    items: Vec<u32>,
    operation_type: Option<OperationType>,
    operation_number: Option<u32>,
    test: MonkeyTest,
}

type Jungle = Vec<Monkey>;

trait Player {
    fn inspect_and_throw(&mut self) -> Result<(u32, usize), bool>;
}

impl Player for Monkey {
    fn inspect_and_throw(&mut self) -> Result<(u32, usize), bool> {
        if self.items.len() == 0 {
            return Err(true);
        }

        let mut item = self.items.remove(0);
        item = match self.operation_type.as_mut().unwrap() {
            OperationType::Multiplication => match self.operation_number {
                Some(n) => n * item,
                None => item * item,
            },
            OperationType::Addition => match self.operation_number {
                Some(n) => n + item,
                None => item + item,
            },
        };
        item = item / 3;
        let throw_to = match item % self.test.divisible_by == 0 {
            true => self.test.true_monkey,
            false => self.test.false_monkey,
        };
        Ok((item, throw_to))
    }
}

struct KeepAwayGame {
    monkeys: Jungle,
}

impl KeepAwayGame {
    fn start(&mut self, rounds: usize) {
        let num_monkeys = self.monkeys.len();
        let turns = rounds * num_monkeys; 
        for turn in 0..turns {
            unsafe {
                let monkey = &mut self.monkeys[turn % num_monkeys] as *mut Monkey;
                while let Ok((item, throw_to_idx)) = (*monkey).inspect_and_throw() {
                    self.monkeys[throw_to_idx].items.push(item);
                }
            }
        }
    }
}

fn parser(input: &str) -> Jungle {
    let mut monkeys: Jungle = Vec::new();
    let mut idx: usize = 0;
    for line in input.lines() {
        if line.starts_with("Monkey") {
            monkeys.push(Monkey {
                idx: idx,
                ..Monkey::default()
            });
            idx += 1;
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
        assert_eq!(
            jungle[0].operation_type,
            Some(day11::OperationType::Multiplication)
        );
        assert_eq!(jungle[0].operation_number, Some(19));

        assert_eq!(jungle[2].operation_number, None);
        assert_eq!(jungle[2].test.divisible_by, 13);
        assert_eq!(jungle[2].test.true_monkey, 1);
        assert_eq!(jungle[2].test.false_monkey, 3);

        assert_eq!(jungle[3].items, vec![74]);
        assert_eq!(jungle[3].operation_number, Some(3));
    }

    #[test]
    fn play_twenty_rounds() {
        let jungle = day11::parser(TEST_INPUT);
        let game = &mut day11::KeepAwayGame { monkeys: jungle };
        
        println!("start: {:?}", game.monkeys);
        game.start(20);

        
        assert_eq!(game.monkeys[0].items, vec![10, 12, 14, 26, 34]);
        assert_eq!(game.monkeys[1].items, vec![245, 93, 53, 199, 115]);
        assert_eq!(game.monkeys[2].items, vec![]);
        assert_eq!(game.monkeys[3].items, vec![]);
    }
}
