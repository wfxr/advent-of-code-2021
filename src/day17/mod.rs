use crate::{solution, Result};

fn parse_input(input: &str) -> Result<((i32, i32), (i32, i32))> {
    match input
        .split(|c: char| !matches!(c, '0'..='9' | '-'))
        .filter(|s| !s.is_empty())
        .map(|s| Ok(s.parse()?))
        .collect::<Result<Vec<_>>>()?[..]
    {
        [a, b, c, d] => Ok(((a, b), (c, d))),
        _ => Err("invalid input".into()),
    }
}
fn part1(input: &str) -> Result<i32> {
    let (_, (y_min, _)) = parse_input(input)?;
    let vy_max = (y_min + 1).abs();
    Ok(vy_max * (vy_max + 1) / 2)
}

fn part2(_input: &str) -> Result<usize> {
    todo!()
}

solution!(part1 => 12561, part2 => todo!());

#[cfg(test)]
mod example {
    crate::test!(part1, t1: "target area: x=20..30, y=-10..-5" => 45);
}
