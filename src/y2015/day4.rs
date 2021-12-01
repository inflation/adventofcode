pub fn solve(input: &str) -> color_eyre::Result<u32> {
    for i in 1.. {
        let candidate = format!("{}{}", input, i);
        let hex = format!("{:x}", md5::compute(candidate));
        if hex.starts_with("000000") {
            dbg!(hex);
            return Ok(i);
        }
    }

    unreachable!()
}
