use itertools::Itertools;

const MAP_SIZE: usize = 1000;

#[allow(clippy::cast_sign_loss)]
#[must_use]
pub fn solve(input: &str) -> usize {
    let map = input
        .lines()
        .map(|entry| {
            let ((x1, y1), (x2, y2)) = entry
                .split_ascii_whitespace()
                .filter(|&s| s != "->")
                .filter_map(|coord| coord.split(',').map(|n| n.parse().unwrap()).collect_tuple())
                .collect_tuple()
                .unwrap();

            (x1, y1, x2, y2)
        })
        .fold(
            vec![0_u8; MAP_SIZE * MAP_SIZE],
            |mut map, (x1, y1, x2, y2)| {
                let range = |a: isize, b: isize| {
                    std::iter::successors(Some(a), move |n| Some(n + (b - a).signum()))
                };
                range(x1, x2)
                    .cycle()
                    .zip(range(y1, y2).cycle())
                    .take((x1 - x2).abs().max((y1 - y2).abs()) as usize + 1)
                    .for_each(|(x, y)| map[(x as usize + y as usize * MAP_SIZE) as usize] += 1);

                map
            },
        );

    map.into_iter().filter(|&c| c >= 2).count()
}
