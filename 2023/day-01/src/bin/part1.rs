use std::fs::read_to_string;

fn main() {
    let output = part1("input/p1.txt");
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

fn part1(filename: &str) -> i32 {
    let lines = read_lines(filename);
    let mut sum = 0;
    for line in lines {
        let mut str_num = String::new();
        let mut last_seen_char = 'X';
        for c in line.chars() {
            if c.is_digit(10) {
                if str_num.chars().count() == 0 {
                    str_num.push(c);
                }
                last_seen_char = c;
            }
        }
        if str_num.chars().count() != 0 {
            str_num.push(last_seen_char);
        }
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
        let result = part1("input/p1.example.txt");
        assert_eq!(result, 142);
    }
}
