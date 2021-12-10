use crate::{solution, Result};

fn part1(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .flat_map(|line| {
            let mut st = Vec::new();
            line.chars().find_map(|c| match c {
                ')' => st.pop().and_then(|c| (c != '(').then_some(3)),
                ']' => st.pop().and_then(|c| (c != '[').then_some(57)),
                '}' => st.pop().and_then(|c| (c != '{').then_some(1197)),
                '>' => st.pop().and_then(|c| (c != '<').then_some(25137)),
                _ => {
                    st.push(c);
                    None
                }
            })
        })
        .sum())
}

fn part2(input: &str) -> Result<usize> {
    let mut scores: Vec<_> = input
        .lines()
        .filter_map(|line| {
            let mut st = Vec::new();
            let corrupted = line.chars().any(|c| match c {
                ')' => !matches!(st.pop(), Some('(')),
                ']' => !matches!(st.pop(), Some('[')),
                '}' => !matches!(st.pop(), Some('{')),
                '>' => !matches!(st.pop(), Some('<')),
                _ => {
                    st.push(c);
                    false
                }
            });
            match (corrupted, st.is_empty()) {
                (false, false) => Some(st.into_iter().rev().fold(0, |acc, c| {
                    acc * 5
                        + match c {
                            '(' => 1,
                            '[' => 2,
                            '{' => 3,
                            '<' => 4,
                            _ => 0,
                        }
                })),
                _ => None,
            }
        })
        .collect();
    let middle = scores.len() / 2;
    Ok(*scores.select_nth_unstable(middle).1)
}

solution!(part1 => 166191, part2 => 1152088313);
