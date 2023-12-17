use std::{cmp::max, cmp::min, collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input/p1.txt").expect("input should exist");
    let output = part2(&input, 1000000);
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

fn part2(input: &str, expansion_factor: usize) -> usize {
    let img = parse_img(input);
    let galaxies = get_galaxy_locations(&img);
    // manhattan distance for each pair
    let mut distance_sum = 0;
    let galaxy_count = galaxies.len();
    for i in 1..=galaxy_count {
        for j in (i + 1)..=galaxy_count {
            let (start_row, first_col) = galaxies.get(&i).expect("id should be found in map");
            let (end_row, second_col) = galaxies.get(&j).expect("id should be found in map");
            let mut y_distance = 0;
            // we assign ids via traversing rows then cols so we know we will only move down the
            // image from the lower id
            // check whether a row would be expanded or if it is just a singular row
            for k in (*start_row + 1)..=*end_row {
                y_distance += match img[k].iter().all(|&dp| dp == DataPoint::Nothing) {
                    true => expansion_factor,
                    false => 1,
                };
            }
            let start_col = min(first_col, second_col);
            let end_col = max(first_col, second_col);
            let mut x_distance = 0;
            // check whether a column would be expanded or if it is just a singular column
            for k in (*start_col + 1)..=*end_col {
                let mut galaxy_found = false;
                for l in 0..img.len() {
                    if matches!(img[l][k], DataPoint::Galaxy(_)) {
                        galaxy_found = true;
                    }
                }
                if galaxy_found {
                    x_distance += 1;
                } else {
                    x_distance += expansion_factor;
                }
            }
            distance_sum += y_distance + x_distance;
        }
    }
    distance_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expand_ten() {
        let input = read_to_string("input/p1.example.txt").expect("example input should exist");
        let result = part2(&input, 10);
        assert_eq!(result, 1030);
    }

    #[test]
    fn expand_hundred() {
        let input = read_to_string("input/p1.example.txt").expect("example input should exist");
        let result = part2(&input, 100);
        assert_eq!(result, 8410);
    }
}
