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
    cells:     Vec<Vec<u32>>,
    overlaped: usize,
}

impl FromStr for Point {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self> {
        let mut it = s.split(',').map(|s| s.parse());
        match (it.next(), it.next()) {
            (Some(x), Some(y)) => Ok(Point { x: x?, y: y? }),
            _ => Err("invalid point".into()),
        }
    }
}

impl FromStr for Line {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self> {
        let mut it = s.split("->").map(|s| s.trim().parse());
        match (it.next(), it.next()) {
            (Some(a), Some(b)) => Ok(Line { a: a?, b: b? }),
            _ => Err("invalid line".into()),
        }
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

    fn scan(&mut self, Line { a, b }: &Line) -> usize {
        if a.x == b.x {
            let (from, to) = if a.y <= b.y { (a.y, b.y) } else { (b.y, a.y) };
            for y in from..=to {
                self.cells[a.x][y] += 1;
                if self.cells[a.x][y] == 2 {
                    self.overlaped += 1;
                }
            }
        } else if a.y == b.y {
            let (from, to) = if a.x <= b.x { (a.x, b.x) } else { (b.x, a.x) };
            for x in from..=to {
                self.cells[x][a.y] += 1;
                if self.cells[x][a.y] == 2 {
                    self.overlaped += 1;
                }
            }
        } else {
            unimplemented!("only horizontal and vertical lines are allowed");
        }
        self.overlaped
    }
}

fn parse_input(input: &str) -> Result<Vec<Line>> {
    input.lines().map(|line| line.parse()).collect()
}

fn part1(input: &str) -> Result<usize> {
    let lines = parse_input(input)?;
    let mut diagram = Diagram::new(&lines);

    Ok(lines
        .iter()
        .filter(|&Line { a, b }| a.x == b.x || a.y == b.y)
        .map(|line| diagram.scan(line))
        .last()
        .unwrap_or(0))
}

fn part2(_input: &str) -> Result<usize> {
    unimplemented!()
}

solution!(part1 => 6548, part2 => 0);
