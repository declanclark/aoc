use std::fs::read_to_string;

use nom::{
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    IResult,
};

fn main() {
    let input = read_to_string("input/p1.txt").expect("input should exist");
    let output = part1(&input);
    dbg!(output);
}

fn parse_report(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    let (input, data) =
        separated_list1(line_ending, separated_list1(space1, complete::i64))(input)?;
    Ok((input, data))
}

fn extrapolate(data: &Vec<i64>) -> i64 {
    let mut nums = data.clone();
    let mut end_nums: Vec<i64> = vec![*nums.last().expect("should exist")];
    loop {
        if nums.iter().all(|x| x == &0) {
            break;
        }
        nums = (0..(nums.len() - 1))
            .map(|i| {
                let first = nums[i];
                let second = nums[i + 1];
                second - first
            })
            .collect::<Vec<i64>>();
        end_nums.push(*nums.last().expect("should exist"));
    }
    end_nums.iter().fold(0, |acc: i64, x| acc + x)
}

fn part1(input: &str) -> i64 {
    let (_, report) = parse_report(input).expect("input should be parsable");
    report.iter().map(|data| extrapolate(data)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case() {
        let input = read_to_string("input/p1.example.txt").expect("example input should exist");
        let result = part1(&input);
        assert_eq!(result, 114);
    }
}
