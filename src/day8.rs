use crate::helpers::read_file;

type Forest = Vec<Vec<TreeInForest>>;
struct TreeInForest {
    height: u8,
    visible: bool,
}

fn parser(input: &str) -> Forest {
    let mut max_left = 0;
    let mut max_right = 0;
    let mut max_top = 0;
    let mut max_bottom = 0;
    let len_lines = input.lines().count();
    let len_cols = input.lines().next().unwrap().chars().count();

    for col in 0..len_cols {
        for line in 0..len_lines {
            
        }
    }
    // let mut tree = TreeInForest {
    //     height: char.to_digit(10).unwrap() as u8,
    //     visible: false,
    // };

    vec![vec![TreeInForest { height: 0, visible: false }]]
}


pub fn solution() -> (String, String) {
    let contents = read_file("/inputs/day8.txt");

    let forest = parser(&contents);

    let result1: usize = 0;
    let result2: usize = 0;

    return (result1.to_string(), result2.to_string());
}

#[cfg(test)]
mod tests {

    const TEST_INPUT: &str = "
30373
25512
65332
33549
35390";

    #[test]
    fn build() {
        assert_eq!(0, 0);
    }
}
