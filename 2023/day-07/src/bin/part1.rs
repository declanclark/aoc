use core::panic;
use std::{cmp::Ordering, collections::HashMap, fs::read_to_string};

use nom::{
    bytes::complete::take,
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq, Clone, Copy)]
struct Hand {
    card_values: [u8; 5],
    hand_type: HandType,
    bid: usize,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_type_cmp = self.hand_type.cmp(&other.hand_type);
        if !hand_type_cmp.is_eq() {
            return hand_type_cmp;
        }
        // if hand type is equal, we must check the individual cards
        for i in 0..self.card_values.len() {
            let this_value_cmp = self.card_values[i].cmp(&other.card_values[i]);
            if this_value_cmp.is_eq() {
                continue;
            }
            return this_value_cmp;
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

// won't make it generic, for now
#[derive(Debug)]
struct BTreeNode {
    val: Hand,
    left: Option<Box<BTreeNode>>,
    right: Option<Box<BTreeNode>>,
}

#[derive(Debug)]
struct BTree {
    root: Option<Box<BTreeNode>>,
}

impl From<BTreeNode> for Option<Box<BTreeNode>> {
    fn from(value: BTreeNode) -> Self {
        Some(Box::new(value))
    }
}

impl BTreeNode {
    fn new(v: Hand) -> Self {
        BTreeNode {
            val: v,
            left: None,
            right: None,
        }
    }
}

impl BTree {
    fn new() -> Self {
        BTree { root: None }
    }

    fn insert(&mut self, v: Hand) {
        if self.root.is_none() {
            self.root = BTreeNode::new(v).into();
            return;
        }
        let mut q: Vec<&mut Box<BTreeNode>> = Vec::new();
        let root = self.root.as_mut().unwrap();
        q.push(root);
        while let Some(node) = q.pop() {
            if v > node.val {
                let right = &mut node.right;
                match right {
                    Some(n) => {
                        q.push(n);
                    }
                    None => {
                        *right = BTreeNode::new(v).into();
                    }
                }
            } else {
                let left = &mut node.left;
                match left {
                    Some(n) => {
                        q.push(n);
                    }
                    None => {
                        *left = BTreeNode::new(v).into();
                    }
                }
            }
        }
    }

    fn inorder_rec(values: &mut Vec<Hand>, node: &Box<BTreeNode>) {
        if let Some(ref left) = node.left {
            BTree::inorder_rec(values, left);
        }
        values.push(node.val);
        if let Some(ref right) = node.right {
            BTree::inorder_rec(values, right);
        }
    }

    fn inorder(&self) -> Vec<Hand> {
        let mut results = Vec::new();
        if self.root.is_none() {
            return results;
        }
        if let Some(ref root) = self.root {
            BTree::inorder_rec(&mut results, root);
        }
        results
    }
}

impl HandType {
    fn check(card_values: &[u8; 5]) -> Self {
        let mut map: HashMap<u8, u32> = HashMap::new();
        card_values
            .iter()
            .for_each(|v| *map.entry(*v).or_insert(0) += 1);
        match map.len() {
            1 => HandType::FiveOfAKind,
            2 => match map.into_values().max() {
                Some(4) => HandType::FourOfAKind,
                Some(3) => HandType::FullHouse,
                _ => panic!("impossible"),
            },
            3 => match map.into_values().max() {
                Some(3) => HandType::ThreeOfAKind,
                Some(2) => HandType::TwoPair,
                _ => panic!("impossible"),
            },
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!("impossible"),
        }
    }
}

fn parse_game(input: &str) -> IResult<&str, BTree> {
    let mut tree = BTree::new();
    let (input, output) = separated_list1(
        line_ending,
        separated_pair(take(5usize), space1, complete::u32),
    )(input)?;
    output.into_iter().for_each(|(vals, bid): (&str, u32)| {
        let card_values = vals
            .chars()
            .map(|v| match v.to_digit(10) {
                Some(digit) => digit.try_into().expect("should not be bigger than u8"),
                None => match v {
                    'T' => 10,
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => panic!("recieved card with unknown value"),
                },
            })
            .collect::<Vec<u8>>();
        let card_values: [u8; 5] = card_values
            .as_slice()
            .try_into()
            .expect("should have exactly five card values");
        let hand_type = HandType::check(&card_values);
        tree.insert(Hand {
            card_values,
            hand_type,
            bid: bid.try_into().expect("should be able to convert to usize"),
        });
    });
    Ok((input, tree))
}

fn part1(input: &str) -> usize {
    let (_, hands) = parse_game(input).expect("input should be parsable");
    BTree::inorder(&hands)
        .into_iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i + 1))
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case() {
        let input = read_to_string("input/p1.example.txt").expect("example input should exist");
        let result = part1(&input);
        assert_eq!(result, 6440);
    }
}
