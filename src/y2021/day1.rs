use itertools::Itertools;

pub fn solve(input: &str) -> color_eyre::Result<usize> {
    let values: Result<Vec<i32>, _> = input.lines().map(str::parse).collect();
    let values = values?;
    let mut previous = values.iter().take(3).sum();
    let mut res = 0;

    for (a, b, c) in values.iter().tuple_windows().skip(1) {
        let current = a + b + c;

        if current > previous {
            res += 1;
        }

        previous = current;
    }

    Ok(res)
}
