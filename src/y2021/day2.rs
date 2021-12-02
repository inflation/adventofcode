use itertools::Itertools;

pub fn solve(input: &str) -> color_eyre::Result<i32> {
    let mut depth = 0;
    let mut distance = 0;
    let mut aim = 0;

    input
        .lines()
        .filter_map(|l| l.split(' ').collect_tuple())
        .for_each(|(method, x)| {
            let x: i32 = x.parse().unwrap();
            match method {
                "forward" => {
                    distance += x;
                    depth += aim * x;
                }
                "down" => aim += x,
                "up" => aim -= x,
                _ => unreachable!(),
            }
        });

    Ok(depth * distance)
}
