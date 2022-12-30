use std::{collections::HashMap, hash::Hash};

use crate::helpers::read_file;

/// (number of cycles, register value increase/decrease)
type Operation = (usize, i32);

/// Vec<(cycle index, signal strength)>
type SignalWave = Vec<(usize, i32)>;

#[derive(Debug)]
struct Register {
    operations: Vec<Operation>,
    signal_wave: SignalWave,
}

impl Register {
    fn parse(ops: &str) -> Register {
        let mut register = Register {
            operations: ops
                .lines()
                .map(|op_str| match op_str {
                    "noop" => (1, 0),
                    _ => (2, op_str.split_once(" ").unwrap().1.parse::<i32>().unwrap()),
                })
                .collect(),
            signal_wave: Vec::new(),
        };
        register.signal_wave = register.get_signal_wave();
        register
    }

    /// Return SignalWave for this register
    /// Example:
    /// noop     -> (1,1), (2,1)
    /// addx 3   -> ..., (4,4)
    /// addx -5  -> ..., (6,-1)
    fn get_signal_wave(&self) -> SignalWave {
        let mut signal_wave: SignalWave = vec![(1, 1)];

        for (idx, (op_cycles, op)) in self.operations.iter().enumerate() {
            let (cycles, strength) = signal_wave[idx];
            signal_wave.push((cycles + op_cycles, strength + op))
        }
        signal_wave
    }

    fn get_x_at_cycles(&self, cycles: Vec<usize>) -> Vec<(usize, i32)> {
        let mut sorted_cycles = cycles.clone();
        sorted_cycles.sort();

        let mut i = 0;
        let mut result = Vec::new();
        for (j, (curr_cycle, _)) in self.signal_wave.iter().enumerate() {
            if i >= sorted_cycles.len() {
                break;
            }
            if curr_cycle > &sorted_cycles[i] {
                result.push((sorted_cycles[i], self.signal_wave[j - 1].1));
                i += 1;
            }
        }
        result
    }

    fn get_x_per_cycle(&self) -> HashMap<usize, i32> {
        let mut v: HashMap<usize, i32> = HashMap::new();
        let mut i = 1;
        for (cycle, x) in &self.signal_wave {
            while v.len() <= *cycle {
                v.insert(i, x.clone());
                i += 1;
            }
        }
        v
    }


    /// each cycle draw a pixel: either "#" or "." if it's within the sprite position
    /// sprite is 3 pixels wide and positioned at current register
    fn draw_screen(&self) -> String {
        let mut result: Vec<String> = vec![];
        let sprite_positions = self.get_x_per_cycle();
        let max_cycle = sprite_positions.keys().max().unwrap();

        for cycle in 1..(max_cycle - 1) {
            let sprite_pos: usize = sprite_positions.get(&cycle).unwrap().clone() as usize;
            let crt_pos = cycle % 40;
            if crt_pos >= sprite_pos && crt_pos <= (sprite_pos + 2) {
                result.push("#".to_owned());
            } else {
                result.push(".".to_owned());
            }
            if cycle > 1 && crt_pos == 0 {
                result.push("\n".to_owned());
            }
        }
        result.join("")
    }
}

pub fn solution() -> (String, String) {
    let contents = read_file("/inputs/day10.txt");

    let register = Register::parse(&contents);

    let max_cycle = register.signal_wave.last().unwrap().0;
    let cycles: Vec<usize> = (20..max_cycle)
        .filter(|cycle| (cycle - 20) % 40 == 0)
        .collect();
    let x_at_cycles = register.get_x_at_cycles(cycles);
    let result1: i32 = x_at_cycles
        .iter()
        .map(|(cycle, signal)| (*cycle as i32) * signal)
        .sum();

    let result2: String = vec!["\n".to_owned(), register.draw_screen()].join("");

    return (result1.to_string(), result2);
}

#[cfg(test)]
mod tests {
    use crate::day10;
    use crate::day10_test::PARSED_OPERATIONS;

    #[test]
    fn parse() {
        let test_input: String = day10::read_file("/inputs/day10_test.txt");
        let register = day10::Register::parse(&test_input);
        assert_eq!(register.operations, PARSED_OPERATIONS)
    }

    #[test]
    fn get_signal_wave() {
        let test_input: String = day10::read_file("/inputs/day10_test.txt");
        let register = day10::Register::parse(&test_input);

        let expected_x_at_cycles: Vec<(usize, i32)> = vec![
            (20, 21),
            (60, 19),
            (100, 18),
            (140, 21),
            (180, 16),
            (220, 18),
        ];

        assert_eq!(
            register.get_x_at_cycles(vec![20, 60, 100, 140, 180, 220]),
            expected_x_at_cycles
        );
    }

    #[test]
    fn get_x_per_cycle() {
        let test_input: String = day10::read_file("/inputs/day10_test.txt");
        let register = day10::Register::parse(&test_input);
        let x_per_cycles = register.get_x_per_cycle();
        let mut sorted_x_per_cycles = x_per_cycles.iter().collect::<Vec<_>>();
        sorted_x_per_cycles.sort_by(|(a, _), (b, _)| a.cmp(b));
        for (cycle, x) in sorted_x_per_cycles {
            println!("cycle: {}, x: {}", cycle, x)
        }
        assert_eq!(x_per_cycles.get(&20), Some(&21));
        assert_eq!(x_per_cycles.get(&60), Some(&19));
        assert_eq!(x_per_cycles.get(&100), Some(&18));
        assert_eq!(x_per_cycles.get(&140), Some(&21));
        assert_eq!(x_per_cycles.get(&180), Some(&16));
        assert_eq!(x_per_cycles.get(&220), Some(&18));
    }

    #[test]
    fn draw_screen() {
        let test_input: String = day10::read_file("/inputs/day10_test.txt");
        let register = day10::Register::parse(&test_input);
        let screen = register.draw_screen();
        println!("{}", screen);
    }

    #[test]
    fn part_1() {
        let test_input: String = day10::read_file("/inputs/day10_test.txt");
        let register = day10::Register::parse(&test_input);

        let max_cycle = register.signal_wave.last().unwrap().0;
        println!("{:?}", max_cycle);
        
        let cycles: Vec<usize> = (20..max_cycle)
            .filter(|cycle| (cycle - 20) % 40 == 0)
            .collect();

        let x_at_cycles = register.get_x_at_cycles(cycles);
        println!("{:?}", x_at_cycles);

        let result1: Vec<i32> = x_at_cycles
            .iter()
            .map(|(cycle, signal)| (*cycle as i32) * signal).collect();
        println!("{:?}", result1);

        assert_eq!(result1.iter().sum::<i32>(), 13140);
    }
}
