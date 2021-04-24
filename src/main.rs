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

    println!("{:?}", find_gap(&inputs));

    Ok(())
}

fn max_seat_id(input: &[String]) -> Option<usize> {
    calculate_seats(input).into_iter().max()
}

fn calculate_seats(input: &[String]) -> Vec<usize> {
    input
        .iter()
        .map(|boarding_pass| boarding_pass.split_at(7))
        .filter_map(|(rows, columns)| {
            parse_boarding_row(rows)
                .map(|row| (row * 8))
                .and_then(|row| parse_boarding_column(columns).map(|columns| row + columns))
        })
        .collect()
}

fn find_gap(input: &[String]) -> Option<usize> {
    let mut seat_ids = calculate_seats(input);
    seat_ids.sort_unstable();
    let first = seat_ids.first()?;
    seat_ids
        .iter()
        .enumerate()
        .take_while(|(enumerator, seating_number)| first + (enumerator) == **seating_number)
        .map(|(_enumerator, seating_number)| seating_number + 1)
        .last()
}

fn parse_boarding_row(input: &str) -> Option<usize> {
    binary_string_to_usize(input, "B", "F")
}

fn parse_boarding_column(input: &str) -> Option<usize> {
    binary_string_to_usize(input, "R", "L")
}

fn binary_string_to_usize(input: &str, one: &str, zero: &str) -> Option<usize> {
    usize::from_str_radix(&input.replace(one, "1").replace(zero, "0"), 2).ok()
}

#[cfg(test)]
mod tests {
    use crate::{
        calculate_seats, find_gap, max_seat_id, parse_boarding_column, parse_boarding_row,
    };

    #[test]
    fn no_seats_is_none() {
        assert_eq!(None, max_seat_id(&[]));
    }

    #[test]
    fn simple_example() {
        assert_eq!(Some(567_usize), max_seat_id(&["BFFFBBFRRR".into()]));
    }

    #[test]
    fn multiple_cards() {
        assert_eq!(
            Some(820_usize),
            max_seat_id(&[
                "BBFFBBFRLL".into(),
                "FFFBBBFRRR".into(),
                "BFFFBBFRRR".into(),
            ])
        );
    }

    #[test]
    fn calculate_seat() {
        assert_eq!(
            vec![567_usize, 119_usize, 820_usize],
            calculate_seats(&[
                "BFFFBBFRRR".into(),
                "FFFBBBFRRR".into(),
                "BBFFBBFRLL".into()
            ])
        );
    }

    #[test]
    fn find_first_gap() {
        assert_eq!(
            Some(2_usize),
            find_gap(&[
                "FFFFFFFLLR".into(),
                "FFFFFFFRRL".into(),
                "FFFFFFFLLL".into(),
                "FFFFFFFLRR".into(),
            ])
        );
    }

    #[test]
    fn parse_row() {
        assert_eq!(Some(70_usize), parse_boarding_row("BFFFBBF"));
        assert_eq!(Some(14_usize), parse_boarding_row("FFFBBBF"));
    }

    #[test]
    fn parse_column() {
        assert_eq!(Some(4_usize), parse_boarding_column("RLL"));
        assert_eq!(Some(7_usize), parse_boarding_column("RRR"));
    }
}
