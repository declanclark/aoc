use core::panic;
use std::{cmp, fs::read_to_string};

fn main() {
    let lines = read_lines("input/p1.txt");
    let output = part2(lines);
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

fn part2(lines: Vec<String>) -> i32 {
    let output = lines
        .iter()
        .map(|line| {
            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;
            let mut chars = line.chars();
            let colon_pos = chars
                .position(|char| char == ':')
                .expect("string should contain a colon after game id");
            let reduced_line = &line[colon_pos + 1..];
            let subsets: Vec<&str> = reduced_line.split(';').map(|s| s.trim()).collect();
            for subset in subsets {
                let counts: Vec<&str> = subset.split(',').map(|s| s.trim()).collect();
                for count in counts {
                    let parts: Vec<&str> = count.split_whitespace().collect();
                    let num_cubes = parts[0]
                        .parse::<i32>()
                        .expect("expected integer number of cubes");
                    let colour = parts[1];
                    match colour {
                        "red" => max_red = cmp::max(max_red, num_cubes),
                        "green" => max_green = cmp::max(max_green, num_cubes),
                        "blue" => max_blue = cmp::max(max_blue, num_cubes),
                        _ => panic!("unexpected colour string"),
                    }
                }
            }
            max_red * max_green * max_blue
        })
        .sum::<i32>();
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case() {
        let lines = read_lines("input/p2.example.txt");
        let result = part2(lines);
        assert_eq!(result, 2286);
    }
}
