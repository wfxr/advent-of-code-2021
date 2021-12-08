use crate::{solution, Result};

fn count_fish(input: &str, days: usize) -> Result<usize> {
    let mut school = input
        .split(',')
        .map(|x| x.trim().parse())
        .try_fold([0; 9], |mut school, x| {
            x.map(|x: usize| {
                school[x] += 1;
                school
            })
        })?;
    (0..days).for_each(|i| school[(i + 7) % 9] += school[i % 9]);
    Ok(school.iter().sum())
}

fn part1(input: &str) -> Result<usize> {
    count_fish(input, 80)
}

fn part2(input: &str) -> Result<usize> {
    count_fish(input, 256)
}

solution!(part1 => 379414, part2 => 1705008653296);
