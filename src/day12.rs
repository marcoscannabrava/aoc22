use crate::helpers::read_file;
use once_cell::sync::Lazy;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

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

fn get_low_points(grid: &Grid) -> Vec<Pos> {
    let mut result = Vec::new();
    grid.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, char)| {
            if char.to_string() == "a" {
                result.push((y, x))
            }
        })
    });
    result
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
    let curr_elevation;
    let next_elevation;
    match grid[curr.0][curr.1] {
        'S' => curr_elevation = 'a',
        'E' => curr_elevation = 'z',
        _ => curr_elevation = grid[curr.0][curr.1],
    }
    match grid[next.0][next.1] {
        'S' => next_elevation = 'a',
        'E' => next_elevation = 'z',
        _ => next_elevation = grid[next.0][next.1],
    }
    ALPHABET[&next_elevation] as i32 - ALPHABET[&curr_elevation] as i32
}

fn dijkstra(grid: &Grid, start: &Pos, end: &Pos) -> usize {
    let mut visited = HashMap::new();
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, *start)));
    while let Some(Reverse((steps, curr))) = heap.pop() {
        if &curr == end {
            return steps;
        }
        if visited.contains_key(&curr) {
            continue;
        }
        visited.insert(curr, steps);
        // println!("{}: {:?} {:?}", steps, curr, grid[curr.0][curr.1]);
        for next in neighbors(&grid, &curr) {
            if height_diff(&grid, &curr, &next) <= 1 {
                heap.push(Reverse((steps + 1, next)));
            }
        }
    }
    0
}

pub fn solution() -> (String, String) {
    let contents = read_file("/inputs/day12.txt");

    let (start, end, grid) = parser(&contents);

    let result1: usize = dijkstra(&grid, &start, &end);

    let low_points = get_low_points(&grid);
    let result2: usize = low_points
        .iter()
        .map(|start_pos| {
            match dijkstra(&grid, &start_pos, &end) {
                0 => usize::MAX,
                x => x,
            }
        })
        .min()
        .unwrap();

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
        assert_eq!(day12::height_diff(&grid, &(1, 5), &end), 2);
    }

    #[test]
    fn dijkstra() {
        let (start, end, grid) = day12::parser(TEST_INPUT);
        assert_eq!(day12::dijkstra(&grid, &start, &end), 31);
    }

    const TEST_INPUT_TWO: &str = "\
Szzzzzzj
zabqfhEi
abcdefgh
accczzxk";

    #[test]
    fn dijkstra_two() {
        let (start, end, grid) = day12::parser(TEST_INPUT_TWO);
        assert_eq!(day12::dijkstra(&grid, &start, &end), 0);
    }

    #[test]
    fn get_low_points() {
        let (_, _, grid) = day12::parser(TEST_INPUT);
        assert_eq!(
            day12::get_low_points(&grid),
            vec![(0, 1), (1, 0), (2, 0), (3, 0), (4, 0)]
        );
    }
}
