use core::panic;
use std::fs::read_to_string;

fn main() {
    let lines = read_lines("input/p1.txt");
    let output = part1(lines);
    dbg!(output);
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename)
        .expect("unable to read file")
        .lines()
    {
        result.push(line.to_string());
    }
    return result;
}

fn part1(lines: Vec<String>) -> i32 {
    const RED: i32 = 12;
    const GREEN: i32 = 13;
    const BLUE: i32 = 14;
    let output = lines
        .iter()
        .map(|line| {
            // start analysing string from idx 5, we don't need the "Game " part of the string
            let mut reduced_line = &line[5..];
            let mut chars = reduced_line.chars();
            let colon_pos = chars
                .position(|char| char == ':')
                .expect("string should contain a colon after game id");
            let game_id = reduced_line[0..colon_pos]
                .parse::<i32>()
                .expect("game id should be an integer");
            reduced_line = &reduced_line[colon_pos + 1..];
            let subsets: Vec<&str> = reduced_line.split(';').map(|s| s.trim()).collect();
            for subset in subsets {
                let counts: Vec<&str> = subset.split(',').map(|s| s.trim()).collect();
                for count in counts {
                    let possible: bool;
                    let parts: Vec<&str> = count.split_whitespace().collect();
                    let num_cubes = parts[0]
                        .parse::<i32>()
                        .expect("expected integer number of cubes");
                    let colour = parts[1];
                    match colour {
                        "red" => possible = num_cubes <= RED,
                        "green" => possible = num_cubes <= GREEN,
                        "blue" => possible = num_cubes <= BLUE,
                        _ => panic!("unexpected colour string"),
                    }
                    if !possible {
                        return 0;
                    }
                }
            }
            game_id
        })
        .sum::<i32>();
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case() {
        let lines = read_lines("input/p1.example.txt");
        let result = part1(lines);
        assert_eq!(result, 8);
    }
}
