/* Approach:
1. build adjacency matrix from input
2. DFS counting steps until "E", dead-end or exceeds steps from previous path
*/
use crate::helpers::read_file;

type Grid = Vec<Vec<char>>;
type Pos = (usize, usize);

fn dfs(grid: Grid) {}

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
}
