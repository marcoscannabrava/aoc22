use crate::helpers::read_file;

#[derive(PartialEq, Eq, Debug)]
enum PacketData {
    Num(u8),
    List(Vec<PacketData>),
}

type PacketDataInput = Vec<(Vec<PacketData>, Vec<PacketData>)>;

fn parse_packet_data(s: &str) -> Vec<PacketData> {
    // approaches: recursive and non recursive solutions
    // if s.starts_with('[') {
    //     parse_packet_data(s.trim_start_matches('['), 1)
    // }
    // parse_packet_data(s.trim_start_matches('['), 1)
    // s.chars().take_while(predicate)
    Vec::new()
}

fn parser(input: &str) -> PacketDataInput {
    input
        .trim()
        .split("\n\n")
        .map(|pair| {
            let mut lines = pair.lines();
            let left = lines.next().unwrap();
            let right = lines.next().unwrap();

            (parse_packet_data(left), parse_packet_data(right))
        })
        .collect::<PacketDataInput>()
}

pub fn solution() -> (String, String) {
    let contents = read_file("/inputs/day13.txt");

    let result1: usize = 0;
    let result2: usize = 0;
    return (result1.to_string(), result2.to_string());
}

#[cfg(test)]
mod tests {
    use crate::day13;

    const TEST_INPUT: &str = "
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    #[test]
    fn parse_packet_data() {
        let input = "[[1],[2,3,4]]";
        // let t = input.trim().
        // println!("{:?}", t)
        // let i = "1]";
        // let t = i.trim_end_matches("]");
        // println!("{:?}", t);

        let result = day13::parse_packet_data(input);
        println!("{:?}", result);
        assert_eq!(result.len(), 2);
    }
    #[test]
    fn parser() {
        let result = day13::parser(TEST_INPUT);
        assert_eq!(result.len(), 8);
    }
}
