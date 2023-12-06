use std::{fs::read_to_string, ops::Range};

use indicatif::ProgressIterator;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{self, line_ending, multispace1, space0},
    multi::{many1, separated_list1},
    sequence::{terminated, tuple},
    IResult, Parser,
};
use nom_supreme::ParserExt;

fn main() {
    let input = read_to_string("input/p1.txt").expect("input should exist");
    let output = part2(&input);
    dbg!(output);
}

#[derive(Debug)]
struct Directory {
    // a lits of mappings for each section,
    // dst range -> src range
    maps: Vec<Vec<(Range<u64>, Range<u64>)>>,
}

impl Directory {
    fn min_dst_from_range(&mut self, seed_ranges: &[Range<u64>]) -> u64 {
        seed_ranges
            .iter()
            .map(|range| {
                let seeds = range.clone().into_iter().collect::<Vec<u64>>();
                seeds
                    .iter()
                    .progress()
                    .map(|seed| {
                        let mut next_dst = *seed;
                        self.maps.iter().for_each(|map| {
                            for (dst, src) in map {
                                if src.contains(&next_dst) {
                                    next_dst = dst.start + (next_dst - src.start);
                                    break;
                                }
                            }
                            // if there is no match within the ranges, we can use the same value for next_dst
                            // (the rules state that if there is no entry, then src -> dst directly)
                        });
                        next_dst
                    })
                    .min()
                    .unwrap_or(0)
            })
            .min()
            .unwrap_or(0)
    }
}

fn num(input: &str) -> IResult<&str, u64> {
    let (input, num) = terminated(complete::u64, space0)(input)?;
    Ok((input, num))
}

fn mapping(input: &str) -> IResult<&str, Vec<(Range<u64>, Range<u64>)>> {
    let (input, mappings) = take_until(":")
        .precedes(tag(":"))
        .precedes(multispace1)
        .precedes(separated_list1(line_ending, tuple((num, num, num))))
        .parse(input)?;
    let ranges = mappings
        .iter()
        .map(|(dst, src, n)| (*dst..(*dst + *n), *src..(*src + *n)))
        .collect::<Vec<(Range<u64>, Range<u64>)>>();
    Ok((input, ranges))
}

fn almanac(input: &str) -> IResult<&str, (Vec<Range<u64>>, Directory)> {
    let (input, seed_pairs) = tag("seeds: ")
        .precedes(many1(tuple((
            terminated(complete::u64, space0),
            terminated(complete::u64, space0),
        ))))
        .parse(input)?;
    let seeds = seed_pairs
        .iter()
        .map(|(src, n)| (*src..*src + *n))
        .collect();
    let (input, maps) = many1(mapping)(input)?;
    Ok((input, (seeds, Directory { maps })))
}

fn part2(input: &str) -> u64 {
    let (_, (seeds, mut dir)) = almanac(input).expect("input should be parsable");
    dir.min_dst_from_range(&seeds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case() {
        let input = read_to_string("input/p1.example.txt").expect("example input should exist");
        let result = part2(&input);
        assert_eq!(result, 46);
    }
}
