use crate::{solution, Result};

fn parse_input(input: &str) -> Result<[i32; 4]> {
    input
        .split(|c: char| !matches!(c, '0'..='9' | '-'))
        .filter(|s| !s.is_empty())
        .map(|s| Ok(s.parse()?))
        .collect::<Result<Vec<_>>>()?
        .try_into()
        .map_err(|_| "invalid input".into())
}

fn part1(input: &str) -> Result<i32> {
    let [_, _, y_min, _] = parse_input(input)?;
    let vy_max = -y_min - 1;
    Ok(vy_max * (vy_max + 1) / 2)
}

fn part2(input: &str) -> Result<usize> {
    let [x_min, x_max, y_min, y_max] = parse_input(input)?;
    let hit_target = |&(mut vx, mut vy): &(i32, i32)| {
        let (mut x, mut y) = (0, 0);
        while x <= x_max && y >= y_min {
            x += vx;
            y += vy;
            if (x_min..=x_max).contains(&x) && (y_min..=y_max).contains(&y) {
                return true;
            }
            vx = (vx - 1).max(0);
            vy -= 1;
        }
        false
    };
    Ok((1..=x_max)
        .flat_map(|vx| (y_min..=-y_min - 1).map(move |vy| (vx, vy)))
        .filter(hit_target)
        .count())
}

solution!(part1 => 12561, part2 => 3785);

#[cfg(test)]
mod example {
    crate::test!(part1, t1: "target area: x=20..30, y=-10..-5" => 45);
    crate::test!(part2, t1: "target area: x=20..30, y=-10..-5" => 112);
}
