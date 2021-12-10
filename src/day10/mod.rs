use crate::{solution, Result};

fn part1(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(|line| {
            let mut st = Vec::new();
            macro_rules! score {
                ($lhs:pat, $rhs:expr, $score:expr) => {
                    match st.pop() {
                        Some($lhs) => None,
                        _ => Some($score),
                    }
                };
            }
            line.chars()
                .find_map(|c| match c {
                    ')' => score!('(', c, 3),
                    ']' => score!('[', c, 57),
                    '}' => score!('{', c, 1197),
                    '>' => score!('<', c, 25137),
                    _ => {
                        st.push(c);
                        None
                    }
                })
                .unwrap_or(0)
        })
        .sum())
}

fn part2(input: &str) -> Result<usize> {
    let mut scores: Vec<_> = input
        .lines()
        .map(|line| {
            let mut st = Vec::new();
            macro_rules! score {
                ($lhs:pat, $rhs:expr, $score:expr) => {
                    match st.pop() {
                        Some($lhs) => None,
                        _ => Some($score),
                    }
                };
            }
            match line.chars().find_map(|c| match c {
                ')' => score!('(', c, 3),
                ']' => score!('[', c, 57),
                '}' => score!('{', c, 1197),
                '>' => score!('<', c, 25137),
                _ => {
                    st.push(c);
                    None
                }
            }) {
                Some(_) => 0,
                None => match st.is_empty() {
                    true => 0,
                    false => st.into_iter().rev().fold(0, |acc, c| {
                        acc * 5
                            + match c {
                                '(' => 1,
                                '[' => 2,
                                '{' => 3,
                                '<' => 4,
                                _ => 0,
                            }
                    }),
                },
            }
        })
        .filter(|&x| x > 0)
        .collect();
    let middle = scores.len() / 2;
    Ok(*scores.select_nth_unstable(middle).1)
}

solution!(part1 => todo!(), part2 => todo!());
