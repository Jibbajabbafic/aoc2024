use std::collections::{HashSet, VecDeque};

use advent_of_code::{Matrix, Position};

advent_of_code::solution!(12);

struct Garden {
    grid: Matrix,
    regions: Vec<Region>,
    total_price: u64,
}

impl Garden {
    fn from_input(input: &str) -> Self {
        Self {
            grid: Matrix::from_input(&input),
            regions: Vec::new(),
            total_price: 0,
        }
    }

    fn find_regions(&mut self) {
        // check all spaces, track which ones already used in regions
        let mut used_pos = HashSet::<Position>::new();
        for y in 0..self.grid.height as i32 {
            for x in 0..self.grid.width as i32 {
                let pos = Position::new(x, y);
                if used_pos.contains(&pos) {
                    // this pos is already in a region
                    continue;
                }
                // new region to search!
                let mut region = Region {
                    character: self.grid.get(pos).unwrap(),
                    area: 0,
                    perimiter: 0,
                    price: 0,
                };
                // begin search for connected areas with same char
                let mut search_queue = VecDeque::<Position>::from([pos]);
                while let Some(new_pos) = search_queue.pop_front() {
                    // don't search previous pos again
                    if used_pos.contains(&new_pos) {
                        continue;
                    }
                    // store this pos so we don't search it again
                    used_pos.insert(new_pos);
                    let adjacent_positions = Position::get_adjacted_positions(new_pos);
                    // see which positons are connected to this region
                    let connected_positions: Vec<&Position> = adjacent_positions
                        .iter()
                        .filter(|&p| {
                            // must be in bounds
                            self.grid.is_in_bounds(p)
                            // character must match
                                && self.grid.get(*p).unwrap() == region.character
                        })
                        .collect();
                    // update region stats for this pos
                    region.area += 1;
                    region.perimiter += 4 - connected_positions.len() as u32;
                    // add connections to keep the search going
                    for p in connected_positions {
                        search_queue.push_back(*p);
                    }
                }
                // region fully explored
                region.price = region.area * region.perimiter;
                self.total_price += region.price as u64;
                self.regions.push(region);
            }
        }
    }
}

struct Region {
    character: char,
    area: u32,
    perimiter: u32,
    price: u32,
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut garden = Garden::from_input(input);
    garden.find_regions();
    Some(garden.total_price)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(140));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(772));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
