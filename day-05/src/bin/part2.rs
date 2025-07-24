use std::cmp::Ordering;
use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");
    dbg!(filename);
    let (edges, mut updates) = get_part_1_input(filename);
    let ans = part_2(&edges, &mut updates);
    dbg!(ans);
}
fn part_2(edges: &[(i64, i64)], updates: &mut [Vec<i64>]) -> i64 {
    let orderings: HashSet<(i64, i64)> = edges.iter().copied().collect();
    let compare_func = |x: &i64, y: &i64| {
        let (x, y) = (*x, *y);
        if orderings.contains(&(x, y)) {
            Ordering::Less
        } else if orderings.contains(&(y, x)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    };
    let mut midlle_sum = 0;
    for update in updates {
        if !update.is_sorted_by(|a, b| compare_func(a, b) != Ordering::Greater) {
            update.sort_by(compare_func);
            midlle_sum += update[update.len() / 2];
        }
    }
    midlle_sum
}

fn get_part_1_input(filename: &str) -> (Vec<(i64, i64)>, Vec<Vec<i64>>) {
    let inp = read_to_string(filename).unwrap();
    parse_input(&inp)
}
fn parse_input(inp: &str) -> (Vec<(i64, i64)>, Vec<Vec<i64>>) {
    let mut edges = Vec::new();
    let mut updates = Vec::new();
    let mut parsing_coords = true;
    for line in inp.lines() {
        if line.is_empty() {
            parsing_coords = false;
            continue;
        }

        if parsing_coords {
            let nums: Vec<i64> = line.split('|').map(|s| s.parse().unwrap()).collect();
            edges.push((nums[0], nums[1]));
        } else {
            let nums: Vec<i64> = line.split(',').map(|s| s.parse().unwrap()).collect();
            updates.push(nums);
        }
    }

    (edges, updates)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_part_1_input() {
        let input = "\
1|2
3|4
5|6

7,8,9
10,11";

        let (edges, updates) = parse_input(input);

        assert_eq!(edges, vec![(1, 2), (3, 4), (5, 6)]);
        assert_eq!(updates, vec![vec![7, 8, 9], vec![10, 11]]);
    }
    #[test]
    fn test_example() {
        let input = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

        let (edges, mut updates) = parse_input(input);

        assert_eq!(part_2(&edges, &mut updates), 123);
    }
}
