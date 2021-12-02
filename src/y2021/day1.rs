use itertools::Itertools;

pub fn solve(input: &str) -> color_eyre::Result<usize> {
    Ok(input
        .lines()
        .filter_map(|s| s.parse::<i32>().ok())
        .tuple_windows()
        .filter(|(a, _, _, d)| a < d)
        .count())
}
