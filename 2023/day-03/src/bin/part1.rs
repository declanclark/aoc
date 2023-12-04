use std::fs::read_to_string;

fn main() {
    let lines = read_lines("input/p1.txt");
    let output = part1(lines);
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

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_digit(10)
}

fn part1(lines: Vec<Vec<char>>) -> i32 {
    assert!(!lines.is_empty());
    let rows = lines.len();
    let cols = lines[0].len();
    let mut current_num = String::new();
    let mut part_number_sum = 0;
    for i in 0..rows {
        let current_line = &lines[i];
        let mut part_number_found = false;
        for j in 0..cols {
            let current_char = current_line[j];
            let is_digit = current_char.is_digit(10);
            if is_digit {
                if !part_number_found {
                    let check_above = i > 0;
                    let check_below = i < (rows - 1);
                    let check_left = j > 0;
                    let check_right = j < (cols - 1);
                    part_number_found |= check_above && is_symbol(lines[i - 1][j]);
                    part_number_found |= check_right && is_symbol(lines[i][j + 1]);
                    part_number_found |= check_below && is_symbol(lines[i + 1][j]);
                    part_number_found |= check_left && is_symbol(lines[i][j - 1]);
                    part_number_found |=
                        check_above && check_left && is_symbol(lines[i - 1][j - 1]);
                    part_number_found |=
                        check_above && check_right && is_symbol(lines[i - 1][j + 1]);
                    part_number_found |=
                        check_below && check_left && is_symbol(lines[i + 1][j - 1]);
                    part_number_found |=
                        check_below && check_right && is_symbol(lines[i + 1][j + 1]);
                }
                current_num.push(current_char);
            }
            if !is_digit || j == cols - 1 {
                if part_number_found {
                    part_number_sum += current_num
                        .parse::<i32>()
                        .expect("part number should be an integer");
                }
                current_num.clear();
                part_number_found = false;
            }
        }
    }
    part_number_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case() {
        let lines = read_lines("input/p1.example.txt");
        let result = part1(lines);
        assert_eq!(result, 4361);
    }

    #[test]
    fn smoke() {
        let lines = read_lines("input/p1.test.txt");
        let result = part1(lines);
        assert_eq!(result, 8730);
    }
}
