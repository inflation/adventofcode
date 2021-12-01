pub fn solve(input: &str) -> color_eyre::Result<usize> {
    let mut res = 0;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => res += 1,
            ')' => res -= 1,
            _ => unreachable!(),
        }

        if res == -1 {
            return Ok(i + 1);
        }
    }

    unreachable!()
}
