use std::fs::read_to_string;

type Grid = Vec<Vec<char>>;

fn main() {
    let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");
    dbg!(filename);
    let grid = get_part_1_input(filename);
    let ans = part_1(grid);
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
fn part_1(mut grid: Grid) -> i64 {
    // find position of the '^';
    let (mut i, mut j) = find_guard_pos(&grid);
    let rows = grid.len() as i64;
    let cols = grid[0].len() as i64;
    let mut dir = Dir::Up;
    'outer: loop {
        grid[i as usize][j as usize] = 'X';
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
    count_num_of_hashes(&grid)
}

fn count_num_of_hashes(grid: &Grid) -> i64 {
    let mut ans = 0;
    for row in grid {
        for char_ele in row {
            if char_ele == &'X' {
                ans += 1;
            }
        }
    }
    ans
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
    fn test_taking_input() {
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
        let p1_inp = parse_input(test_example);
        let ans = part_1(p1_inp);
        assert_eq!(ans, 41);
    }
}
