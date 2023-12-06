use std::fs::read_to_string;

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, multispace1},
    multi::separated_list1,
    sequence::preceded,
    IResult, Parser,
};
use nom_supreme::ParserExt;

fn main() {
    let input = read_to_string("input/p1.txt").expect("input should exist");
    let output = part1(&input);
    dbg!(output);
}

#[derive(Debug)]
struct Record {
    time: u32,
    distance: u32,
}

impl Record {
    fn count_ways_to_beat(&self) -> u32 {
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

fn parse_records(input: &str) -> IResult<&str, Vec<Record>> {
    let (input, times) = tag("Time:")
        .precedes(preceded(
            multispace1,
            separated_list1(multispace1, complete::u32),
        ))
        .terminated(line_ending)
        .parse(input)?;
    let (input, distances) = tag("Distance:")
        .precedes(preceded(
            multispace1,
            separated_list1(multispace1, complete::u32),
        ))
        .terminated(line_ending)
        .parse(input)?;
    assert_eq!(times.len(), distances.len());
    let records = times
        .iter()
        .enumerate()
        .map(|(idx, time)| {
            let distance = distances[idx];
            Record {
                time: *time,
                distance,
            }
        })
        .collect();
    Ok((input, records))
}

fn part1(input: &str) -> u32 {
    let (_, records) = parse_records(input).expect("input should be parsable");
    records
        .iter()
        .map(|record| record.count_ways_to_beat())
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case() {
        let input = read_to_string("input/p1.example.txt").expect("example input should exist");
        let result = part1(&input);
        assert_eq!(result, 288);
    }
}
