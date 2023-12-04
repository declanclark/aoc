use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    u32,
};

use nom::{
    bytes::complete::{tag, take_till},
    character::complete::{self, digit1, line_ending, multispace0, multispace1},
    multi::separated_list1,
    sequence::{preceded, terminated},
    IResult,
};

fn main() {
    let input = read_to_string("input/p1.txt").expect("input should exist");
    let output = part2(&input);
    dbg!(output);
}

#[derive(Debug)]
struct Scratchcard {
    id: u32,
    winning_nums: HashSet<u32>,
    my_nums: HashSet<u32>,
}

impl Scratchcard {
    fn match_count(&self) -> u32 {
        self.my_nums
            .intersection(&self.winning_nums)
            .count()
            .try_into()
            .expect("should fit in u32")
    }
}

fn num_set(input: &str) -> IResult<&str, HashSet<u32>> {
    let (input, set) = preceded(multispace0, separated_list1(multispace1, complete::u32))(input)?;
    Ok((input, set.into_iter().collect()))
}

fn card(input: &str) -> IResult<&str, Scratchcard> {
    let (input, _) = preceded(tag("Card"), multispace1)(input)?;
    let (input, card_id) = terminated(digit1, tag(":"))(input)?;
    let id = card_id
        .parse::<u32>()
        .expect("card id should be an integer");
    let (input, winning_section) = take_till(|c| c == '|')(input)?;
    let (_, winning_nums) =
        num_set(winning_section).expect("winning numbers should be a set of integers");
    let (input, _) = preceded(tag("|"), multispace1)(input)?;
    let (input, my_nums) = num_set(input).expect("my numbers should be a set of integers");
    Ok((
        input,
        Scratchcard {
            id,
            winning_nums,
            my_nums,
        },
    ))
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Scratchcard>> {
    let (input, cards) = separated_list1(line_ending, card)(input)?;
    Ok((input, cards))
}

fn part2(input: &str) -> u32 {
    let (_, cards) = parse_cards(input).expect("input should be parsable");
    let mut card_counts: HashMap<u32, u32> = HashMap::new();
    for card in cards.iter() {
        let matches = card.match_count();
        let copies = *card_counts.entry(card.id).or_insert(1);
        for i in 0..matches {
            let card_id = card.id + i + 1;
            *card_counts.entry(card_id).or_insert(1) += copies;
        }
    }
    card_counts.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case() {
        let input = read_to_string("input/p1.example.txt").expect("example input should exist");
        let result = part2(&input);
        assert_eq!(result, 30);
    }
}
