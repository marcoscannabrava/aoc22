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

    // Example:
    // noop     -> (1,1), (2,1)
    // addx 3   -> ..., (4,4)
    // addx -5  -> ..., (6,-1)
    fn get_signal_wave(&self) -> SignalWave {
        let mut signal_wave: SignalWave = vec![(1, 1)];

        for (idx, (op_cycles, op)) in self.operations.iter().enumerate() {
            let (cycles, strength) = signal_wave[idx];
            signal_wave.push((cycles + op_cycles, strength + op))
        }
        signal_wave
    }

    fn get_x_at_cycle(&self, cycle: &usize) -> i32 {
        self.signal_wave
            .iter()
            .rev()
            .find(|(idx, _)| idx <= cycle)
            .unwrap()
            .1
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

    let result2: usize = 0;

    return (result1.to_string(), result2.to_string());
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
        assert_eq!(register.get_x_at_cycle(&20), 21);
        assert_eq!(register.get_x_at_cycle(&60), 19);
        assert_eq!(register.get_x_at_cycle(&100), 18);
        assert_eq!(register.get_x_at_cycle(&140), 21);
        assert_eq!(register.get_x_at_cycle(&180), 16);
        assert_eq!(register.get_x_at_cycle(&220), 18);

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
