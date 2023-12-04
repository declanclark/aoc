use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let lines = read_lines("input/p1.txt");
    let output = part2(lines);
    dbg!(output);
}

fn read_lines(filename: &str) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    for line in read_to_string(filename)
        .expect("unable to read file")
        .lines()
    {
        result.push(line.chars().collect());
    }
    return result;
}

fn is_star(c: char) -> bool {
    c == '*'
}

fn part2(lines: Vec<Vec<char>>) -> i32 {
    assert!(!lines.is_empty());
    let rows = lines.len();
    let cols = lines[0].len();
    let mut current_num = String::new();
    let mut possible_gear_parts: HashMap<String, Vec<i32>> = HashMap::new();
    for i in 0..rows {
        let current_line = &lines[i];
        let mut gear_candidates: HashSet<String> = HashSet::new();
        for j in 0..cols {
            let current_char = current_line[j];
            let is_digit = current_char.is_digit(10);
            if is_digit {
                current_num.push(current_char);
                let check_above = i > 0;
                let check_below = i < (rows - 1);
                let check_left = j > 0;
                let check_right = j < (cols - 1);
                if check_above && is_star(lines[i - 1][j]) {
                    gear_candidates.insert(format!("{},{}", i - 1, j));
                }
                if check_right && is_star(lines[i][j + 1]) {
                    gear_candidates.insert(format!("{},{}", i, j + 1));
                }
                if check_below && is_star(lines[i + 1][j]) {
                    gear_candidates.insert(format!("{},{}", i + 1, j));
                }
                if check_left && is_star(lines[i][j - 1]) {
                    gear_candidates.insert(format!("{},{}", i, j - 1));
                }
                if check_above && check_left && is_star(lines[i - 1][j - 1]) {
                    gear_candidates.insert(format!("{},{}", i - 1, j - 1));
                }
                if check_above && check_right && is_star(lines[i - 1][j + 1]) {
                    gear_candidates.insert(format!("{},{}", i - 1, j + 1));
                }
                if check_below && check_left && is_star(lines[i + 1][j - 1]) {
                    gear_candidates.insert(format!("{},{}", i + 1, j - 1));
                }
                if check_below && check_right && is_star(lines[i + 1][j + 1]) {
                    gear_candidates.insert(format!("{},{}", i + 1, j + 1));
                }
            }
            if !is_digit || j == cols - 1 {
                for part in &gear_candidates {
                    let part_num = current_num
                        .parse::<i32>()
                        .expect("part number should be an integer");
                    match possible_gear_parts.get_mut(part) {
                        Some(vec) => {
                            vec.push(part_num);
                        }
                        None => {
                            possible_gear_parts.insert(part.to_string(), vec![part_num]);
                        }
                    }
                }
                current_num.clear();
                gear_candidates.clear();
            }
        }
    }
    let mut gear_ratio_sum = 0;
    for part_nums in possible_gear_parts.values() {
        if part_nums.len() == 2 {
            gear_ratio_sum += part_nums[0] * part_nums[1];
        }
    }
    gear_ratio_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case() {
        let lines = read_lines("input/p1.example.txt");
        let result = part2(lines);
        assert_eq!(result, 467835);
    }
}
