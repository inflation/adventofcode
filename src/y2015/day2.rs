pub fn solve(input: &str) -> color_eyre::Result<u32> {
    let mut res = 0;

    for l in input.lines() {
        let sizes: Vec<u32> = l
            .split('x')
            .map(|s| {
                s.parse()
                    .unwrap_or_else(|_| panic!("Failed to parse number {} at line {}", s, l))
            })
            .collect();

        let (p1, p2, p3) = (
            sizes[0] + sizes[1],
            sizes[1] + sizes[2],
            sizes[2] + sizes[0],
        );
        let perimeter = 2 * (p1.min(p2).min(p3));
        res += perimeter + sizes[0] * sizes[1] * sizes[2];
    }

    Ok(res)
}
