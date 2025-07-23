use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    //let file = include_str!("./input.txt");
    let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");
    let (mut left_list, mut right_list) = get_two_lists(filename);
    let ans = part_1(&mut left_list, &mut right_list);
    dbg!(ans);
}
fn part_1(left_list: &mut [i64], right_list: &mut [i64]) -> i64 {
    left_list.sort();
    right_list.sort();
    left_list
        .iter()
        .zip(right_list.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i64>()
}
fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn get_two_lists(filename: &str) -> (Vec<i64>, Vec<i64>) {
    let file_lines = read_lines(filename).unwrap();
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    for line in file_lines.map_while(Result::ok) {
        let (num_1, num_2) = line.split_once(&" ".repeat(3)).unwrap();
        let num_1 = num_1.parse::<i64>().unwrap();
        let num_2 = num_2.parse::<i64>().unwrap();
        //println!("{} {}", num_1, num_2);
        left_list.push(num_1);
        right_list.push(num_2);
    }
    (left_list, right_list)
}
