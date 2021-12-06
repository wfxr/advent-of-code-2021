use crate::{solution, Result};

fn count_fish(input: &str, days: usize) -> Result<usize> {
    let mut school = [0; 9];
    for x in input.split(',').map(|x| x.trim().parse::<usize>()) {
        school[x?] += 1
    }
    for i in (0..9).cycle().take(days) {
        school[(i + 7) % 9] += school[i] // let the new fish inplace and accumulate the old fish
    }
    Ok(school.iter().sum())
}

fn part1(input: &str) -> Result<usize> {
    count_fish(input, 80)
}

fn part2(input: &str) -> Result<usize> {
    count_fish(input, 256)
}

solution!(part1 => 379414, part2 => 1705008653296);
