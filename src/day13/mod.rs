use std::collections::HashSet;

use crate::{solution, Result};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn fold(&self, f: Fold) -> Self {
        match f {
            Fold::X(x) => {
                let x = if self.x < x { self.x } else { x + x - self.x };
                Self { x, y: self.y }
            }
            Fold::Y(y) => {
                let y = if self.y < y { self.y } else { y + y - self.y };
                Self { x: self.x, y }
            }
        }
    }
}

#[derive(Clone, Copy)]
enum Fold {
    X(usize),
    Y(usize),
}

fn parse_input(input: &str) -> Result<(Vec<Point>, Vec<Fold>)> {
    let (dots, ins) = input.split_once("\n\n").ok_or("missing instructions")?;
    let dots = dots
        .lines()
        .filter_map(|l| {
            l.split_once(',')
                .map(|(x, y)| Ok(Point { x: x.parse()?, y: y.parse()? }))
        })
        .collect::<Result<_>>()?;
    let folds = ins
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
    for d in dots.iter_mut() {
        *d = d.fold(*folds.first().ok_or("empty fold instructions")?)
    }
    Ok(dots.iter().collect::<HashSet<_>>().len())
}

fn part2(_input: &str) -> Result<usize> {
    unimplemented!()
}

solution!(part1 => 790, part2 => todo!());
