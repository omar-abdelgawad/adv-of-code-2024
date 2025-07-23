use std::fs::File;
use std::io::{BufRead, BufReader};

type Pattern = [[i64; 2]; 3];
fn main() {
    let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");
    dbg!(filename);
    let input = get_part_1_input(filename);
    let ans = part_2(&input);
    dbg!(ans);
}
fn part_2(input: &[Vec<u8>]) -> i64 {
    //const HORIZONTAL: Pattern = [[0, 1], [0, 2], [0, 3]];
    //const HORIZONTAL_BACK: Pattern = [[0, -1], [0, -2], [0, -3]];
    //const VERTICAL: Pattern = [[1, 0], [2, 0], [3, 0]];
    //const VERTICAL_BACK: Pattern = [[-1, 0], [-2, 0], [-3, 0]];
    //const MAIN_DIAG: Pattern = [[1, 1], [2, 2], [3, 3]];
    //const MAIN_DIAG_BACK: Pattern = [[-1, -1], [-2, -2], [-3, -3]];
    //const ANTI_DIAG: Pattern = [[1, -1], [2, -2], [3, -3]];
    //const ANTI_DIAG_BACK: Pattern = [[-1, 1], [-2, 2], [-3, 3]];
    //const PATTERNS: [Pattern; 8] = [
    //    HORIZONTAL,
    //    HORIZONTAL_BACK,
    //    VERTICAL,
    //    VERTICAL_BACK,
    //    MAIN_DIAG,
    //    MAIN_DIAG_BACK,
    //    ANTI_DIAG,
    //    ANTI_DIAG_BACK,
    //];
    let rows = input.len();
    let cols = input[0].len();
    let mut occurrences = 0;
    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            if input[i][j] == b'A' && pattern_exist(input, (i, j)) {
                occurrences += 1;
            }
        }
    }
    occurrences
}

fn pattern_exist(input: &[Vec<u8>], (i, j): (usize, usize)) -> bool {
    fn get(input: &[Vec<u8>], (row, col): (usize, usize)) -> Option<u8> {
        input.get(row)?.get(col).copied()
    }

    let up_right = (i - 1, j - 1);
    let up_left = (i - 1, j + 1);
    let bot_right = (i + 1, j - 1);
    let bot_left = (i + 1, j + 1);
    let main_diag: bool = (get(input, up_left) == Some(b'M')
        && get(input, bot_right) == Some(b'S'))
        || (get(input, up_left) == Some(b'S') && get(input, bot_right) == Some(b'M'));

    let anti_diag: bool = (get(input, up_right) == Some(b'M')
        && get(input, bot_left) == Some(b'S'))
        || (get(input, up_right) == Some(b'S') && get(input, bot_left) == Some(b'M'));
    (main_diag) && (anti_diag)
}
fn get_part_1_input(filename: &str) -> Vec<Vec<u8>> {
    let file = File::open(filename).unwrap();
    BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().into_bytes())
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        let raw = "\
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";

        let grid: Vec<Vec<u8>> = raw.lines().map(|line| line.as_bytes().to_vec()).collect();
        assert_eq!(part_2(&grid), 9);
    }
}
