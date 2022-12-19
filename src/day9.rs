use crate::helpers::read_file;
use std::collections::HashSet;

type Position = (i32, i32);
type LinkedRope = Option<Box<Rope>>;

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
            follow_knot(self.tail, self.head);

            // TODO: remove tied_to and make tail an Optional LinkedRope?
            self.tail_path_set.insert(self.tail);
            let next_knot = self.tied_to.as_mut();
            match next_knot {
                Some(knot) => {
                    follow_knot(knot.head, self.tail);
                    follow_knot(knot.tail, knot.head);
                }
                None => (),
                
            }
        }
    }
}
fn follow_knot(mut follower: Position, mover: Position) {
    let (dx, dy) = delta_pos(follower, mover);
    if dy.abs() > 1 || dx.abs() > 1 {
        follower.0 += 1 * dx.signum();
        follower.1 += 1 * dy.signum();
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
        let mut rope = day9::Rope::start_tied_knots(10);
        for cmd in TEST_INPUT_TWO.lines() {
            rope.move_head(cmd);
        }
        assert_eq!(rope.head, (0, 15));
        assert_eq!(rope.tail, (0, 6));
        assert_eq!(rope.tail_path_set.len(), 36);
    }
}
