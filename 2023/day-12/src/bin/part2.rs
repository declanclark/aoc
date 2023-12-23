use core::panic;
use std::{collections::HashMap, fs::read_to_string};

use itertools::Itertools;
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
    fn to_string(&self) -> String {
        let mut str = String::new();
        str += &self
            .springs
            .iter()
            .map(|state| match state {
                SpringState::Unknown => '?',
                SpringState::Damaged => '#',
                SpringState::Operational => '.',
            })
            .join(",");
        str += ":";
        str += &self
            .damaged_count
            .iter()
            .map(|num| num.to_string())
            .join(",");
        str
    }

    fn count_rec(
        states: &[SpringState],
        damaged: &[u64],
        cache: &mut HashMap<String, usize>,
    ) -> usize {
        if states.is_empty() {
            if damaged.is_empty() {
                return 1;
            }
            return 0;
        }
        if damaged.is_empty() {
            if states.contains(&SpringState::Damaged) {
                return 0;
            }
            return 1;
        }
        let key = SpringRow {
            springs: states.to_vec(),
            damaged_count: damaged.to_vec(),
        }
        .to_string();
        let cache_entry = cache.get(&key);
        if cache_entry.is_some() {
            return *cache_entry.unwrap();
        }
        let mut result = 0;
        if states[0] == SpringState::Unknown || states[0] == SpringState::Operational {
            result += SpringRow::count_rec(&states[1..], damaged, cache);
        }
        if states[0] == SpringState::Unknown || states[0] == SpringState::Damaged {
            if damaged[0] as usize <= states.len()
                && !states
                    .iter()
                    .take(damaged[0] as usize)
                    .contains(&SpringState::Operational)
                && (damaged[0] as usize == states.len()
                    || states[damaged[0] as usize] != SpringState::Damaged)
            {
                let state_start_bound = if damaged[0] as usize == states.len() {
                    damaged[0]
                } else {
                    damaged[0] + 1
                };
                result += SpringRow::count_rec(
                    &states[state_start_bound as usize..],
                    &damaged[1..],
                    cache,
                );
            }
        }
        cache.insert(key, result);
        result
    }

    fn count(&self) -> usize {
        SpringRow::count_rec(
            &self.springs[..],
            &self.damaged_count[..],
            &mut HashMap::new(),
        )
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
            springs: std::iter::repeat(
                springs
                    .chars()
                    .map(|s| match s {
                        '?' => SpringState::Unknown,
                        '#' => SpringState::Damaged,
                        '.' => SpringState::Operational,
                        _ => panic!("unexpected character"),
                    })
                    .collect::<Vec<SpringState>>(),
            )
            .take(5)
            .intersperse(vec![SpringState::Unknown])
            .flatten()
            .collect::<Vec<SpringState>>(),
            damaged_count: std::iter::repeat(damaged_count)
                .take(5)
                .flatten()
                .collect::<Vec<u64>>(),
        })
        .collect::<SpringField>();
    Ok((input, field))
}

fn part1(input: &str) -> usize {
    let (_, field) = parse_input(input).expect("input should be parsable");
    field.iter().map(|row| row.count()).sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case() {
        let input = read_to_string("input/p1.example.txt").expect("example input should exist");
        let result = part1(&input);
        assert_eq!(result, 525152);
    }
}
