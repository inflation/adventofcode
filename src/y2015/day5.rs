use fancy_regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE_PAIR: Regex = Regex::new(r"(..).*\1").unwrap();
    static ref RE_REPEAT: Regex = Regex::new(r"(.).\1").unwrap();
}

pub fn solve(input: &str) -> color_eyre::Result<usize> {
    Ok(input.lines().filter(|&s| nice(s)).count())
}

fn nice(s: &str) -> bool {
    RE_PAIR.is_match(s).unwrap() && RE_REPEAT.is_match(s).unwrap()
}
