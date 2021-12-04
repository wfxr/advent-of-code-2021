use crate::{solution, Result};

const N: usize = 5;

struct Board {
    cells: [Option<u32>; N * N],
}

impl Board {
    fn mark(&mut self, x: u32) -> bool {
        self.cells.iter_mut().filter(|c| *c == &Some(x)).for_each(|c| *c = None);
        let row_marked = |i| self.cells.iter().skip(i * N).take(N).all(|c| c.is_none());
        let col_marked = |i| self.cells.iter().skip(i).step_by(N).all(|c| c.is_none());
        (0..N).any(|i| row_marked(i) || col_marked(i))
    }

    fn score(&self) -> u32 {
        self.cells.iter().flatten().sum()
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
    let boards = input
        .map(|s| s.split_whitespace().map(|x| Ok(Some(x.parse()?))).collect())
        .map(|cells: Result<Vec<_>>| Ok(Board { cells: cells?.try_into().map_err(|_| "invalid board")? }))
        .collect::<Result<_>>()?;
    Ok((seq, boards))
}

fn part1(input: &str) -> Result<u32> {
    let (seq, boards) = parse_input(input)?;
    let stop_after = 0; // stop after first finished
    play(seq, boards, stop_after)
}

fn part2(input: &str) -> Result<u32> {
    let (seq, boards) = parse_input(input)?;
    let stop_after = boards.len() - 1; // stop after last finished
    play(seq, boards, stop_after)
}

fn play(seq: Vec<u32>, mut boards: Vec<Board>, stop_at: usize) -> Result<u32> {
    let mut finished = Vec::new();
    seq.into_iter()
        .find_map(|x| {
            finished.extend(boards.drain_filter(|board| board.mark(x)));
            finished.get(stop_at).map(|board| board.score() * x)
        })
        .ok_or_else(|| "no winner".into())
}

solution!(part1 => 38913, part2 => 16836);
