use itertools::Itertools;

pub fn solve(input: &[u8]) -> usize {
    let mut lights = input
        .iter()
        .filter_map(|&c| if c == b'\n' { None } else { Some(c - b'0') })
        .collect_vec();
    let masks: Vec<u8> = (0..10)
        .flat_map(|i| {
            (0..10).map(move |j| match (i, j) {
                (0, 0) => 0b01010001,
                (0, 9) => 0b01100100,
                (9, 0) => 0b10010010,
                (9, 9) => 0b10101000,
                (0, _) => 0b01110101,
                (9, _) => 0b10111010,
                (_, 0) => 0b11010011,
                (_, 9) => 0b11101100,
                (_, _) => 0xff,
            })
        })
        .collect();

    for k in 0.. {
        lights.iter_mut().for_each(|l| *l += 1);

        while lights.iter().any(|&l| l > 9) {
            for i in 0..10 {
                for j in 0..10 {
                    if lights[i * 10 + j] > 9 {
                        let mask = masks[i * 10 + j];

                        if mask >> 7 & 0b1 == 1 && lights[(i - 1) * 10 + j] != 0 {
                            lights[(i - 1) * 10 + j] += 1;
                        }
                        if mask >> 6 & 0b1 == 1 && lights[(i + 1) * 10 + j] != 0 {
                            lights[(i + 1) * 10 + j] += 1;
                        }
                        if mask >> 5 & 0b1 == 1 && lights[i * 10 + j - 1] != 0 {
                            lights[i * 10 + j - 1] += 1;
                        }
                        if mask >> 4 & 0b1 == 1 && lights[i * 10 + j + 1] != 0 {
                            lights[i * 10 + j + 1] += 1;
                        }
                        if mask >> 3 & 0b1 == 1 && lights[(i - 1) * 10 + j - 1] != 0 {
                            lights[(i - 1) * 10 + j - 1] += 1;
                        }
                        if mask >> 2 & 0b1 == 1 && lights[(i + 1) * 10 + j - 1] != 0 {
                            lights[(i + 1) * 10 + j - 1] += 1;
                        }
                        if mask >> 1 & 0b1 == 1 && lights[(i - 1) * 10 + j + 1] != 0 {
                            lights[(i - 1) * 10 + j + 1] += 1;
                        }
                        if mask & 0b1 == 1 && lights[(i + 1) * 10 + j + 1] != 0 {
                            lights[(i + 1) * 10 + j + 1] += 1;
                        }

                        lights[i * 10 + j] = 0;
                    }
                }
            }
        }

        if lights.iter().all(|&l| l == 0) {
            return k;
        }
    }

    unreachable!()
}
