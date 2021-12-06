use crate::{solution, Result};

fn parse_input(input: &str) -> Result<Vec<u8>> {
    input.split(',').map(|x| Ok(x.trim().parse()?)).collect()
}

fn part1(input: &str) -> Result<usize> {
    Ok((0..80)
        .fold(parse_input(input)?, |school, _| {
            school
                .into_iter()
                .flat_map(|fish| match fish {
                    0 => vec![6, 8],
                    x => vec![x - 1],
                })
                .collect()
        })
        .len())
}

fn part2(_input: &str) -> Result<usize> {
    unimplemented!()
}

solution!(part1 => 379414, part2 => 0);
