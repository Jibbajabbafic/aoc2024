use std::collections::{HashMap, HashSet};

use advent_of_code::Position;
use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(Debug, Default)]
struct AntennaMap {
    antennas: HashMap<char, Vec<Position>>,
    width: i32,
    height: i32,
}

impl AntennaMap {
    pub fn parse_input(input: &str) -> Self {
        let mut map = AntennaMap::default();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate().filter(|(_, c)| *c != '.') {
                let pos = Position::new(x as i32, y as i32);
                if map.antennas.contains_key(&c) {
                    map.antennas.get_mut(&c).unwrap().push(pos);
                } else {
                    map.antennas.insert(c, vec![pos]);
                }
            }
        }
        map.height = input.lines().count() as i32;
        map.width = input.lines().next().unwrap().chars().count() as i32;
        map
    }
}

fn is_in_bounds(pos: Position, width: i32, height: i32) -> bool {
    return pos.x >= 0 && pos.x < width && pos.y >= 0 && pos.y < height;
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = AntennaMap::parse_input(&input);
    let mut antinodes = HashSet::<Position>::new();
    for values in map.antennas.values() {
        for (&a, &b) in values.iter().tuple_combinations() {
            let anti_node1 = a + ((b - a) * 2);
            if is_in_bounds(anti_node1, map.width, map.height) {
                antinodes.insert(anti_node1);
            }
            let anti_node2 = b + ((a - b) * 2);
            if is_in_bounds(anti_node2, map.width, map.height) {
                antinodes.insert(anti_node2);
            }
        }
    }

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = AntennaMap::parse_input(&input);
    let mut antinodes = HashSet::<Position>::new();
    for values in map.antennas.values() {
        for (&a, &b) in values.iter().tuple_combinations() {
            let mut i = 0;
            loop {
                let antinode = a + ((b - a) * i);
                if is_in_bounds(antinode, map.width, map.height) {
                    antinodes.insert(antinode);
                } else {
                    break;
                }
                i += 1;
            }
            i = 0;
            loop {
                let antinode = b + ((a - b) * i);
                if is_in_bounds(antinode, map.width, map.height) {
                    antinodes.insert(antinode);
                } else {
                    break;
                }
                i += 1;
            }
        }
    }

    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(9));
    }
}
