use std::{collections::HashSet, fs::read_to_string, u32};

use nom::{
    bytes::complete::{tag, take_till},
    character::complete::{self, digit1, line_ending, multispace0, multispace1},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

fn main() {
    let input = read_to_string("input/p1.txt").expect("input should exist");
    let output = part1(&input);
    dbg!(output);
}

#[derive(Debug)]
struct Scratchcard {
    winning_nums: HashSet<u32>,
    my_nums: HashSet<u32>,
}

impl Scratchcard {
    fn get_points(&self) -> u32 {
        let matches = self.my_nums.intersection(&self.winning_nums);
        match matches.count() {
            0 => 0,
            x => u32::pow(2, (x - 1).try_into().expect("should fit into u32")),
        }
    }
}

fn num_set(input: &str) -> IResult<&str, HashSet<u32>> {
    let (input, set) = preceded(multispace0, separated_list1(multispace1, complete::u32))(input)?;
    Ok((input, set.into_iter().collect()))
}

fn card(input: &str) -> IResult<&str, Scratchcard> {
    let (input, _) = preceded(tag("Card"), multispace1)(input)?;
    let (input, _) = preceded(digit1, tag(":"))(input)?;
    let (input, winning_section) = take_till(|c| c == '|')(input)?;
    let (_, winning_nums) =
        num_set(winning_section).expect("winning numbers should be a set of integers");
    let (input, _) = preceded(tag("|"), multispace1)(input)?;
    let (input, my_nums) = num_set(input).expect("my numbers should be a set of integers");
    Ok((
        input,
        Scratchcard {
            winning_nums,
            my_nums,
        },
    ))
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Scratchcard>> {
    let (input, cards) = separated_list1(line_ending, card)(input)?;
    Ok((input, cards))
}

fn part1(input: &str) -> u32 {
    let (_, cards) = parse_cards(input).expect("input should be parsable");
    cards.iter().map(|card| card.get_points()).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case() {
        let input = read_to_string("input/p1.example.txt").expect("example input should exist");
        let result = part1(&input);
        assert_eq!(result, 13);
    }
}
