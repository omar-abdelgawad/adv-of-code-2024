fn main() {
    let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");
    dbg!(filename);
    let input = get_part_1_input(filename);
    let ans = part_2(&input);
    dbg!(ans);
}
fn part_2(input: &str) -> i64 {
    todo!()
}
fn get_part_1_input(filename: &str) -> String {
    todo!()
}
#[cfg(test)]
mod tests {
    use super::*;
}
