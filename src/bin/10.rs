use std::collections::{HashSet, VecDeque};

use advent_of_code::{Matrix, Position};

advent_of_code::solution!(10);

const START: char = '0';
const END: char = '9';

struct Map {
    trailheads: Vec<Trailhead>,
    grid: Matrix,
}

impl Map {
    fn from_input(input: &str) -> Self {
        let mtx = Matrix::from_input(input);
        Self {
            trailheads: mtx
                .find_all(START)
                .unwrap()
                .iter()
                .map(|x| Trailhead {
                    start: *x,
                    score: 0,
                    ends: HashSet::default(),
                    rank: 0,
                })
                .collect(),
            grid: mtx,
        }
    }

    fn get_next_trail_char(input: char) -> char {
        char::from_u32(input as u32 + 1).unwrap()
    }

    fn score_trails(&mut self) {
        for trail in &mut self.trailheads {
            let mut search_positions = VecDeque::from([trail.start]);
            while let Some(cur_pos) = search_positions.pop_front() {
                // search through all positions
                let cur_char = self.grid.get(cur_pos).unwrap();
                let next_char = Map::get_next_trail_char(cur_char);
                // check all adjacent positions
                let next_positions = [
                    cur_pos + Position::UP,
                    cur_pos + Position::RIGHT,
                    cur_pos + Position::DOWN,
                    cur_pos + Position::LEFT,
                ];
                let valid_positions = next_positions.iter().filter_map(|&p| {
                    if let Some(c) = self.grid.get(p) {
                        if c == next_char {
                            if next_char == END {
                                // This position reached the end, so save it and stop searching
                                if trail.ends.insert(p) {
                                    trail.score += 1;
                                }
                            } else {
                                // found valid trail pos, so return it to keep searching
                                return Some(p);
                            }
                        }
                    }
                    None
                });
                for p in valid_positions {
                    // println!("valid pos: {:?}", p);
                    search_positions.push_back(p);
                }
            }
        }
    }

    fn rank_trails(&mut self) {
        for trail in &mut self.trailheads {
            let mut search_positions = VecDeque::from([trail.start]);
            let mut unique_trails = 0;
            while let Some(cur_pos) = search_positions.pop_front() {
                // search through all positions
                let cur_char = self.grid.get(cur_pos).unwrap();
                let next_char = Map::get_next_trail_char(cur_char);
                // check all adjacent positions
                let next_positions = [
                    cur_pos + Position::UP,
                    cur_pos + Position::RIGHT,
                    cur_pos + Position::DOWN,
                    cur_pos + Position::LEFT,
                ];
                let valid_positions: Vec<Position> = next_positions
                    .iter()
                    .filter_map(|&p| {
                        if let Some(c) = self.grid.get(p) {
                            if c == next_char {
                                if next_char == END {
                                    // increment pos
                                    unique_trails += 1;
                                } else {
                                    // found valid trail pos, so return it to keep searching
                                    return Some(p);
                                }
                            }
                        }
                        None
                    })
                    .collect();
                // increment trails if there's a split
                // decrement by 1 if deadend
                // unique_trails += valid_positions.len() - 1;
                for p in valid_positions {
                    // println!("valid pos: {:?}", p);
                    search_positions.push_back(p);
                }
            }
            trail.rank = unique_trails;
        }
    }
}

#[derive(Debug)]
struct Trailhead {
    start: Position,
    ends: HashSet<Position>,
    score: u32,
    rank: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Map::from_input(input);
    map.score_trails();
    Some(map.trailheads.iter().map(|t| t.score).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = Map::from_input(input);
    map.rank_trails();
    Some(map.trailheads.iter().map(|t| t.rank).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(3));
    }
}
