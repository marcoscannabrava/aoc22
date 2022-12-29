use crate::helpers::read_file;
use std::{collections::HashSet, fmt};

type Position = (i32, i32);
type Knots = Vec<Knot>;

#[derive(Clone)]
struct Knot {
    pos: Position,
    path_set: HashSet<Position>,
}

impl fmt::Debug for Knot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Knot @ {:?}", self.pos)
    }
}

#[derive(Debug, Clone)]
struct Rope {
    knots: Knots,
}

impl Rope {
    fn start(knots: u32) -> Rope {
        let start = (0, 0);
        Rope {
            knots: vec![
                Knot {
                    pos: start,
                    path_set: HashSet::from([start]),
                };
                knots as usize
            ],
        }
    }

    fn move_head(&mut self, cmd: &str) {
        let direction = cmd.chars().nth(0).unwrap();
        let distance = cmd[2..].parse::<i32>().unwrap();
        for _ in 0..distance {
            match direction {
                'U' => self.knots[0].pos.1 += 1,
                'D' => self.knots[0].pos.1 -= 1,
                'L' => self.knots[0].pos.0 -= 1,
                'R' => self.knots[0].pos.0 += 1,
                _ => panic!("Invalid direction"),
            }
            self.pull_tail();
        }
    }

    fn pull_tail(&mut self) {
        for i in 1..self.knots.len() {
            let (left, right) = self.knots.split_at_mut(i);
            let head = left.last().unwrap();
            let tail = &mut right[0];
            let (dx, dy) = delta_pos(tail.pos, head.pos);
            if dy.abs() > 1 || dx.abs() > 1 {
                tail.pos.0 += 1 * dx.signum();
                tail.pos.1 += 1 * dy.signum();
            }
            tail.path_set.insert(tail.pos);
        };
    }

    fn tail(&self) -> Option<&Knot> {
        self.knots.last()
    }
}

fn delta_pos(pos1: Position, pos2: Position) -> Position {
    (pos2.0 - pos1.0, pos2.1 - pos1.1)
}

pub fn solution() -> (String, String) {
    let contents = read_file("/inputs/day9.txt");

    let mut rope = Rope::start(2);
    let mut rope_with_ten_knots = Rope::start(10);
    for cmd in contents.lines() {
        rope.move_head(cmd);
        rope_with_ten_knots.move_head(cmd);
    }
    let result1: usize = rope.tail().unwrap().path_set.len();
    let result2: usize = rope_with_ten_knots.tail().unwrap().path_set.len();

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
        let mut rope = day9::Rope::start(2);
        for cmd in TEST_INPUT.lines() {
            rope.move_head(cmd);
        }
        assert_eq!(rope.knots[0].pos, (2, 2));
        assert_eq!(rope.tail().unwrap().pos, (1, 2));
        assert_eq!(rope.tail().unwrap().path_set.len(), 13);
    }

    #[test]
    fn rope_part2() {
        let mut rope = day9::Rope::start(10);
        for cmd in TEST_INPUT_TWO.lines() {
            rope.move_head(cmd);
        }
        let last_knot = rope.tail().unwrap();
        assert_eq!(rope.knots[0].pos, (-11, 15));
        assert_eq!(last_knot.pos, (-11, 6));
        assert_eq!(last_knot.path_set.len(), 36);
    }
}
