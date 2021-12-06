use crate::{solution, Result};

fn parse_input(input: &str) -> Result<[usize; 10]> {
    input
        .split(',')
        .map(|x| x.trim().parse::<usize>())
        .try_fold([0; 10], |mut school, fish| {
            school[fish?] += 1;
            Ok(school)
        })
}

fn count_fish(school: [usize; 10], days: usize) -> usize {
    (0..10)
        .cycle()
        .take(days)
        .fold(school, |mut s, i| {
            s[(i + 9) % 10] += s[i];
            s[(i + 7) % 10] += s[i];
            s[i] = 0;
            s
        })
        .iter()
        .sum()
}

fn part1(input: &str) -> Result<usize> {
    Ok(count_fish(parse_input(input)?, 80))
}

fn part2(input: &str) -> Result<usize> {
    Ok(count_fish(parse_input(input)?, 256))
}

solution!(part1 => 379414, part2 => 1705008653296);
