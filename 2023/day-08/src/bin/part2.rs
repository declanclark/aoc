use core::panic;
use std::{collections::HashMap, fs::read_to_string, usize};

use nom::{
    bytes::complete::{is_a, tag, take_until},
    character::complete::{line_ending, multispace1},
    multi::fold_many1,
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

fn main() {
    let input = read_to_string("input/p1.txt").expect("input should exist");
    let output = part2(&input);
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

fn part2(input: &str) -> usize {
    let (_, (directions, map)) = parse_map(input).expect("input should be parsable");
    let directions = directions.chars().collect::<Vec<char>>();
    let mut current_nodes = map
        .keys()
        .filter(|k| (**k).ends_with("A"))
        .copied()
        .collect::<Vec<&str>>();
    let mut steps = 0;
    let mut steps_per_route = Vec::new();
    while steps_per_route.len() != current_nodes.len() {
        let direction = directions[steps % directions.len()];
        steps += 1;
        for i in 0..current_nodes.len() {
            let n = current_nodes[i];
            if !n.ends_with("Z") {
                let (l, r) = map.get(n).expect("map should contain any referenced node");
                match direction {
                    'L' => current_nodes[i] = l,
                    'R' => current_nodes[i] = r,
                    c => {
                        panic!("impossible case: {c}");
                    }
                }
                let next = current_nodes[i];
                if next.ends_with("Z") {
                    steps_per_route.push(steps);
                }
            }
        }
    }
    steps_per_route
        .into_iter()
        .fold(1, |acc: usize, s| num::integer::lcm(acc, s))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case() {
        let input = read_to_string("input/p2.example.txt").expect("example input should exist");
        let result = part2(&input);
        assert_eq!(result, 6);
    }
}
