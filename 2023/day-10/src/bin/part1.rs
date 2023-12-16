use core::panic;
use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("input/p1.txt").expect("input should exist");
    let output = part1(&input);
    dbg!(output);
}

#[derive(Debug, Clone, Copy)]
enum PipeType {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

#[derive(Debug, Clone, Copy)]
enum TileType {
    PipeType(PipeType),
    Ground,
    Start,
}

#[derive(Debug)]
struct Field {
    tiles: Vec<Vec<TileType>>,
}

impl Field {
    fn get_start_tile(&self) -> (usize, usize) {
        for i in 0..self.tiles.len() {
            for j in 0..self.tiles[i].len() {
                if matches!(self.tiles[i][j], TileType::Start) {
                    return (i, j);
                }
            }
        }
        panic!("expected start tile to be present in field")
    }

    fn get_tile_type(&self, position: (usize, usize)) -> TileType {
        let (i, j) = position;
        self.tiles[i][j]
    }

    fn find_loop_len(&self) -> usize {
        let rows = self.tiles.len();
        assert_ne!(rows, 0);
        let cols = self.tiles[0].len();
        let s = self.get_start_tile();
        let mut seen = HashSet::from([(s.0, s.1)]);
        let mut q = VecDeque::from([(s.0, s.1)]);
        loop {
            if q.is_empty() {
                break;
            }
            let current = q.pop_front().expect("should not be empty");
            let current_type = self.get_tile_type(current);
            // try to move north
            let north_idx = current.0.checked_sub(1);
            let north_tile = if let Some(idx) = north_idx {
                Some(self.get_tile_type((idx, current.1)))
            } else {
                None
            };
            let current_allows_north = match current_type {
                TileType::Start
                | TileType::PipeType(PipeType::Vertical)
                | TileType::PipeType(PipeType::NorthEast)
                | TileType::PipeType(PipeType::NorthWest) => true,
                _ => false,
            };
            let north_allows_current = match north_tile {
                Some(
                    TileType::PipeType(PipeType::Vertical)
                    | TileType::PipeType(PipeType::SouthWest)
                    | TileType::PipeType(PipeType::SouthEast),
                ) => true,
                _ => false,
            };
            if current_allows_north && north_allows_current {
                let north_pos = (north_idx.unwrap(), current.1);
                if !seen.contains(&north_pos) {
                    seen.insert(north_pos);
                    q.push_back(north_pos);
                }
            }
            // try to move east
            let east_idx = current.1 + 1;
            let east_tile = if east_idx < cols {
                Some(self.get_tile_type((current.0, east_idx)))
            } else {
                None
            };
            let current_allows_east = match current_type {
                TileType::Start
                | TileType::PipeType(PipeType::Horizontal)
                | TileType::PipeType(PipeType::NorthEast)
                | TileType::PipeType(PipeType::SouthEast) => true,
                _ => false,
            };
            let east_allows_current = match east_tile {
                Some(
                    TileType::PipeType(PipeType::Horizontal)
                    | TileType::PipeType(PipeType::SouthWest)
                    | TileType::PipeType(PipeType::NorthWest),
                ) => true,
                _ => false,
            };
            if current_allows_east && east_allows_current {
                let east_pos = (current.0, east_idx);
                if !seen.contains(&east_pos) {
                    seen.insert(east_pos);
                    q.push_back(east_pos);
                }
            }
            // try to move south
            let south_idx = current.0 + 1;
            let south_tile = if south_idx < rows {
                Some(self.get_tile_type((south_idx, current.1)))
            } else {
                None
            };
            let current_allows_south = match current_type {
                TileType::Start
                | TileType::PipeType(PipeType::Vertical)
                | TileType::PipeType(PipeType::SouthEast)
                | TileType::PipeType(PipeType::SouthWest) => true,
                _ => false,
            };
            let south_allows_current = match south_tile {
                Some(
                    TileType::PipeType(PipeType::Vertical)
                    | TileType::PipeType(PipeType::NorthEast)
                    | TileType::PipeType(PipeType::NorthWest),
                ) => true,
                _ => false,
            };
            if current_allows_south && south_allows_current {
                let south_pos = (south_idx, current.1);
                if !seen.contains(&south_pos) {
                    seen.insert(south_pos);
                    q.push_back(south_pos);
                }
            }
            // try to move west
            let west_idx = current.1.checked_sub(1);
            let west_tile = if let Some(idx) = west_idx {
                Some(self.get_tile_type((current.0, idx)))
            } else {
                None
            };
            let current_allows_west = match current_type {
                TileType::Start
                | TileType::PipeType(PipeType::Horizontal)
                | TileType::PipeType(PipeType::NorthWest)
                | TileType::PipeType(PipeType::SouthWest) => true,
                _ => false,
            };
            let west_allows_current = match west_tile {
                Some(
                    TileType::PipeType(PipeType::Horizontal)
                    | TileType::PipeType(PipeType::NorthEast)
                    | TileType::PipeType(PipeType::SouthEast),
                ) => true,
                _ => false,
            };
            if current_allows_west && west_allows_current {
                let west_pos = (current.0, west_idx.unwrap());
                if !seen.contains(&west_pos) {
                    seen.insert(west_pos);
                    q.push_back(west_pos);
                }
            }
        }
        seen.len() / 2
    }
}

fn parse_field(input: &str) -> Field {
    Field {
        tiles: input
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(|row| {
                row.chars()
                    .filter_map(|c| match c {
                        '.' => Some(TileType::Ground),
                        'S' => Some(TileType::Start),
                        '|' => Some(TileType::PipeType(PipeType::Vertical)),
                        '-' => Some(TileType::PipeType(PipeType::Horizontal)),
                        'L' => Some(TileType::PipeType(PipeType::NorthEast)),
                        'J' => Some(TileType::PipeType(PipeType::NorthWest)),
                        '7' => Some(TileType::PipeType(PipeType::SouthWest)),
                        'F' => Some(TileType::PipeType(PipeType::SouthEast)),
                        _ => None,
                    })
                    .collect::<Vec<TileType>>()
            })
            .collect::<Vec<Vec<TileType>>>(),
    }
}

fn part1(input: &str) -> usize {
    let field = parse_field(input);
    field.find_loop_len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case() {
        let input = read_to_string("input/p1.example.txt").expect("example input should exist");
        let result = part1(&input);
        assert_eq!(result, 4);
    }

    #[test]
    fn second_case() {
        let input = read_to_string("input/p1.example_2.txt").expect("example input should exist");
        let result = part1(&input);
        assert_eq!(result, 8);
    }
}
