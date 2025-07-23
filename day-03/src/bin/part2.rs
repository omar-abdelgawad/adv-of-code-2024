use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");
    dbg!(filename);
    let input = get_part_1_input(filename);
    let ans = part_2(&input);
    dbg!(ans);
}
fn part_2(input: &str) -> i64 {
    let re = Regex::new(r"(?<op>mul\((?<X>\d{1,3}),(?<Y>\d{1,3})\)|do\(\)|don't\(\))").unwrap();
    let mut enabled = true;
    let mut sum = 0;
    for cap in re.captures_iter(input) {
        let op_token = &cap["op"];
        if op_token.starts_with("mul") {
            if enabled {
                let x: i64 = cap["X"].parse().unwrap();
                let y: i64 = cap["Y"].parse().unwrap();
                sum += x * y;
            }
        } else {
            enabled = !op_token.starts_with("don't");
        }
    }
    sum
}
fn get_part_1_input(filename: &str) -> String {
    read_to_string(filename).unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        let test_str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let expected_answer = 2 * 4 + 5 * 5 + 11 * 8 + 8 * 5;
        assert_eq!(part_2(test_str), expected_answer);
    }

    #[test]
    fn part_2_example() {
        let test_str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let expected_answer = 2 * 4 + 8 * 5;
        assert_eq!(part_2(test_str), expected_answer);
    }
}
