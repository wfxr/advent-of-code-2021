use std::collections::HashSet;

use crate::{solution, Result};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Dot(u32, u32);

impl Dot {
    fn fold_by(&self, f: Fold) -> Self {
        match f {
            Fold::X(x) => Self(if self.0 < x { self.0 } else { x + x - self.0 }, self.1),
            Fold::Y(y) => Self(self.0, if self.1 < y { self.1 } else { y + y - self.1 }),
        }
    }
}

#[derive(Clone, Copy)]
enum Fold {
    X(u32),
    Y(u32),
}

fn parse_input(input: &str) -> Result<(Vec<Dot>, Vec<Fold>)> {
    let (dots, folds) = input.split_once("\n\n").ok_or("missing fold instructions")?;
    let dots = dots
        .lines()
        .filter_map(|l| l.split_once(',').map(|(x, y)| Ok(Dot(x.parse()?, y.parse()?))))
        .collect::<Result<_>>()?;
    let folds = folds
        .lines()
        .filter_map(|l| {
            l.split_once('=').and_then(|(lhs, rhs)| match lhs.chars().last() {
                Some('x') => Some(rhs.parse().map(Fold::X).map_err(|e| e.into())),
                Some('y') => Some(rhs.parse().map(Fold::Y).map_err(|e| e.into())),
                _ => None,
            })
        })
        .collect::<Result<_>>()?;
    Ok((dots, folds))
}

fn part1(input: &str) -> Result<usize> {
    let (mut dots, folds) = parse_input(input)?;
    for dot in dots.iter_mut() {
        *dot = dot.fold_by(*folds.first().ok_or("empty fold instructions")?)
    }
    Ok(dots.iter().collect::<HashSet<_>>().len())
}

fn part2(input: &str) -> Result<String> {
    let (mut dots, folds) = parse_input(input)?;
    dots.iter_mut()
        .for_each(|dot| *dot = folds.iter().fold(*dot, |dot, &fold| dot.fold_by(fold)));
    let dots: HashSet<_> = dots.into_iter().collect();
    let (w, h) = dots.iter().fold((0, 0), |(w, h), &dot| (w.max(dot.0), h.max(dot.1)));
    Ok((0..=h)
        .map(|y| {
            (0..=w)
                .map(|x| match dots.contains(&Dot(x, y)) {
                    true => '#',
                    false => ' ',
                })
                .collect::<String>()
                .trim()
                .to_string()
        })
        .intersperse(String::from("\n"))
        .collect())
}

solution!(part1 => 790, part2 => "
###   ##  #  # #### ###  ####   ##  ##
#  # #  # #  #    # #  # #       # #  #
#  # #    ####   #  ###  ###     # #
###  # ## #  #  #   #  # #       # #
#    #  # #  # #    #  # #    #  # #  #
#     ### #  # #### ###  #     ##   ##
");
