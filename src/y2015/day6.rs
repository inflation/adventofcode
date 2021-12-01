pub fn solve(input: &str) -> color_eyre::Result<i32> {
    let mut lights = vec![0_i32; 1000 * 1000];
    for l in input.lines() {
        let splitted: Vec<&str> = l.split_whitespace().flat_map(|s| s.split(',')).collect();
        match splitted.as_slice() {
            ["turn", "on", x1, y1, "through", x2, y2] => {
                for i in x1.parse::<usize>()?..=x2.parse()? {
                    for j in y1.parse::<usize>()?..=y2.parse()? {
                        lights[i * 1000 + j] += 1;
                    }
                }
            }
            ["turn", "off", x1, y1, "through", x2, y2] => {
                for i in x1.parse::<usize>()?..=x2.parse()? {
                    for j in y1.parse::<usize>()?..=y2.parse()? {
                        lights[i * 1000 + j] = 0.max(lights[i * 1000 + j] - 1);
                    }
                }
            }
            ["toggle", x1, y1, "through", x2, y2] => {
                for i in x1.parse::<usize>()?..=x2.parse()? {
                    for j in y1.parse::<usize>()?..=y2.parse()? {
                        lights[i * 1000 + j] += 2;
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    Ok(lights.into_iter().sum())
}
