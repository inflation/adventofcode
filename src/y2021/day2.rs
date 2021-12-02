use itertools::Itertools;

pub fn solve(input: &str) -> color_eyre::Result<i32> {
    let mut depth = 0;
    let mut distance = 0;
    let mut aim = 0;

    for l in input.lines() {
        if let [method, x] = *l.split_whitespace().collect_vec().as_slice() {
            let x: i32 = x.parse()?;
            match method {
                "forward" => {
                    distance += x;
                    depth += aim * x;
                }
                "down" => aim += x,
                "up" => aim -= x,
                _ => unreachable!(),
            }
        }
    }

    Ok(depth * distance)
}
