use itertools::Itertools;

pub fn solve(input: &[u8]) -> u32 {
    let map = input
        .split(|&c| c == b'\n')
        .flat_map(|l| l.iter().map(|&c| c - b'0'))
        .collect_vec();
    let m = input.iter().position(|&c| c == b'\n').unwrap();
    let n = map.len() / m;

    let masks = (0..n)
        .flat_map(|i| {
            (0..m).map(move |j| {
                let mut mask = 0b1111;
                if i == 0 {
                    mask ^= 0b1000;
                };
                if i == n - 1 {
                    mask ^= 0b0100;
                }
                if j == 0 {
                    mask ^= 0b0010;
                }
                if j == m - 1 {
                    mask ^= 0b0001;
                }
                mask
            })
        })
        .collect_vec();

    let check_low = |i: usize, j: usize| -> bool {
        let t = map[i * m + j];
        let mask = masks[i * m + j];

        let mut r = true;
        if mask >> 3 & 0b1 == 1 {
            r &= t < map[(i - 1) * m + j];
        }
        if mask >> 2 & 0b1 == 1 {
            r &= t < map[(i + 1) * m + j];
        }
        if mask >> 1 & 0b1 == 1 {
            r &= t < map[i * m + j - 1];
        }
        if mask & 0b1 == 1 {
            r &= t < map[i * m + j + 1];
        }
        r
    };

    let mut basins = vec![];

    for i in 0..n {
        for j in 0..m {
            if check_low(i, j) {
                basins.push(find_basin(
                    (&map, &masks, m),
                    &mut vec![false; m * n],
                    i,
                    j,
                    0,
                ));
            }
        }
    }

    basins.sort_unstable();
    basins.into_iter().rev().take(3).product()
}

fn find_basin(
    info @ (map, masks, m): (&[u8], &[u8], usize),
    checked: &mut [bool],
    i: usize,
    j: usize,
    mut size: u32,
) -> u32 {
    let mask = masks[i * m + j];

    if map[i * m + j] == 9 {
        return size;
    }

    size += 1;
    checked[i * m + j] = true;

    if mask >> 3 & 0b1 == 1 && !checked[(i - 1) * m + j] {
        size = find_basin(info, checked, i - 1, j, size);
    }
    if mask >> 2 & 0b1 == 1 && !checked[(i + 1) * m + j] {
        size = find_basin(info, checked, i + 1, j, size);
    }
    if mask >> 1 & 0b1 == 1 && !checked[i * m + j - 1] {
        size = find_basin(info, checked, i, j - 1, size);
    }
    if mask & 0b1 == 1 && !checked[i * m + j + 1] {
        size = find_basin(info, checked, i, j + 1, size);
    }

    size
}
