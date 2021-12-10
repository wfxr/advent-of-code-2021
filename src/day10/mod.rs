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

fn part2(_input: &str) -> Result<usize> {
    todo!()
}

solution!(part1 => todo!(), part2 => todo!());
