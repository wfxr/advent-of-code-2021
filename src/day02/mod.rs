use self::Ins::*;
use crate::{solution, Result};

enum Ins {
    Forward(i32),
    Down(i32),
}

fn parse_input(input: &str) -> impl Iterator<Item = Result<Ins>> + '_ {
    input
        .lines()
        .filter_map(|l| l.split_once(' '))
        .map(|(ins, x)| match (ins, x.parse()?) {
            ("forward", x) => Ok(Forward(x)),
            ("up", x) => Ok(Down(-x)),
            ("down", x) => Ok(Down(x)),
            _ => Err(format!("invalid instruction: {}", ins).into()),
        })
}

fn part1(input: &str) -> Result<i32> {
    parse_input(input)
        .try_fold((0, 0), |(forward, down), ins| {
            ins.map(|ins| match ins {
                Forward(x) => (forward + x, down),
                Down(x) => (forward, down + x),
            })
        })
        .map(|(forward, down)| forward * down)
}

fn part2(input: &str) -> Result<i32> {
    parse_input(input)
        .try_fold((0, 0, 0), |(forward, down, aim), ins| {
            ins.map(|ins| match ins {
                Forward(x) => (forward + x, down + x * aim, aim),
                Down(x) => (forward, down, aim + x),
            })
        })
        .map(|(forward, down, _)| forward * down)
}

solution!(part1 => 1924923, part2 => 1982495697);
