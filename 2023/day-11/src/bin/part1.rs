use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input/p1.txt").expect("input should exist");
    let output = part1(&input);
    dbg!(output);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum DataPoint {
    Nothing,
    Galaxy(usize),
}

type Image = Vec<Vec<DataPoint>>;

fn parse_img(input: &str) -> Image {
    let mut next_id = 1;
    input
        .split('\n')
        .filter_map(|r| match r {
            "" => None,
            _ => Some(
                r.chars()
                    .map(|c| match c {
                        '.' => DataPoint::Nothing,
                        _ => DataPoint::Galaxy({
                            let tmp = next_id;
                            next_id += 1;
                            tmp
                        }),
                    })
                    .collect::<Vec<DataPoint>>(),
            ),
        })
        .collect::<Image>()
}

fn transpose(img: &Image) -> Image {
    if img.is_empty() || img[0].is_empty() {
        return vec![];
    }

    let row_count = img.len();
    let col_count = img[0].len();
    let mut transposed = vec![vec![DataPoint::Nothing; row_count]; col_count];

    for (i, row) in img.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            transposed[j][i] = ch;
        }
    }

    transposed
}

fn expand_rows(img: &Image) -> Image {
    assert!(!img.is_empty());
    let mut new_img = img.clone();
    let col_count = img[0].len();
    let rows_to_expand = img
        .iter()
        .enumerate()
        .filter_map(
            |(i, row)| match row.iter().all(|c| *c == DataPoint::Nothing) {
                true => Some(i),
                false => None,
            },
        )
        .collect::<Vec<usize>>();
    let mut expanded_rows = 0;
    let empty_row = Vec::from_iter(std::iter::repeat(DataPoint::Nothing).take(col_count));
    for row_to_expand in rows_to_expand {
        new_img.insert(row_to_expand + expanded_rows, empty_row.clone());
        expanded_rows += 1;
    }
    new_img
}

fn expand(img: &Image) -> Image {
    let expanded_rows = expand_rows(img);
    let transposed = transpose(&expanded_rows);
    let expanded_cols = expand_rows(&transposed);
    transpose(&expanded_cols)
}

fn get_galaxy_locations(img: &Image) -> HashMap<usize, (usize, usize)> {
    let mut map: HashMap<usize, (usize, usize)> = HashMap::new();
    img.iter().enumerate().for_each(|(i, r)| {
        r.iter().enumerate().for_each(|(j, dp)| {
            if let DataPoint::Galaxy(id) = dp {
                map.insert(*id, (i, j));
            }
        })
    });
    map
}

fn part1(input: &str) -> usize {
    let mut img = parse_img(input);
    img = expand(&img);
    let galaxies = get_galaxy_locations(&img);
    // manhattan distance for each pair
    let mut distance_sum = 0;
    let galaxy_count = galaxies.len();
    for i in 1..=galaxy_count {
        for j in (i + 1)..=galaxy_count {
            let first_pos = galaxies.get(&i).expect("id should be found in map");
            let second_pos = galaxies.get(&j).expect("id should be found in map");
            let distance = second_pos.0.abs_diff(first_pos.0) + second_pos.1.abs_diff(first_pos.1);
            distance_sum += distance;
        }
    }
    distance_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case() {
        let input = read_to_string("input/p1.example.txt").expect("example input should exist");
        let result = part1(&input);
        assert_eq!(result, 374);
    }
}
