use std::collections::HashSet;

use crate::{solution, Result};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Dot(usize, usize);

enum Fold {
    X(usize),
    Y(usize),
}

impl Fold {
    fn fold(&self, dots: &mut [Dot]) {
        dots.iter_mut().for_each(|dot| match *self {
            Fold::X(x) => dot.0 = if dot.0 < x { dot.0 } else { x + x - dot.0 },
            Fold::Y(y) => dot.1 = if dot.1 < y { dot.1 } else { y + y - dot.1 },
        })
    }
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
    folds.first().ok_or("empty fold instructions")?.fold(&mut dots);
    Ok(dots.into_iter().collect::<HashSet<_>>().len())
}

fn part2(input: &str) -> Result<String> {
    let (mut dots, folds) = parse_input(input)?;
    folds.iter().for_each(|fold| fold.fold(&mut dots));
    let (w, h) = dots.iter().fold((0, 0), |(w, h), &dot| (w.max(dot.0), h.max(dot.1)));
    let mut buf = vec![vec!['.'; w + 1]; h + 1];
    dots.into_iter().for_each(|Dot(x, y)| buf[y][x] = '#');
    Ok(buf.into_iter().intersperse(vec!['\n']).flatten().collect())
}

solution!(part1 => 790, part2 => "
###...##..#..#.####.###..####...##..##.
#..#.#..#.#..#....#.#..#.#.......#.#..#
#..#.#....####...#..###..###.....#.#...
###..#.##.#..#..#...#..#.#.......#.#...
#....#..#.#..#.#....#..#.#....#..#.#..#
#.....###.#..#.####.###..#.....##...##.
");
