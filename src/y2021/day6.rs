const DAYS: usize = 256;

#[must_use]
pub fn solve(input: &str) -> u64 {
    let mut map = input
        .split(',')
        .map(|s| s.parse().unwrap())
        .fold([0; 9], |mut map, n: usize| {
            map[n] += 1;
            map
        });

    for i in 0..DAYS {
        map[(i + 7) % 9] += map[i % 9];
    }

    map.iter().sum()
}
