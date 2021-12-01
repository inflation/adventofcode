use std::collections::HashMap;

type Name<'a> = &'a str;

struct Var<'a> {
    name: Name<'a>,
    val: u16,
}

enum Instr<'a> {
    And(Name<'a>, Name<'a>),
    Or(Name<'a>, Name<'a>),
    Lshift(Name<'a>, Name<'a>),
    Rshift(Name<'a>, Name<'a>),
    Not(Name<'a>),
    Literal(u16),
}

pub fn solve(input: &str) -> color_eyre::Result<u16> {
    let mut variables = HashMap::new();

    for l in input.lines() {
        let v: Vec<_> = l.split(" -> ").collect();
        let instr: Vec<_> = v[0].split_whitespace().collect();
        let target = v[1];

        let res = match instr.as_slice() {
            [x, "AND", y] => Instr::And(x, y),
            [x, "OR", y] => Instr::Or(x, y),
            [x, "LSHIFT", y] => Instr::Lshift(x, y),
            [x, "RSHIFT", y] => Instr::Rshift(x, y),
            ["NOT", x] => Instr::Not(x),
            [literal] => Instr::Literal(literal.parse()?),
            _ => unreachable!(),
        };

        variables.insert(target, res);
    }

    todo!()
}
