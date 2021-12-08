use itertools::Itertools;

pub fn solve(input: &str) -> usize {
    let positions: Vec<i32> = input.split(',').map(|s| s.parse().unwrap()).collect();

    if let itertools::MinMaxResult::MinMax(min, max) = positions.iter().minmax() {
        let mut sums = vec![0; *max as usize + 1];

        return (*min..=*max)
            .map(|x| {
                positions
                    .iter()
                    .map(|y| sum(&mut sums, (*y - x).abs() as usize))
                    .sum()
            })
            .min()
            .unwrap();
    }

    unreachable!()
}

fn sum(map: &mut Vec<usize>, n: usize) -> usize {
    if map[n] == 0 {
        map[n] = n * (n + 1) / 2;
    }
    map[n]
}
