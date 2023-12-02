use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let output = part2("input/p1.txt");
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

fn part2(filename: &str) -> i32 {
    let lines = read_lines(filename);
    let digit_words: HashMap<&str, char> = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]
    .iter()
    .cloned()
    .collect();
    let mut sum = 0;
    for line in lines {
        let mut digits = Vec::new();
        let mut c = line.chars().peekable();
        while let Some(&current_char) = c.peek() {
            if current_char.is_digit(10) {
                digits.push(current_char);
                c.next();
            } else {
                let mut lookahead = c.clone();
                let mut lookahead_count = 0;
                let mut digit_word = String::new();
                while let Some(&next_char) = lookahead.peek() {
                    lookahead_count += 1;
                    if !next_char.is_alphabetic() || lookahead_count > 6 {
                        break;
                    }
                    digit_word.push(next_char);
                    lookahead.next();
                    if let Some(&digit) = digit_words.get(digit_word.as_str()) {
                        digits.push(digit);
                        break;
                    }
                }
                c.next();
            }
        }
        assert!(!digits.is_empty());
        let mut str_num = String::new();
        str_num.push(digits[0]);
        str_num.push(digits[digits.len() - 1]);
        print!("{line}:{}:{str_num}\n", digits.len());
        let this_num = str_num
            .parse::<i32>()
            .expect("string should hold a two digit integer");
        sum += this_num;
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case() {
        let result = part2("input/p2.example.txt");
        assert_eq!(result, 281);
    }
}
