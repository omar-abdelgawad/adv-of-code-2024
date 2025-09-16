use std::{fs::read_to_string, vec};

type Grid = Vec<Vec<char>>;

fn main() {
    let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");
    dbg!(filename);
    let grid = get_part_1_input(filename);
    let ans = part_2(grid);
    dbg!(ans);
}
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
}
impl Dir {
    pub fn rot(self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }

    pub fn advance_pos(
        (i, j): (i64, i64),
        dir: Self,
        (rows, cols): (i64, i64),
    ) -> Option<(i64, i64)> {
        match dir {
            Dir::Up => {
                if i == 0 {
                    None
                } else {
                    Some((i - 1, j))
                }
            }
            Dir::Down => {
                if i == rows - 1 {
                    None
                } else {
                    Some((i + 1, j))
                }
            }
            Dir::Right => {
                if j == cols - 1 {
                    None
                } else {
                    Some((i, j + 1))
                }
            }
            Dir::Left => {
                if j == 0 {
                    None
                } else {
                    Some((i, j - 1))
                }
            }
        }
    }
}
fn part_2(mut grid: Grid) -> i64 {
    // find position of the '^';
    let (i, j) = find_guard_pos(&grid);
    let (rows, cols) = get_rows_and_cols(&grid);
    let dir = Dir::Up;
    let mut inf_loops = 0;

    for i_obs in 0..rows {
        for j_obs in 0..cols {
            if i_obs == i && j_obs == j {
                continue;
            }
            if grid[i_obs as usize][j_obs as usize] == '.' {
                grid[i_obs as usize][j_obs as usize] = '#';
                if position_can_have_inf_loop(&grid, dir, (i, j)) {
                    inf_loops += 1;
                }
                grid[i_obs as usize][j_obs as usize] = '.';
            }
        }
    }
    inf_loops
}

fn position_can_have_inf_loop(grid: &Grid, mut dir: Dir, (mut i, mut j): (i64, i64)) -> bool {
    let (rows, cols) = get_rows_and_cols(grid);
    let mut visited: Vec<Vec<Option<Dir>>> = vec![vec![None; cols as usize]; rows as usize];

    'outer: loop {
        if visited[i as usize][j as usize] == Some(dir) {
            return true;
        }
        visited[i as usize][j as usize] = Some(dir);
        let Some((mut pos_i, mut pos_j)) = Dir::advance_pos((i, j), dir, (rows, cols)) else {
            break;
        };
        while grid[pos_i as usize][pos_j as usize] == '#' {
            dir = dir.rot();
            if let Some((pos_i_new, pos_j_new)) = Dir::advance_pos((i, j), dir, (rows, cols)) {
                pos_i = pos_i_new;
                pos_j = pos_j_new;
            } else {
                break 'outer;
            };
        }
        if let Some((i_new, j_new)) = Dir::advance_pos((i, j), dir, (rows, cols)) {
            i = i_new;
            j = j_new;
        } else {
            break;
        };
    }
    false
}

fn get_rows_and_cols(grid: &Grid) -> (i64, i64) {
    let rows = grid.len() as i64;
    let cols = grid[0].len() as i64;
    (rows, cols)
}

fn find_guard_pos(grid: &Grid) -> (i64, i64) {
    grid.iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .position(|&c| c == '^')
                .map(|j| (i as i64, j as i64))
        })
        .expect("Guard '^' not found in the grid")
}

fn get_part_1_input(filename: &str) -> Grid {
    let inp = read_to_string(filename).unwrap();
    parse_input(&inp)
}
fn parse_input(inp: &str) -> Grid {
    let mut ret: Grid = Vec::new();
    for line in inp.lines() {
        ret.push(line.chars().collect());
    }
    ret
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_taking_input_part2() {
        let test_example = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        // too lazy to continue
        let p2_inp = parse_input(test_example);
        let ans = part_2(p2_inp);
        assert_eq!(ans, 6);
    }
}
