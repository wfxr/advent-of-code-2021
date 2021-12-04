use crate::{solution, Result};

fn part1(input: &str) -> Result<u32> {
    let cols = input.lines().next().ok_or("empty input")?.len();
    let rows = input.lines().count();
    let (gamma, epsilon) = (0..cols)
        .map(|i| input.lines().filter(|line| line.as_bytes()[i] == b'1').count())
        .fold((0, 0), |(gamma, epsilon), ones| {
            if ones * 2 > rows {
                ((gamma << 1) | 1, epsilon << 1)
            } else {
                (gamma << 1, (epsilon << 1) | 1)
            }
        });
    Ok(gamma * epsilon)
}

fn part2(input: &str) -> Result<u32> {
    let life_support_rating = |most_common: bool| -> Result<u32> {
        let mut seq: Vec<_> = input.lines().collect();
        let mut col = 0;
        while seq.len() > 1 {
            let ones = seq.iter().filter(|l| l.as_bytes()[col] == b'1').count();
            let target = match (most_common, ones * 2 >= seq.len()) {
                (true, true) | (false, false) => b'1',
                _ => b'0',
            };
            seq = seq.into_iter().filter(|l| l.as_bytes()[col] == target).collect();
            col += 1;
        }
        Ok(u32::from_str_radix(seq.first().ok_or("empty input")?, 2)?)
    };
    Ok(life_support_rating(true)? * life_support_rating(false)?)
}

solution!(part1 => 841526, part2 => 4790390);
