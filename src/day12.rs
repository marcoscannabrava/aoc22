use crate::helpers::read_file;
use once_cell::sync::Lazy;

use std::{collections::{BinaryHeap, HashMap}, cmp::min};

type Grid = Vec<Vec<char>>;
type Pos = (usize, usize);

static ALPHABET: Lazy<HashMap<char, u32>> =
    Lazy::new(|| HashMap::from_iter((10..36).map(|n| (char::from_digit(n, 36).unwrap(), n))));

/// Returns (Start, End, Grid) tuple
fn parser(input: &str) -> (Pos, Pos, Grid) {
    let mut start: Pos = (0, 0);
    let mut end: Pos = (0, 0);
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c.to_string() == "S" {
                        start = (y, x)
                    }
                    if c.to_string() == "E" {
                        end = (y, x)
                    }
                    c
                })
                .collect()
        })
        .collect();
    (start, end, grid)
}

/// Returns list of neighbors of a given position
fn neighbors(grid: &Grid, pos: &Pos) -> Vec<Pos> {
    let mut result = Vec::new();
    if pos.0 > 0 {
        result.push((pos.0 - 1, pos.1));
    }
    if pos.1 > 0 {
        result.push((pos.0, pos.1 - 1));
    }
    if pos.0 + 1 < grid.len() {
        result.push((pos.0 + 1, pos.1));
    }
    if pos.1 + 1 < grid[0].len() {
        result.push((pos.0, pos.1 + 1));
    }
    result.iter().cloned().collect()
}

fn height_diff(grid: &Grid, curr: &Pos, next: &Pos) -> i32 {
    if grid[next.0][next.1].to_string() == "S"
        || grid[curr.0][curr.1].to_string() == "S"
        || grid[next.0][next.1].to_string() == "E" // TODO: can we hop on E from any char?
        || grid[curr.0][curr.1].to_string() == "E"
    {
        return 0;
    }
    if grid[next.0][next.1].to_string() == "E" && grid[curr.0][curr.1].to_string() == "z" {
        return 1;
    } else {
        ALPHABET[&grid[next.0][next.1]] as i32 - ALPHABET[&grid[curr.0][curr.1]] as i32
    }
}

fn dijkstra(grid: &Grid, start: &Pos, end: &Pos) -> usize {
    let mut visited = HashMap::new();
    let mut stack = BinaryHeap::new();
    stack.push((0, start.clone()));
    while let Some((steps, curr)) = stack.pop() {
        if &curr == end {
            return steps;
        }
        visited
            .entry(curr)
            .and_modify(|e| {
                *e = min(*e, steps);
            })
            .or_insert(steps);
        if visited.contains_key(&curr) {
            continue;
        }
        for next in neighbors(&grid, &curr) {
            if height_diff(&grid, &curr, &next) <= 1 {
                stack.push((steps + 1, next));
            }
        }
    }
    0
}

pub fn solution() -> (String, String) {
    let contents = read_file("/inputs/day12.txt");

    let (start, end, grid) = parser(&contents);

    let result1: usize = dijkstra(&grid, &start, &end);
    let result2: usize = 0;

    return (result1.to_string(), result2.to_string());
}

#[cfg(test)]
mod tests {
    use crate::day12;

    const TEST_INPUT: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn parser() {
        let (start, end, grid) = day12::parser(TEST_INPUT);
        assert_eq!(start, (0, 0));
        assert_eq!(end, (2, 5));
        assert_eq!(grid.len(), 5);
        assert_eq!(grid[0].len(), 8);
        assert_eq!(grid[0][0], 'S');
        assert_eq!(grid[2][1], 'c');
        assert_eq!(grid[2][1], 'c');
        assert_eq!(grid[2][2], 'c');
        assert_eq!(grid[2][3], 's');
        assert_eq!(grid[2][4], 'z');
        assert_eq!(grid[2][5], 'E');
        assert_eq!(grid[2][7], 'k');
        assert_eq!(grid[3][7], 'j');
        assert_eq!(grid[4][7], 'i');
    }

    #[test]
    fn alphabet() {
        let mut alphabet = day12::ALPHABET.iter().collect::<Vec<(&char, &u32)>>();
        alphabet.sort_by(|a, b| a.1.cmp(b.1));
        println!("{:?}", alphabet);
        assert_eq!(alphabet[0].0, &'a');
        assert_eq!(alphabet[1].0, &'b');
        assert_eq!(alphabet.last().unwrap().0, &'z');
    }

    #[test]
    fn neighbors() {
        let (start, _, grid) = day12::parser(TEST_INPUT);
        assert_eq!(day12::neighbors(&grid, &start), vec![(1, 0), (0, 1)]);
        assert_eq!(
            day12::neighbors(&grid, &(0, 1)),
            vec![(0, 0), (1, 1), (0, 2)]
        );
        assert_eq!(
            day12::neighbors(&grid, &(3, 3)),
            vec![(2, 3), (3, 2), (4, 3), (3, 4)]
        );
    }

    #[test]
    fn height_diff() {
        let (start, end, grid) = day12::parser(TEST_INPUT);
        assert_eq!(day12::height_diff(&grid, &start, &(1, 0)), 0);
        assert_eq!(day12::height_diff(&grid, &start, &(0, 1)), 0);
        assert_eq!(day12::height_diff(&grid, &end, &(1, 1)), 0);
    }

    #[test]
    fn dijkstra() {
        let (start, end, grid) = day12::parser(TEST_INPUT);
        assert_eq!(day12::dijkstra(&grid, &start, &end), 31);
    }
}
