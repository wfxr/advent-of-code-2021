use crate::{solution, Result};

const N: usize = 5;

struct Board([i32; N * N]);

impl Board {
    fn mark(&mut self, x: u32) -> bool {
        (0..N * N)
            .find_map(|i| (self.0[i] == x as i32).then_some(i))
            .map(|i| {
                self.0[i] = -1;
                self.row_marked(i / N) || self.col_marked(i % N)
            })
            .unwrap_or(false)
    }

    fn score(&self, num: u32) -> u32 {
        self.0.iter().filter(|&&x| x > 0).sum::<i32>() as u32 * num
    }
    fn row_marked(&self, row: usize) -> bool {
        self.0.iter().skip(row * N).take(N).all(|&c| c < 0)
    }
    fn col_marked(&self, col: usize) -> bool {
        self.0.iter().skip(col).step_by(N).all(|&c| c < 0)
    }
}

#[rustfmt::skip]
fn parse_input(input: &str) -> Result<(Vec<u32>, Vec<Board>)> {
    let mut input = input.split("\n\n");
    let seq = input.next().ok_or("missing seq")?
        .split(',')
        .map(|x| Ok(x.parse()?))
        .collect::<Result<_>>()?;
    let boards = input
        .map(|s| s.split_whitespace().map(|x| Ok(x.parse()?)).collect())
        .map(|cells: Result<Vec<_>>| Ok(Board(cells?.try_into().map_err(|_| "invalid board")?)))
        .collect::<Result<_>>()?;
    Ok((seq, boards))
}

fn part1(input: &str) -> Result<u32> {
    let (seq, boards) = parse_input(input)?;
    play(seq, boards, 1) // score of 1st winner
}

fn part2(input: &str) -> Result<u32> {
    let (seq, boards) = parse_input(input)?;
    let last = boards.len();
    play(seq, boards, last) // score of last winner
}

fn play(seq: Vec<u32>, mut remain: Vec<Board>, nth_won: usize) -> Result<u32> {
    let all = remain.len();
    seq.into_iter()
        .find_map(|x| match remain.drain_filter(|board| board.mark(x)).last() {
            Some(board) => (all - remain.len() == nth_won).then(|| board.score(x)),
            None => None,
        })
        .ok_or_else(|| "no winner".into())
}

solution!(part1 => 38913, part2 => 16836);
