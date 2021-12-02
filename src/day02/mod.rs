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
    let (x, y) = parse_input(input)?
        .into_iter()
        .fold((0, 0), |(horizontal, depth), ins| match ins {
            Forward(n) => (horizontal + n, depth),
            Down(n) => (horizontal, depth + n),
        });
    Ok(x * y)
}

fn part2(input: &str) -> Result<i32> {
    let (horizontal, depth, _) = parse_input(input)?
        .into_iter()
        .fold((0, 0, 0), |(horizontal, depth, aim), ins| match ins {
            Forward(n) => (horizontal + n, depth + n * aim, aim),
            Down(n) => (horizontal, depth, aim + n),
        });
    Ok(horizontal * depth)
}

solution!(part1 => 1924923, part2 => 1982495697);
