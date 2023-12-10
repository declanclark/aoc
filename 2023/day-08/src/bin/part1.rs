use std::{collections::HashMap, fs::read_to_string};

use nom::{
    bytes::complete::{is_a, tag, take_until},
    character::complete::{line_ending, multispace1},
    multi::fold_many1,
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

fn main() {
    let input = read_to_string("input/p1.txt").expect("input should exist");
    let output = part1(&input);
    dbg!(output);
}

fn node(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    let (input, output) = separated_pair(
        take_until(" "),
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(take_until(","), tag(", "), take_until(")")),
            tag(")"),
        ),
    )(input)?;
    Ok((input, output))
}

fn nodes(input: &str) -> IResult<&str, HashMap<&str, (&str, &str)>> {
    fold_many1(
        terminated(node, line_ending),
        HashMap::new,
        |mut map: HashMap<&str, (&str, &str)>, (node, edges)| {
            map.insert(node, edges);
            map
        },
    )(input)
}

fn parse_map(input: &str) -> IResult<&str, (&str, HashMap<&str, (&str, &str)>)> {
    separated_pair(is_a("LR"), multispace1, nodes)(input)
}

fn part1(input: &str) -> u32 {
    let (_, (directions, map)) = parse_map(input).expect("input should be parsable");
    let directions = directions.chars().collect::<Vec<char>>();
    let mut current_node = "AAA";
    let mut steps = 0;
    while current_node != "ZZZ" {
        let direction = directions[steps % directions.len()];
        let (l, r) = map
            .get(current_node)
            .expect("map should have any referenced node");
        match direction {
            'L' => current_node = l,
            'R' => current_node = r,
            c => {
                panic!("received impossible case: {}", c);
            }
        }
        steps += 1;
    }
    steps.try_into().expect("should fit in u32")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case() {
        let input = read_to_string("input/p1.example.txt").expect("example input should exist");
        let result = part1(&input);
        assert_eq!(result, 2);
    }

    #[test]
    fn repeated_case() {
        let input = read_to_string("input/p1.example2.txt").expect("example input should exist");
        let result = part1(&input);
        assert_eq!(result, 6);
    }
}
