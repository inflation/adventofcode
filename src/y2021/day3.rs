use itertools::Itertools;

const BITS: usize = 12;

pub fn solve(input: &str) -> u32 {
    let nums = input
        .lines()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect_vec();

    let o2 = (0..BITS)
        .rev()
        .scan(nums.clone(), |o2, i| {
            let one = 2 * o2.iter().filter(|n| *n & 1 << i > 0).count() >= o2.len();
            o2.retain(|n| (*n & 1 << i > 0) == one);
            o2.first().copied()
        })
        .last()
        .unwrap();

    let co2 = (0..BITS)
        .rev()
        .scan(nums, |co2, i| {
            let one = 2 * co2.iter().filter(|n| *n & 1 << i > 0).count() >= co2.len();
            co2.retain(|n| (*n & 1 << i > 0) != one);
            co2.first().copied()
        })
        .last()
        .unwrap();

    o2 * co2
}
