use itertools::Itertools;
use miette::miette;
use nom::{
    character::complete::{self, newline, space1},
    multi::separated_list1,
    IResult,
};

type Report = Vec<i32>;

enum Direction {
    Increasing,
    Decreasing,
}

enum Safety {
    Safe,
    Unsafe,
}

fn is_safe(a: i32, b: i32, direction: &Direction) -> bool {
    match b - a {
        (1..=3) => match direction {
            Direction::Increasing => true,
            Direction::Decreasing => false,
        },
        (-3..=-1) => match direction {
            Direction::Increasing => false,
            Direction::Decreasing => true,
        },
        _ => false,
    }
}

fn safety(report: &Report) -> Safety {
    let direction = if report[0] < report[1] {
        Direction::Increasing
    } else {
        Direction::Decreasing
    };

    let err = report
        .iter()
        .tuple_windows()
        .find(|(a, b)| !is_safe(*(*a), *(*b), &direction));

    match err {
        None => Safety::Safe,
        Some(_) => Safety::Unsafe,
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Report>> {
    separated_list1(newline, separated_list1(space1, complete::i32))(input)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, reports) = parse(input).map_err(|e| miette!("parse failed with error: {:?}", e))?;

    let result = reports
        .iter()
        .map(|report| match safety(report) {
            Safety::Safe => Safety::Safe,
            Safety::Unsafe => {
                for idx in 0..report.len() {
                    let mut dampened = report.clone();
                    dampened.remove(idx);
                    match safety(&dampened) {
                        Safety::Safe => return Safety::Safe,
                        Safety::Unsafe => (),
                    };
                }

                Safety::Unsafe
            }
        })
        .map(|safety| match safety {
            Safety::Safe => 1,
            Safety::Unsafe => 0,
        })
        .sum::<i32>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
