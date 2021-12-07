use std::collections::BTreeMap;

const ROW: u32 = 0b11111;
#[allow(clippy::unusual_byte_groupings)]
const COL: u32 = 0b10000_10000_10000_10000_1;

#[must_use]
pub fn solve(input: &str) -> u32 {
    let (draw, boards) = input.split_once("\n\n").unwrap();
    let mut boards: Vec<(BTreeMap<u8, usize>, u32)> = boards
        .split("\n\n")
        .map(|b| {
            (
                b.split_ascii_whitespace()
                    .enumerate()
                    .map(|(i, n)| (n.parse().unwrap(), i))
                    .collect(),
                0,
            )
        })
        .collect();

    let (board, mark, num) = draw
        .split(',')
        .map(|n| n.parse().unwrap())
        .filter_map(|n| {
            for i in 0..boards.len() {
                if check_win(&mut boards[i], n) {
                    let (b, m) = boards.remove(i);
                    return Some((b, m, n));
                }
            }

            None
        })
        .last()
        .unwrap();

    board
        .into_iter()
        .map(|(n, i)| (mark >> i & 1 ^ 1) * u32::from(n) * u32::from(num))
        .sum()
}

fn check_win((b, m): &mut (BTreeMap<u8, usize>, u32), n: u8) -> bool {
    b.get(&n).map(|i| *m |= 1 << *i).map_or(false, |_| {
        (0..5).any(|i| *m >> i & COL == COL || *m >> (i * 5) & ROW == ROW)
    })
}
