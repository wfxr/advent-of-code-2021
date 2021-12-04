use crate::{solution, Result};

const N: usize = 5;

struct Board([Option<u32>; N * N]);

impl Board {
    fn mark(&mut self, x: u32) -> bool {
        self.0.iter_mut().filter(|c| c == &&Some(x)).for_each(|c| *c = None);
        let row_marked = |i| self.0.iter().skip(i * N).take(N).all(|c| c.is_none());
        let col_marked = |i| self.0.iter().skip(i).step_by(N).all(|c| c.is_none());
        (0..N).any(|i| row_marked(i) || col_marked(i))
    }

    fn score(&self) -> u32 {
        self.0.iter().flatten().sum()
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
            let board: Vec<_> = board
                .split_whitespace()
                .map(|x| Ok(Some(x.parse()?)))
                .collect::<Result<Vec<_>>>()?;
            Ok(Board(board.try_into().map_err(|_| "invalid board")?))
        })
        .collect::<Result<_>>()?;
    Ok((seq, boards))
}

fn part1(input: &str) -> Result<u32> {
    let (seq, boards) = parse_input(input)?;
    let first = 0;
    play(seq, boards, first)
}

fn part2(input: &str) -> Result<u32> {
    let (seq, boards) = parse_input(input)?;
    let last = boards.len() - 1;
    play(seq, boards, last)
}

fn play(seq: Vec<u32>, mut boards: Vec<Board>, stop_at: usize) -> Result<u32> {
    let mut finished = Vec::new();
    seq.into_iter()
        .find_map(|x| {
            finished.extend(boards.drain_filter(|b| b.mark(x)));
            finished.get(stop_at).map(|b| b.score() * x)
        })
        .ok_or_else(|| "no winner".into())
}

solution!(part1 => 38913, part2 => 16836);
