use crate::helpers::read_file;
use std::collections::HashSet;

type Position = (i32, i32);
type LinkedRope = Option<Box<Rope>>;

#[derive(Debug, Clone)]
struct Rope {
    head: Position,
    tail: Position,
    tail_path_set: HashSet<Position>,
    tied_to: LinkedRope,
}

impl Rope {
    fn start() -> Rope {
        let start = (0, 0);
        Rope {
            head: start,
            tail: start,
            tail_path_set: HashSet::from([start]),
            tied_to: None,
        }
    }

    fn start_tied_knots(knots: u32) -> Rope {
        let mut rope_head = Rope::start();
        let mut curr_rope = &mut rope_head;
        for _ in 0..knots {
            let new_rope = Rope::start();
            curr_rope.tie_to(new_rope);
            curr_rope = curr_rope.tied_to.as_mut().unwrap();
        }
        rope_head
    }

    fn tie_to(&mut self, rope: Rope) {
        self.tied_to = Some(Box::new(rope));
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
            self.pull_tail();
        }
    }

    fn pull_tail(&mut self) {
        let (dx, dy) = delta_pos(self.tail, self.head);
        if dy.abs() > 1 || dx.abs() > 1 {
            self.tail.0 += 1 * dx.signum();
            self.tail.1 += 1 * dy.signum();
        }
        self.tail_path_set.insert(self.tail);

        let mut curr_knot = self;
        let mut next_knot = curr_knot.tied_to.as_mut();
        while next_knot.is_some() {
            let next = next_knot.unwrap();
            next.head = curr_knot.tail;
            next.pull_tail();
            curr_knot = next.as_mut();
            next_knot = curr_knot.tied_to.as_mut();
        }
    }

    /// helper function get last knot and print out all of the knots in a rope
    fn get_last_knot(&self, print: bool) -> &Rope {
        if print {
            println!("All of the rope's knots:");
        }
        let mut last_knot = self;
        let mut i = 1;
        while last_knot.tied_to.is_some() {
            if print {
                println!(
                    "{} head: {:?}, tail: {:?}, tail_path_len: {:?}",
                    i, last_knot.head, last_knot.tail, last_knot.tail_path_set.len()
                );
            }
            last_knot = last_knot.tied_to.as_ref().unwrap();
            i += 1;
        }
        last_knot
    }
}

fn delta_pos(pos1: Position, pos2: Position) -> Position {
    (pos2.0 - pos1.0, pos2.1 - pos1.1)
}

pub fn solution() -> (String, String) {
    let contents = read_file("/inputs/day9.txt");

    let mut rope = Rope::start();
    let mut rope_with_ten_knots = Rope::start_tied_knots(10);
    for cmd in contents.lines() {
        rope.move_head(cmd);
        rope_with_ten_knots.move_head(cmd);
    }
    let result1: usize = rope.tail_path_set.len();
    let result2: usize = rope_with_ten_knots.get_last_knot(false).tail_path_set.len();

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
        let mut rope = day9::Rope::start_tied_knots(10);
        println!("Moving the head!");
        for cmd in TEST_INPUT_TWO.lines() {
            println!("cmd: {}", cmd);
            rope.move_head(cmd);
            rope.get_last_knot(true);
        }
        println!("Finished moving.");
        let last_knot = rope.get_last_knot(true);
        println!(
            "last knot: head: {:?}, tail: {:?}",
            last_knot.head, last_knot.tail
        );
        assert_eq!(rope.head, (-11, 15));
        assert_eq!(last_knot.head, (-11, 6));
        assert_eq!(last_knot.tail_path_set.len(), 36);
    }
}
