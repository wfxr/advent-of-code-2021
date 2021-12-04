use crate::{solution, Result};

const N: usize = 5;

struct Board(Vec<(u32, bool)>);

impl Board {
    fn mark(&mut self, x: u32) {
        self.0
            .iter_mut()
            .filter(|(e, _)| *e == x)
            .for_each(|(_, marked)| *marked = true);
    }

    fn win(&self) -> bool {
        let row = |i| self.0.iter().skip(i * N).take(N);
        let col = |i| self.0.iter().skip(i).step_by(N);
        (0..N).any(|i| row(i).all(|&(_, marked)| marked) || col(i).all(|&(_, marked)| marked))
    }
    fn score(&self) -> u32 {
        self.0.iter().filter(|&(_, marked)| !marked).map(|&(x, _)| x).sum()
    }
}

fn parse_input(input: &str) -> Result<(Vec<u32>, Vec<Board>)> {
    let mut input = input.split("\n\n");
    let seq = input
        .next()
        .ok_or("missing seq")?
        .split(',')
        .map(|x| Ok(x.parse()?))
        .collect::<Result<_>>()?;
    let boards: Vec<_> = input
        .map(|board| {
            let board: Vec<(u32, bool)> = board
                .split_whitespace()
                .map(|x| Ok((x.parse()?, false)))
                .collect::<Result<_>>()?;
            Ok(Board(board))
        })
        .collect::<Result<_>>()?;
    Ok((seq, boards))
}

fn part1(input: &str) -> Result<u32> {
    let (seq, mut boards) = parse_input(input)?;

    for x in seq {
        for board in boards.iter_mut() {
            board.mark(x);
            if board.win() {
                return Ok(board.score() * x);
            }
        }
    }
    Err("no winner".into())
}

fn part2(_input: &str) -> Result<usize> {
    unimplemented!()
}

solution!(part1 => 38913, part2 => 0);
