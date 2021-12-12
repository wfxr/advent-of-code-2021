use crate::{solution, Result};

const N: usize = 10;
const FLASH_LEVEL: i8 = 10;

type Octopuses = [i8; N * N];

fn parse_input(input: &str) -> Result<Octopuses> {
    input
        .bytes()
        .filter(|&c| c.is_ascii_digit())
        .map(|c| (c - b'0') as i8)
        .collect::<Vec<_>>()
        .try_into()
        .map_err(|_| format!("not a {N} x {N} array").into())
}

fn flash(octopuses: &mut Octopuses, pos: usize) -> usize {
    match octopuses[pos] {
        -1 => 0,
        x if x < FLASH_LEVEL => 0,
        _ => {
            octopuses[pos] = -1;
            1 + neighbors(pos)
                .map(|pos| {
                    if octopuses[pos] != -1 {
                        octopuses[pos] += 1;
                    }
                    flash(octopuses, pos)
                })
                .sum::<usize>()
        }
    }
}

fn neighbors(x: usize) -> impl Iterator<Item = usize> {
    [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
        .into_iter()
        .map(move |(di, dj)| (((x / N) as isize + di) as usize, ((x % N) as isize + dj) as usize))
        .filter(|&(ii, jj)| ii < N && jj < N)
        .map(|(ii, jj)| ii * N + jj)
}

fn part1(input: &str) -> Result<usize> {
    let mut octopuses = parse_input(input)?;
    let mut count = 0;

    for _ in 0..100 {
        octopuses.iter_mut().for_each(|x| match *x {
            -1 => *x += 2,
            _ => *x += 1,
        });
        count += (0..N * N).map(|pos| flash(&mut octopuses, pos)).sum::<usize>();
    }
    Ok(count)
}

fn part2(_input: &str) -> Result<usize> {
    todo!()
}

solution!(part1 => 1691, part2 => todo!());
