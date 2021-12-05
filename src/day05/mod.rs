use std::str::FromStr;

use crate::{solution, Result};

struct Point {
    x: usize,
    y: usize,
}

struct Line {
    a: Point,
    b: Point,
}

struct Diagram {
    cells:     Vec<Vec<u8>>,
    overlaped: usize,
}

impl FromStr for Point {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self> {
        let (x, y) = s.split_once(',').ok_or("invalid point")?;
        Ok(Point { x: x.trim().parse()?, y: y.trim().parse()? })
    }
}

impl FromStr for Line {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self> {
        let (a, b) = s.split_once("->").ok_or("invalid line")?;
        Ok(Line { a: a.parse()?, b: b.parse()? })
    }
}

impl Line {
    fn points(&self) -> impl Iterator<Item = Point> {
        fn range(x1: usize, x2: usize) -> Box<dyn Iterator<Item = usize>> {
            if x1 <= x2 {
                Box::new((x1..=x2).cycle())
            } else {
                Box::new((x2..=x1).rev().cycle())
            }
        }
        let count = self.a.x.abs_diff(self.b.x).max(self.a.y.abs_diff(self.b.y)) + 1;
        range(self.a.x, self.b.x)
            .zip(range(self.a.y, self.b.y))
            .map(|(x, y)| Point { x, y })
            .take(count)
    }
}

impl Diagram {
    fn new(vents: &[Line]) -> Self {
        let (x_max, y_max) = vents
            .iter()
            .flat_map(|Line { a, b }| [a, b])
            .fold((0, 0), |(rows, cols), &Point { x, y }| (rows.max(y), cols.max(x)));
        Diagram { cells: vec![vec![0; y_max + 1]; x_max + 1], overlaped: 0 }
    }

    fn scan(&mut self, line: &Line) -> usize {
        line.points().for_each(|Point { x, y }| {
            self.cells[x][y] = self.cells[x][y].saturating_add(1);
            if self.cells[x][y] == 2 {
                self.overlaped += 1
            }
        });
        self.overlaped
    }
}

fn overlaped_points(input: &str, filter: fn(&&Line) -> bool) -> Result<usize> {
    let lines: Vec<_> = input.lines().map(str::parse).collect::<Result<_>>()?;
    let mut diagram = Diagram::new(&lines);
    Ok(lines
        .iter()
        .filter(filter)
        .map(|line| diagram.scan(line))
        .last()
        .unwrap_or(0))
}

fn part1(input: &str) -> Result<usize> {
    overlaped_points(input, |&Line { a, b }| a.x == b.x || a.y == b.y)
}

fn part2(input: &str) -> Result<usize> {
    overlaped_points(input, |_| true)
}

solution!(part1 => 6548, part2 => 19663);
