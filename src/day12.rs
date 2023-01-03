use crate::helpers::read_file;
use std::collections::{HashMap, HashSet};

/* Approach:
1. build adjacency matrix from input
2. DFS counting steps until "E", dead-end or exceeds steps from previous path
*/

type Grid = Vec<Vec<char>>;
type Pos = (usize, usize);

/*DFS Algorithm:
1. push neighbors onto stack
2. pop stack, add to visited, add to path_count
3. if E, dead-end or path_count >= min, reduce path_count by 1, repeat step 2
*/
/// Returns min path
fn dfs(start: Pos, grid: Grid) -> u32 {
    let mut stack: Vec<Pos> = vec![start];
    let mut path_count: u32 = 0;
    let mut min_path_count: u32 = u32::MAX;
    let visited: &mut HashSet<Pos> = &mut HashSet::new();
    let neighbors = |pos: Pos, visited: &HashSet<(usize, usize)>| -> Vec<Pos> {
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
        result
            .iter()
            .filter(|x| !visited.contains(x))
            .cloned()
            .collect()
    };

    let alphabet: HashMap<char, u32> =
        HashMap::from_iter((10..36).map(|n| (char::from_digit(n, 36).unwrap(), n)));
    let height_diff = |curr: &Pos, next: &Pos| -> i32 {
        if &grid[next.0][next.1].to_string() == "S"
            || &grid[curr.0][curr.1].to_string() == "S"
            || &grid[next.0][next.1].to_string() == "E"
            || &grid[curr.0][curr.1].to_string() == "E"
        {
            return 0;
        }
        if &grid[next.0][next.1].to_string() == "E" && &grid[curr.0][curr.1].to_string() == "z" {
            return 1;
        } else {
            alphabet[&grid[next.0][next.1]] as i32 - alphabet[&grid[curr.0][curr.1]] as i32
        }
    };

    while !stack.is_empty() {
        let curr = stack.pop().unwrap();
        if visited.insert(curr) {
            path_count += 1;
        }
        let n = neighbors(curr, visited)
            .iter()
            .filter(|x| height_diff(&curr, x) <= 1)
            .cloned()
            .collect::<Vec<Pos>>();
        let dead_end = n.is_empty();

        if grid[curr.0][curr.1] == 'E' {
            min_path_count = path_count.min(min_path_count);
            path_count -= 1;
            continue;
        }
        if dead_end || path_count >= min_path_count {
            path_count -= 1;
            continue;
        }
        stack = [stack, n].concat();
    }

    min_path_count
}

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

pub fn solution() -> (String, String) {
    let contents = read_file("/inputs/day12.txt");

    let grid = parser(&contents);
    let result1: usize = 0;
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
    fn dfs() {
        let (start, end, grid) = day12::parser(TEST_INPUT);
        assert_eq!(day12::dfs(start, grid), 31);
    }
}
