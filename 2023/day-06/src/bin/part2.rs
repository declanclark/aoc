use std::fs::read_to_string;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, multispace1},
    multi::separated_list1,
    sequence::preceded,
    IResult, Parser,
};
use nom_supreme::ParserExt;

fn main() {
    let input = read_to_string("input/p1.txt").expect("input should exist");
    let output = part2(&input);
    dbg!(output);
}

#[derive(Debug)]
struct Record {
    time: u64,
    distance: u64,
}

impl Record {
    fn count_ways_to_beat(&self) -> u64 {
        // for each millisecond that the button on the boat is held for, the boat will travel at
        // that many millimetres per second i.e. hold for 2ms -> boat speed is 2mm/ms
        // therefore, we don't need to check 0 or the max time as these will result in moving 0mm
        // which would only beat a negative distance (not possible)
        let mut count = 0;
        for i in 1..self.time {
            let post_button_time = self.time - i;
            let distance_can_travel = post_button_time * i;
            if distance_can_travel > self.distance {
                count += 1;
            }
        }
        count
    }
}

fn parse_record(input: &str) -> IResult<&str, Record> {
    let (input, times) = tag("Time:")
        .precedes(preceded(multispace1, separated_list1(multispace1, digit1)))
        .terminated(line_ending)
        .parse(input)?;
    let (input, distances) = tag("Distance:")
        .precedes(preceded(multispace1, separated_list1(multispace1, digit1)))
        .terminated(line_ending)
        .parse(input)?;
    let time = times
        .join("")
        .parse::<u64>()
        .expect("race duration should be an integer");
    let distance = distances
        .join("")
        .parse::<u64>()
        .expect("record distance should be an integer");
    Ok((input, Record { time, distance }))
}

fn part2(input: &str) -> u64 {
    let (_, record) = parse_record(input).expect("input should be parsable");
    record.count_ways_to_beat()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case() {
        let input = read_to_string("input/p1.example.txt").expect("example input should exist");
        let result = part2(&input);
        assert_eq!(result, 71503);
    }
}
