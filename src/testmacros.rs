#[macro_export]
macro_rules! test {
    ($part:ident $(,$name:ident: $input:expr => $expected:expr)* $(,)?) => {
        mod $part {
            #[allow(unused_imports)]
            use super::*;
            use super::super::SOLUTION;
            #[allow(unused_imports)]
            use crate::input;
            $(
                #[test]
                fn $name() -> Result<(), Box<dyn std::error::Error>> {
                    let res = (SOLUTION.$part)($input)?;
                    assert_eq!(res, stringify!($expected));
                    Ok(())
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! solution_test {
    ($part:ident => $answer:expr) => {
        #[test]
        fn $part() -> Result<(), Box<dyn std::error::Error>> {
            let input = include_str!("input");
            let res = (super::SOLUTION.$part)(input)?;
            assert_eq!(
                stringify!($answer).trim_matches(|c| c == '"').trim_matches('\n'),
                res
            );
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! solution_bench {
    ($part:ident) => {
        #[bench]
        fn $part(b: &mut test::Bencher) {
            let input = include_str!("input");
            b.iter(|| {
                (super::SOLUTION.$part)(input).unwrap();
            })
        }
    };
}

#[macro_export]
macro_rules! input {
    ($line:expr) => { $line };
    ($line:expr, $($rest:expr),+ $(,)?) => { concat!($line, '\n', input!($($rest),+)) };
}
