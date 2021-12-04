use self::Ins::*;
use crate::{solution, Result};

enum Ins {
    Forward(i32),
    Down(i32),
}

fn parse_input(input: &str) -> Result<Vec<Ins>> {
    input
        .lines()
        .map(|line| match line.split_ascii_whitespace().collect::<Vec<_>>()[..] {
            ["forward", x] => Ok(Forward(x.parse()?)),
            ["up", x] => Ok(Down(-x.parse()?)),
            ["down", x] => Ok(Down(x.parse()?)),
            _ => Err(format!("invalid input: {}", line).into()),
        })
        .collect()
}

fn part1(input: &str) -> Result<i32> {
    let (forward, down) = parse_input(input)?
        .into_iter()
        .fold((0, 0), |(forward, down), ins| match ins {
            Forward(x) => (forward + x, down),
            Down(x) => (forward, down + x),
        });
    Ok(forward * down)
}

fn part2(input: &str) -> Result<i32> {
    let (forward, down, _) = parse_input(input)?
        .into_iter()
        .fold((0, 0, 0), |(forward, down, aim), ins| match ins {
            Forward(x) => (forward + x, down + x * aim, aim),
            Down(x) => (forward, down, aim + x),
        });
    Ok(forward * down)
}

solution!(part1 => 1924923, part2 => 1982495697);
