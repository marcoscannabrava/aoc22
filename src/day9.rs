use crate::helpers::read_file;
use std::collections::HashSet;

type Position = (i32, i32);

struct Rope {
    head: Position,
    tail: Position,
    tail_path_set: HashSet<Position>,
}

impl Rope {
    fn start() -> Rope {
        let start = (0, 0);
        Rope {
            head: start,
            tail: start,
            tail_path_set: HashSet::from([start]),
        }
    }
    fn move_head(&mut self, cmd: &str) {
        let direction = cmd.chars().nth(0).unwrap();
        let distance = cmd[2..].parse::<i32>().unwrap();
        for _ in 0..distance {
            match direction {
                'U' => self.head.1 += 1,
                'D' => self.head.1 -= 1,
                'L' => self.head.0 -= 1,
                'R' => self.head.0 += 1,
                _ => panic!("Invalid direction"),
            }
            self.move_tail();
        }
    }
    fn move_tail(&mut self) {
        let (dx, dy) = delta_pos(self.tail, self.head);
        if dy.abs() > 1 || dx.abs() > 1 {
            self.tail.0 += 1 * dx.signum();
            self.tail.1 += 1 * dy.signum();
            self.tail_path_set.insert(self.tail);
        }
    }
}

fn delta_pos(pos1: Position, pos2: Position) -> Position {
    (pos2.0 - pos1.0, pos2.1 - pos1.1)
}

pub fn solution() -> (String, String) {
    let contents = read_file("/inputs/day9.txt");

    let mut rope = Rope::start();
    for cmd in contents.lines() {
        rope.move_head(cmd);
    }
    let result1: usize = rope.tail_path_set.len();
    let result2: i32 = 0;

    return (result1.to_string(), result2.to_string());
}

#[cfg(test)]
mod tests {
    use crate::day9;

    const TEST_INPUT: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const TEST_INPUT_TWO: &str = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn rope_part1() {
        let mut rope = day9::Rope::start();
        for cmd in TEST_INPUT.lines() {
            rope.move_head(cmd);
        }
        assert_eq!(rope.head, (2, 2));
        assert_eq!(rope.tail, (1, 2));
        assert_eq!(rope.tail_path_set.len(), 13);
    }

    #[test]
    fn rope_part2() {
        let mut rope = day9::Rope::start();
        for cmd in TEST_INPUT.lines() {
            rope.move_head(cmd);
        }
        assert_eq!(rope.head, (2, 2));
        assert_eq!(rope.tail, (1, 2));
        assert_eq!(rope.tail_path_set.len(), 13);
    }
}
