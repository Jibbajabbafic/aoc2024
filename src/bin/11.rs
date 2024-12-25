use std::{
    collections::{HashMap, VecDeque},
    default,
};

advent_of_code::solution!(11);

struct FunStones {
    current: Vec<u64>,
}

impl FunStones {
    fn from_input(input: &str) -> Self {
        Self {
            current: input
                .split_ascii_whitespace()
                .map(|c| c.parse().unwrap())
                .collect(),
        }
    }

    fn blink(&mut self) {
        let mut next: Vec<u64> = vec![];
        for &stone in &self.current {
            if stone == 0 {
                next.push(1);
            } else if stone.to_string().chars().count() % 2 == 0 {
                // do the split
                let digits = stone.to_string();
                let mid = digits.len() / 2;
                let (d1, d2) = digits.split_at(mid);
                next.push(d1.parse().unwrap());
                next.push(d2.parse().unwrap());
            } else {
                next.push(stone * 2024);
            }
        }
        self.current = next;
    }

    // depth first search to get the totals instead of manually doing each iteration
    fn find_total_dfs(&self, max_depth: u32) {
        let mut stone_queue = VecDeque::<u64>::from(self.current.clone());
        // let mut solved_stones = HashMap::<u64, u64>::new();

        // do something for dfs and memoisation
        // traverse stones, track each one in a stack, store result as we leave the stack again?
        while let Some(stone) = stone_queue.pop_front() {
            if stone == 0 {
                next.push(1);
            } else if stone.to_string().chars().count() % 2 == 0 {
                // do the split
                let digits = stone.to_string();
                let mid = digits.len() / 2;
                let (d1, d2) = digits.split_at(mid);
                next.push(d1.parse().unwrap());
                next.push(d2.parse().unwrap());
            } else {
                next.push(stone * 2024);
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut stones = FunStones::from_input(input);
    for _ in 0..25 {
        stones.blink();
    }
    Some(stones.current.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut stones = FunStones::from_input(input);
    for _ in 0..75 {
        stones.blink();
    }
    Some(stones.current.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
