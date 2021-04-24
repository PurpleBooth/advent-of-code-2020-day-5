use std::io;
use std::io::BufRead;

use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
}

fn main() -> Result<(), Error> {
    let stdin = io::stdin();
    let mut inputs = vec![];

    for line in stdin.lock().lines() {
        inputs.push(line?)
    }

    println!("{:?}", max_seat_id(&inputs));

    Ok(())
}

fn max_seat_id(input: &[String]) -> Option<isize> {
    input
        .iter()
        .map(|boarding_pass| boarding_pass.split_at(7))
        .filter_map(|(rows, columns)| {
            parse_boarding_row(rows)
                .map(|row| (row * 8))
                .and_then(|row| parse_boarding_column(columns).map(|columns| row + columns))
        })
        .max()
}

fn parse_boarding_row(input: &str) -> Option<isize> {
    binary_string_to_isize(input, "B", "F")
}

fn parse_boarding_column(input: &str) -> Option<isize> {
    binary_string_to_isize(input, "R", "L")
}

fn binary_string_to_isize(input: &str, one: &str, zero: &str) -> Option<isize> {
    isize::from_str_radix(&input.replace(one, "1").replace(zero, "0"), 2).ok()
}

#[cfg(test)]
mod tests {
    use crate::{max_seat_id, parse_boarding_column, parse_boarding_row};

    #[test]
    fn no_seats_is_none() {
        assert_eq!(None, max_seat_id(&[]));
    }

    #[test]
    fn simple_example() {
        assert_eq!(Some(567_isize), max_seat_id(&["BFFFBBFRRR".into()]));
    }

    #[test]
    fn multiple_cards() {
        assert_eq!(
            Some(820_isize),
            max_seat_id(&[
                "BFFFBBFRRR".into(),
                "FFFBBBFRRR".into(),
                "BBFFBBFRLL".into()
            ])
        );
    }

    #[test]
    fn parse_row() {
        assert_eq!(Some(70_isize), parse_boarding_row("BFFFBBF"));
        assert_eq!(Some(14_isize), parse_boarding_row("FFFBBBF"));
    }

    #[test]
    fn parse_column() {
        assert_eq!(Some(4_isize), parse_boarding_column("RLL"));
        assert_eq!(Some(7_isize), parse_boarding_column("RRR"));
    }
}
