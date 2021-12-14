use crate::{solution, Result};

fn parse_input(input: &str) -> Result<Vec<u32>> {
    input.lines().map(|s| Ok(s.parse()?)).collect()
}

fn part1(input: &str) -> Result<usize> {
    let seq = parse_input(input)?;
    Ok(seq.array_windows().filter(|[a, b]| a < b).count())
}

fn part2(input: &str) -> Result<usize> {
    let seq = parse_input(input)?;
    Ok(seq.array_windows().filter(|[a, _, _, d]| a < d).count())
}

solution!(part1 => 1390, part2 => 1457);
