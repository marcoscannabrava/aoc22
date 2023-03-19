use crate::helpers::read_file;


pub fn solution() -> (String, String) {
    let contents = read_file("/inputs/day13.txt");

    let result1: usize = 0;
    let result2: usize = 0;
    return (result1.to_string(), result2.to_string());
}

#[cfg(test)]
mod tests {
    use crate::day13;

    #[test]
    fn test1() {
        assert_eq!(day13::solution(), ("0".to_owned(), "0".to_owned()));
    }
}
