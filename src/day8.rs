use crate::helpers::read_file;

type Forest = Vec<Vec<TreeInForest>>;

#[derive(Debug, Clone)]
struct TreeInForest {
    height: u8,
    visible: bool,
}

impl TreeInForest {
    fn new(height: u8) -> TreeInForest {
        TreeInForest {
            height,
            visible: false,
        }
    }
}

fn parser<'a>(input: &'a str) -> Forest {
    let line_count = input.lines().count();
    let col_count = input.lines().nth(1).unwrap().chars().count();
    let mut forest: Forest = vec![
        vec![
            TreeInForest {
                height: 0,
                visible: false
            };
            line_count
        ];
        col_count
    ];

    let lines: Vec<&str> = input.lines().collect();
    let columns = |l: &'a str| l.chars().collect::<Vec<char>>();

    // 1st iteration: build the forest and find visibility from the left and top
    let mut max_top: Vec<u8> = vec![0; col_count];
    let mut max_left: u8 = 0;
    for (line, line_str) in lines.iter().enumerate() {
        for (col, char) in columns(line_str).iter().enumerate() {
            let mut tree = TreeInForest::new(char.to_digit(10).unwrap() as u8);
            tree.visible =
                col == 0 || line == 0 || tree.height > max_left || tree.height > max_top[col];
            max_left = if col == 0 { tree.height } else { max_left.max(tree.height) };
            max_top[col] = if line == 0 { tree.height } else { max_top[col].max(tree.height) };
            forest[col][line] = tree;
        }
    }

    // 2nd iteration: check visibility from the right and bottom
    let mut max_bottom: Vec<u8> = vec![0; line_count];
    let mut max_right = 0;
    for (line, line_str) in lines.iter().enumerate().rev() {
        for (col, _) in columns(line_str).iter().enumerate().rev() {
            let tree = &mut forest[col][line];
            tree.visible = col == col_count - 1
                || line == line_count - 1
                || tree.visible
                || tree.height > max_right
                || tree.height > max_bottom[col];
            max_right = if col == col_count - 1 { tree.height } else { max_right.max(tree.height) };
            max_bottom[col] = if line == line_count - 1 {
                tree.height
            } else {
                max_bottom[col].max(tree.height)
            };
        }
    }

    forest
}

fn count_visible_trees(forest: &Forest) -> usize {
    forest
        .iter()
        .map(|col| {
            col.iter()
                .map(|tree| if tree.visible { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

fn scenic_score(forest: &Forest, x: usize, y: usize) -> usize {
    let max_col = forest.len();
    let max_line = forest[0].len();
    let taller_or_equal = |t: &TreeInForest| t.height >= forest[x][y].height;
    if x == max_col - 1 || y == max_line - 1 || x == 0 || y == 0 {
        return 0;
    }
    let left = 1 + forest[..x]
        .iter()
        .rev()
        .position(|c| taller_or_equal(&c[y]))
        .unwrap_or(x - 1);
    let right = 1 + forest[x+1..]
        .iter()
        .position(|c| taller_or_equal(&c[y]))
        .unwrap_or(max_col - 2 - x);
    let up = 1 + forest[x][..y]
        .iter()
        .rev()
        .position(taller_or_equal)
        .unwrap_or(y - 1);
    let down = 1 + forest[x][y+1..]
        .iter()
        .position(taller_or_equal)
        .unwrap_or(max_line - 2 - y);

    left * right * up * down
}

pub fn solution() -> (String, String) {
    let contents = read_file("/inputs/day8.txt");

    let forest = parser(&contents);

    let result1: usize = count_visible_trees(&forest);
    
    let mut result2: usize = 0;
    for (x, col) in forest.iter().enumerate() {
        for (y, _) in col.iter().enumerate() {
            result2 = result2.max(scenic_score(&forest, x, y));
        }
    }
    

    return (result1.to_string(), result2.to_string());
}

#[cfg(test)]
mod tests {
    use crate::day8;

    const TEST_INPUT: &str = "\
30373
25512
65332
33549
35390";

    #[test]
    fn parser() {
        let forest = day8::parser(&TEST_INPUT);
        let mut printable_transpose = vec![vec![]; forest[0].len()];
        forest.iter().for_each(|col| {
            for (line, item) in col.iter().enumerate() {
                printable_transpose[line].push(item)
            }
        });
        for line in printable_transpose {
            println!(
                "{:?}",
                line.iter()
                    .map(|x| {
                        if x.visible {
                            format!("{} ", x.height)
                        } else {
                            format!("{}*", x.height)
                        }
                    })
                    .collect::<Vec<String>>()
            );
        }

        // assert positioning
        assert_eq!(forest[0][0].height, 3);
        assert_eq!(forest[0][1].height, 2);
        assert_eq!(forest[0][2].height, 6);
        assert_eq!(forest[0][3].height, 3);
        assert_eq!(forest[0][4].height, 3);
        assert_eq!(forest[1][0].height, 0);
        assert_eq!(forest[1][1].height, 5);
        assert_eq!(forest[2][2].height, 3);
        assert_eq!(forest[3][3].height, 4);
        assert_eq!(forest[3][4].height, 9);
        assert_eq!(forest[4][4].height, 0);

        // assert visibility
        assert_eq!(forest[0][0].visible, true);
        assert_eq!(forest[0][1].visible, true);
        assert_eq!(forest[0][2].visible, true);
        assert_eq!(forest[0][3].visible, true);
        assert_eq!(forest[0][4].visible, true);
        assert_eq!(forest[1][0].visible, true);
        assert_eq!(forest[1][1].visible, true);
        assert_eq!(forest[1][2].visible, true);
        assert_eq!(forest[1][3].visible, false);
        assert_eq!(forest[2][2].visible, false);
        assert_eq!(forest[3][2].visible, true);
        assert_eq!(forest[3][3].visible, false);
        assert_eq!(forest[3][4].visible, true);
        assert_eq!(forest[4][4].visible, true);
    }

    #[test]
    fn count_visible_trees() {
        let forest = day8::parser(&TEST_INPUT);
        assert_eq!(day8::count_visible_trees(&forest), 21);
    }

    #[test]
    fn scenic_score() {
        let forest = day8::parser(&TEST_INPUT);
        
        assert_eq!(day8::scenic_score(&forest, 2, 1), 4);
        assert_eq!(day8::scenic_score(&forest, 2, 2), 1);
        assert_eq!(day8::scenic_score(&forest, 2, 3), 8);
        assert_eq!(day8::scenic_score(&forest, 3, 1), 1);
        assert_eq!(day8::scenic_score(&forest, 3, 3), 3);
        assert_eq!(day8::scenic_score(&forest, 4, 2), 0);
        assert_eq!(day8::scenic_score(&forest, 4, 4), 0);
    }
}
