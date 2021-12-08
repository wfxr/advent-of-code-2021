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

// 0: abc efg
// 1:   c  f
// 2: a cde g
// 3: a cd fg
// 4:  bcd f
// 5: ab d fg
// 6: ab defg
// 7: a c  f
// 8: abcdefg
// 9: abcd fg
fn part2(input: &str) -> Result<u32> {
    let input = parse_input(input)?;
    Ok(input
        .into_iter()
        .map(|(patterns, output)| {
            let with_len = |len| -> _ { patterns.iter().filter(move |x| x.count_ones() == len).copied() };
            let mut map = [0; 1 << 7];
            let x1 = with_len(2).next().unwrap();
            let x4 = with_len(4).next().unwrap();
            let x7 = with_len(3).next().unwrap();
            let x8 = with_len(7).next().unwrap();
            let x6 = with_len(6).find(|&x| x & x1 != x1).unwrap();
            let c = x1 & !(x6 & x1);
            let x5 = with_len(5).find(|&x| x & c == 0).unwrap();
            let e = x5 ^ x6;
            let x9 = with_len(6).find(|&x| x & e == 0).unwrap();
            let x0 = with_len(6).find(|&x| x != x9 && x != x6).unwrap();
            let x2 = with_len(5).find(|&x| x & e != 0).unwrap();
            let x3 = with_len(5).find(|&x| x != x5 && x != x2).unwrap();

            map[x0 as usize] = 0;
            map[x1 as usize] = 1;
            map[x2 as usize] = 2;
            map[x3 as usize] = 3;
            map[x4 as usize] = 4;
            map[x5 as usize] = 5;
            map[x6 as usize] = 6;
            map[x7 as usize] = 7;
            map[x8 as usize] = 8;
            map[x9 as usize] = 9;

            output.iter().fold(0, |acc, &x| acc * 10 + map[x as usize])
        })
        .sum())
}

solution!(part1 => 303, part2 => 961734);
