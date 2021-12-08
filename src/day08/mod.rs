use crate::{solution, Result};

fn parse_input(input: &str) -> Result<Vec<(Vec<&str>, Vec<&str>)>> {
    input
        .lines()
        .map(|line| {
            let (lhs, rhs) = line.split_once('|').ok_or("invalid input")?;
            Ok((
                lhs.split_ascii_whitespace().collect(),
                rhs.split_ascii_whitespace().collect(),
            ))
        })
        .collect::<Result<_>>()
}

fn part1(input: &str) -> Result<usize> {
    Ok(parse_input(input)?
        .into_iter()
        .flat_map(|(_, rhs)| rhs)
        .filter(|x| matches!(x.len(), 2 | 4 | 3 | 7))
        .count())
}

fn part2(_input: &str) -> Result<usize> {
    unimplemented!()
}

solution!(part1 => 0, part2 => 0);
