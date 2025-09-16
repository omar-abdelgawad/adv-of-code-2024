use std::fs::read_to_string;

type Equation = (u64, Vec<u64>);
type Equations = Vec<Equation>;
fn main() {
    let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");
    dbg!(filename);
    let part1_inp = get_part_1_input(filename);
    let ans = part_1(part1_inp);
    dbg!(ans);
}
fn part_1(equations: Equations) -> u64 {
    let mut ans = 0;
    for equation in equations {
        if is_equation_valid(&equation) {
            ans += equation.0;
        }
    }
    ans
}

fn is_equation_valid(equation: &Equation) -> bool {
    is_equation_valid_helper(equation.0, &equation.1, 1, equation.1[0])
}
fn is_equation_valid_helper(test_value: u64, nums: &Vec<u64>, ind: usize, accum: u64) -> bool {
    if ind == nums.len() {
        return accum == test_value;
    }
    if accum > test_value {
        return false;
    }
    let new_accum_plus = accum + nums[ind];
    let new_accum_times = accum * nums[ind];
    if new_accum_plus < accum || new_accum_times < accum {
        return false; // overlfow
    }
    if is_equation_valid_helper(test_value, nums, ind + 1, new_accum_plus) {
        return true;
    }
    if is_equation_valid_helper(test_value, nums, ind + 1, new_accum_times) {
        return true;
    }
    false
}

fn get_part_1_input(filename: &str) -> Equations {
    let inp = read_to_string(filename).unwrap();
    parse_input(&inp)
}

fn parse_input(inp: &str) -> Equations {
    let mut ret = Vec::new();
    for line in inp.lines() {
        let (test_value_str, nums_str) = line.split_once(':').unwrap();
        let test_value: u64 = test_value_str.parse().unwrap();
        let nums = nums_str
            .trim()
            .split(' ')
            .map(|num| num.parse().unwrap())
            .collect();
        ret.push((test_value, nums));
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_taking_input() {
        let test_example = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        // too lazy to continue
        let p1_inp = parse_input(test_example);
        let ans = part_1(p1_inp);
        assert_eq!(ans, 3749);
    }
}
