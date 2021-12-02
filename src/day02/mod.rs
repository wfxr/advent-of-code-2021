use crate::{solution, Result};

fn parse_input(input: &str) -> Result<Vec<(i32, i32)>> {
    input
        .lines()
        .map(|line| match line.split_ascii_whitespace().collect::<Vec<_>>()[..] {
            ["forward", x] => Ok((0, x.parse()?)),
            ["up", x] => Ok((-x.parse::<i32>()?, 0)),
            ["down", x] => Ok((x.parse()?, 0)),
            _ => Err(format!("invalid input: {}", line).into()),
        })
        .collect()
}

fn part1(input: &str) -> Result<usize> {
    let (x, y) = parse_input(input)?
        .into_iter()
        .fold((0, 0), |(x, y), (dx, dy)| (x + dx, y + dy));
    Ok(x as usize * y as usize)
}

fn part2(_input: &str) -> Result<usize> {
    unimplemented!()
}

solution!(part1 => 1924923, part2 => 0);
