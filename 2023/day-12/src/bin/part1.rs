use core::panic;
use std::{fs::read_to_string, usize};

use itertools::{repeat_n, Itertools};
use nom::{
    bytes::complete::{is_a, tag},
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let input = read_to_string("input/p1.txt").expect("input should exist");
    let output = part1(&input);
    dbg!(output);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug)]
struct SpringRow {
    springs: Vec<SpringState>,
    damaged_count: Vec<u64>,
}

impl SpringRow {
    fn generate_possible_completions(&self) -> Vec<Vec<SpringState>> {
        // 1. get number of unknowns
        let unknowns = self
            .springs
            .iter()
            .filter(|state| **state == SpringState::Unknown)
            .count();
        // 2. find possible permutations for unknowns
        let permutations = repeat_n(
            vec![SpringState::Operational, SpringState::Damaged],
            unknowns,
        )
        .multi_cartesian_product();
        // 3. Fill unknowns with possible states
        permutations
            .map(|p| {
                let mut new_row = self.springs.clone();
                let mut perm_idx = 0;
                for state in new_row.iter_mut() {
                    if *state == SpringState::Unknown {
                        *state = p[perm_idx];
                        perm_idx += 1;
                    }
                }
                new_row
            })
            .collect::<Vec<Vec<SpringState>>>()
    }

    fn check_row(row: &Vec<SpringState>, damaged_count: &Vec<u64>) -> bool {
        let mut damage_in_row = Vec::new();
        let mut contiguous_count = 0;
        for state in row {
            if *state == SpringState::Damaged {
                contiguous_count += 1;
            } else {
                if contiguous_count > 0 {
                    damage_in_row.push(contiguous_count);
                }
                contiguous_count = 0;
            }
        }
        if contiguous_count != 0 {
            damage_in_row.push(contiguous_count);
        }
        let not_matched = damage_in_row
            .iter()
            .zip(damaged_count)
            .filter(|(a, b)| a != b)
            .count();
        return damage_in_row.len() == damaged_count.len() && not_matched == 0;
    }

    fn count_completion_options(&self) -> usize {
        let options = self.generate_possible_completions();
        options
            .iter()
            .filter(|opt| SpringRow::check_row(*opt, &self.damaged_count))
            .count()
    }
}

type SpringField = Vec<SpringRow>;

fn parse_input(input: &str) -> IResult<&str, SpringField> {
    let (input, str_field) = separated_list1(
        line_ending,
        separated_pair(
            is_a(".#?"),
            space1,
            separated_list1(tag(","), complete::u64),
        ),
    )(input)?;
    let field = str_field
        .into_iter()
        .map(|(springs, damaged_count)| SpringRow {
            springs: springs
                .chars()
                .map(|s| match s {
                    '?' => SpringState::Unknown,
                    '#' => SpringState::Damaged,
                    '.' => SpringState::Operational,
                    _ => panic!("unexpected character"),
                })
                .collect::<Vec<SpringState>>(),
            damaged_count,
        })
        .collect::<SpringField>();
    Ok((input, field))
}

fn part1(input: &str) -> usize {
    let (_, field) = parse_input(input).expect("input should be parsable");
    field
        .iter()
        .map(|row| row.count_completion_options())
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case() {
        let input = read_to_string("input/p1.example.txt").expect("example input should exist");
        let result = part1(&input);
        assert_eq!(result, 21);
    }
}
