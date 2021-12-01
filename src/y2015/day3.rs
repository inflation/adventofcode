use std::collections::HashMap;

pub fn solve(input: &str) -> color_eyre::Result<usize> {
    let mut houses = HashMap::new();
    let mut current1 = (0, 0);
    let mut current2 = (0, 0);

    for (i, d) in input.chars().enumerate() {
        let current = if i % 2 == 1 {
            &mut current1
        } else {
            &mut current2
        };

        match d {
            '>' => current.1 += 1,
            '<' => current.1 -= 1,
            '^' => current.0 -= 1,
            'v' => current.0 += 1,
            _ => unreachable!(),
        }

        houses.entry(*current).or_insert(1);
    }

    Ok(houses.keys().count())
}
