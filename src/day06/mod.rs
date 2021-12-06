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
        .take(10 * days)
        .fold(school, |mut school, nth| {
            let count = school[nth];
            match nth {
                0 => {
                    school[9] += count;
                    school[7] += count;
                    school[0] -= count;
                }
                x => {
                    school[x - 1] += count;
                    school[x] -= count;
                }
            }
            school
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
