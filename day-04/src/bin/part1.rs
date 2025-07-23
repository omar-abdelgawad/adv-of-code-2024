use std::fs::File;
use std::io::{BufRead, BufReader};

type Pattern = [[i64; 2]; 3];
fn main() {
    let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");
    dbg!(filename);
    let input = get_part_1_input(filename);
    let ans = part_1(&input);
    dbg!(ans);
}
fn part_1(input: &[Vec<u8>]) -> i64 {
    const HORIZONTAL: Pattern = [[0, 1], [0, 2], [0, 3]];
    const HORIZONTAL_BACK: Pattern = [[0, -1], [0, -2], [0, -3]];
    const VERTICAL: Pattern = [[1, 0], [2, 0], [3, 0]];
    const VERTICAL_BACK: Pattern = [[-1, 0], [-2, 0], [-3, 0]];
    const MAIN_DIAG: Pattern = [[1, 1], [2, 2], [3, 3]];
    const MAIN_DIAG_BACK: Pattern = [[-1, -1], [-2, -2], [-3, -3]];
    const ANTI_DIAG: Pattern = [[1, -1], [2, -2], [3, -3]];
    const ANTI_DIAG_BACK: Pattern = [[-1, 1], [-2, 2], [-3, 3]];
    const PATTERNS: [Pattern; 8] = [
        HORIZONTAL,
        HORIZONTAL_BACK,
        VERTICAL,
        VERTICAL_BACK,
        MAIN_DIAG,
        MAIN_DIAG_BACK,
        ANTI_DIAG,
        ANTI_DIAG_BACK,
    ];
    let rows = input.len();
    let cols = input[0].len();
    let mut occurrences = 0;
    for i in 0..rows {
        for j in 0..cols {
            if input[i][j] == b'X' {
                for pattern in PATTERNS {
                    if pattern_exist(input, (i, j), pattern) {
                        occurrences += 1;
                    }
                }
            }
        }
    }
    occurrences
}

fn pattern_exist(input: &[Vec<u8>], (i, j): (usize, usize), pat: Pattern) -> bool {
    fn get(input: &[Vec<u8>], row: i64, col: i64) -> Option<u8> {
        input.get(row as usize)?.get(col as usize).copied()
    }

    let (mi, mj) = (i as i64 + pat[0][0], j as i64 + pat[0][1]);
    let (ai, aj) = (i as i64 + pat[1][0], j as i64 + pat[1][1]);
    let (si, sj) = (i as i64 + pat[2][0], j as i64 + pat[2][1]);

    get(input, mi, mj) == Some(b'M')
        && get(input, ai, aj) == Some(b'A')
        && get(input, si, sj) == Some(b'S')
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
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let grid: Vec<Vec<u8>> = raw.lines().map(|line| line.as_bytes().to_vec()).collect();
        assert_eq!(part_1(&grid), 18);
    }
}
