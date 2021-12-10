use itertools::Itertools;

pub fn solve(input: &[u8]) -> u64 {
    let mut paren = vec![];

    let mut result = input
        .split(|&c| c == b'\n')
        .filter_map(|l| {
            paren.clear();

            for &c in l {
                match c {
                    b'(' | b'[' | b'{' | b'<' => paren.push(c),
                    _ => {
                        let b = paren.pop().unwrap();
                        if c == b')' && b != b'('
                            || c == b']' && b != b'['
                            || c == b'}' && b != b'{'
                            || c == b'>' && b != b'<'
                        {
                            return None;
                        }
                    }
                }
            }

            Some(paren.iter().rev().fold(0, |res, b| {
                res * 5
                    + match b {
                        b'(' => 1,
                        b'[' => 2,
                        b'{' => 3,
                        b'<' => 4,
                        _ => unreachable!(),
                    }
            }))
        })
        .collect_vec();

    result.sort_unstable();
    result[result.len() / 2]
}
