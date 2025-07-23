use std::{
    fs::File,
    io::{self, BufRead},
};
type Report = Vec<i64>;

fn main() {
    let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");
    dbg!(filename);
    let reports = get_part_1_input(filename);
    let ans = part_1(reports);
    dbg!(ans);
}
fn part_1(reports: Vec<Report>) -> i64 {
    reports.iter().map(|report| is_safe(report) as i64).sum()
}
fn is_safe(report: &Report) -> bool {
    if report.len() <= 1 {
        return true;
    }
    let mut increasing = false;
    let mut decreasing = false;
    for i in 0..report.len() - 1 {
        let diff = report[i + 1] - report[i];
        if diff == 0 || diff.abs() > 3 {
            return false;
        }
        if diff > 0 {
            increasing = true;
        } else {
            decreasing = true;
        }
    }
    if increasing && decreasing {
        return false;
    }
    true
}
fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn get_part_1_input(filename: &str) -> Vec<Report> {
    let file_lines = read_lines(filename).unwrap();
    let mut ret = Vec::new();
    for line in file_lines.map_while(Result::ok) {
        let v = line
            .split_whitespace()
            .map(|i| i.parse::<i64>().unwrap())
            .collect();
        ret.push(v);
    }
    ret
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::write;

    #[test]
    fn test_get_part_1_input() {
        let test_filename = "test_input.txt";
        let test_content = "\
38 41 44 47 50 47
75 78 79 82 85 85
11 13 16 19 21 25
39 40 43 44 50";

        // Write the test input file
        write(test_filename, test_content).unwrap();
        let expected = vec![
            vec![38, 41, 44, 47, 50, 47],
            vec![75, 78, 79, 82, 85, 85],
            vec![11, 13, 16, 19, 21, 25],
            vec![39, 40, 43, 44, 50],
        ];
        let result = get_part_1_input(test_filename);
        assert_eq!(result, expected);
        // Clean up the file afterward
        std::fs::remove_file(test_filename).unwrap();
    }
    #[test]
    fn test_safe() {
        let tests: Vec<Vec<i64>> = vec![vec![7, 6, 4, 2, 1], vec![1, 3, 6, 7, 9]];
        for test in tests {
            assert!(is_safe(&test))
        }
    }

    #[test]
    fn test_not_safe() {
        let tests: Vec<Vec<i64>> = vec![
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
        ];
        for test in tests {
            assert!(!is_safe(&test))
        }
    }
}
