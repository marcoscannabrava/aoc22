use crate::helpers::read_file;
use std::{collections::HashSet, rc::Rc, cell::RefCell};

type Position = (i32, i32);
type LinkedRope = Option<Rc<RefCell<Rope>>>;

#[derive(Debug, Clone)]
struct Rope {
    knot: Position,
    path_set: HashSet<Position>,
    tied_to: LinkedRope,
    tail: LinkedRope,
}

impl Rope {
    unsafe fn start(knots: u32) -> Rope {
        let start = (0, 0);
        let mut rope_head = Rope {
            knot: start,
            path_set: HashSet::from([start]),
            tied_to: None,
            tail: None,
        };
        let mut curr_rope = &mut rope_head as *mut Rope;
        for _ in 0..knots {
            let new_rope = Rope {
                knot: start,
                path_set: HashSet::from([start]),
                tied_to: None,
                tail: None,
            };
            (*curr_rope).tie_to(new_rope);
            // if i == knots - 1 {
            //     rope_head.tail = Some(curr_rope.tied_to.as_ref().unwrap());
            // }
            curr_rope = (*curr_rope).tied_to.as_ref().unwrap().as_ptr();
        }
        rope_head
    }

    fn tie_to(&mut self, rope: Rope) {
        self.tied_to = Some(Rc::new(RefCell::new(rope)));
    }

    fn move_head(&mut self, cmd: &str) {
        let direction = cmd.chars().nth(0).unwrap();
        let distance = cmd[2..].parse::<i32>().unwrap();
        for _ in 0..distance {
            match direction {
                'U' => self.knot.1 += 1,
                'D' => self.knot.1 -= 1,
                'L' => self.knot.0 -= 1,
                'R' => self.knot.0 += 1,
                _ => panic!("Invalid direction"),
            }
            self.pull_tail();
        }
    }

    fn pull_tail(self) { 
        let mut curr = Rc::new(RefCell::new(self));
        let mut maybe_next = curr.borrow().tied_to;
        while maybe_next.is_some() {
            let next = maybe_next.unwrap();
            let (dx, dy) = delta_pos(next.borrow().knot, curr.borrow().knot);
            if dy.abs() > 1 || dx.abs() > 1 {
                next.borrow().knot.0 += 1 * dx.signum();
                next.borrow().knot.1 += 1 * dy.signum();
            }
            curr.borrow().path_set.insert(curr.borrow().knot);
            curr = next;
            maybe_next = curr.borrow().tied_to;
        }
    }

    /// helper function get last knot and print out all of the knots in a rope
    fn get_tail(self, print: bool) -> Rc<RefCell<Rope>> {
        if print {
            println!("All of the rope's knots:");
        }
        let mut last_knot = Rc::new(RefCell::new(self));
        let mut i = 1;
        while last_knot.borrow().tied_to.is_some() {
            if print {
                println!(
                    "{} pos: {:?}, path_len: {:?}",
                    i,
                    last_knot.borrow().knot,
                    last_knot.borrow().path_set.len()
                );
            }
            last_knot = last_knot.borrow().tied_to.unwrap();
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

    let mut rope = Rope::start(1);
    let mut rope_with_ten_knots = Rope::start(10);
    for cmd in contents.lines() {
        rope.move_head(cmd);
        rope_with_ten_knots.move_head(cmd);
    }
    let result1: usize = rope.get_tail(false).borrow().path_set.len();
    let result2: usize = rope_with_ten_knots.get_tail(false).borrow().path_set.len();

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
        let mut rope = day9::Rope::start(1);
        for cmd in TEST_INPUT.lines() {
            rope.move_head(cmd);
        }
        assert_eq!(rope.knot, (2, 2));
        assert_eq!(rope.get_tail(false).borrow().knot, (1, 2));
        assert_eq!(rope.get_tail(false).borrow().path_set.len(), 13);
    }

    #[test]
    fn rope_part2() {
        let mut rope = day9::Rope::start(10);
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
