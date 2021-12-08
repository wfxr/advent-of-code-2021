use crate::{solution, Result};

fn parse_input(input: &str) -> Result<Vec<(Vec<u8>, Vec<u8>)>> {
    input
        .lines()
        .map(|line| {
            let mut it = line.split('|').map(|s| {
                s.split_ascii_whitespace()
                    .map(|s| s.bytes().fold(0, |acc, x| acc | (1 << (x - b'a'))))
                    .collect()
            });
            Ok((it.next().ok_or("missing patterns")?, it.next().ok_or("missing output")?))
        })
        .collect()
}

fn part1(input: &str) -> Result<usize> {
    Ok(parse_input(input)?
        .into_iter()
        .flat_map(|(_, rhs)| rhs)
        .filter(|x| matches!(x.count_ones(), 2 | 4 | 3 | 7))
        .count())
}

fn part2(input: &str) -> Result<u32> {
    parse_input(input)?
        .into_iter()
        .map(|(patterns, output)| {
            let mut map = [0; 1 << 7];
            macro_rules! deduct {
                ($x:ident => $v:expr, segments = $len:expr $(,cond = $cond:expr)?) => {
                    let $x = *patterns
                        .iter()
                        .find(move |&&x| x.count_ones() == $len $(&& $cond(x))*)
                        .ok_or_else(|| format!("can not solve '{}'", stringify!($x)))?;
                    map[$x as usize] = $v;
                };
            }
            deduct!(x1 => 1, segments = 2); //   c  f
            deduct!(x4 => 4, segments = 4); //  bcd f
            deduct!(x7 => 7, segments = 3); // a c  f
            deduct!(x8 => 8, segments = 7); // abcdefg
            deduct!(x6 => 6, segments = 6, cond = |x| x & x1 != x1); // ab defg
            let c = x1 & !(x6 & x1); //
            deduct!(x5 => 5, segments = 5, cond = |x| x & c == 0); // ab d fg
            let e = x5 ^ x6; //
            deduct!(x9 => 9, segments = 6, cond = |x| x & e == 0); // abcd fg
            deduct!(x2 => 2, segments = 5, cond = |x| x & e != 0); // a cde g
            deduct!(x3 => 3, segments = 5, cond = |x| x != x5 && x != x2); // a cd fg

            Ok(output.iter().fold(0, |acc, &x| acc * 10 + map[x as usize]))
        })
        .sum()
}

solution!(part1 => 303, part2 => 961734);
