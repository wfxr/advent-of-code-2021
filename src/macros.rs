#[macro_export]
macro_rules! aoc {
    ($($day:ident),+ $(,)?) => {
        $(pub mod $day;)*
        pub fn build_solutions() -> std::collections::BTreeMap<String, Solution> {
            let mut solutions = std::collections::BTreeMap::new();
            $(solutions.insert(stringify!($day).to_owned(), $day::SOLUTION);)*
            solutions
        }
    };
}

#[macro_export]
macro_rules! solution {
    ($part1:ident => $expected1:expr, $part2:ident => $expected2:expr) => {
        pub(super) const SOLUTION: $crate::Solution = $crate::Solution {
            part1: |input| $part1(input).map(|x| x.to_string()),
            part2: |input| $part2(input).map(|x| x.to_string()),
            input: include_str!("input"),
        };
        #[cfg(test)]
        mod test {
            $crate::solution_test!(part1 => $expected1);
            $crate::solution_test!(part2 => $expected2);
        }
        #[cfg(test)]
        mod bench {
            $crate::solution_bench!(part1);
            $crate::solution_bench!(part2);
        }
    };
}

#[macro_export]
macro_rules! err {
    ($($tt:tt)*) => { Err(format!($($tt)*).into()) };
}
