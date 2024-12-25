use std::collections::HashMap;

advent_of_code::solution!(11);

struct FunStones {
    current: Vec<u64>,
    solved_stones_cache: HashMap<u32, HashMap<u64, u64>>,
}

impl FunStones {
    fn from_input(input: &str) -> Self {
        Self {
            current: input
                .split_ascii_whitespace()
                .map(|c| c.parse().unwrap())
                .collect(),
            solved_stones_cache: HashMap::new(),
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
    fn find_total_dfs(&mut self, max_depth: u32) -> u64 {
        let mut total_stones = 0;
        for stone in self.current.clone() {
            total_stones += self.recursive_solve_stone(stone, max_depth, 1);
        }
        return total_stones;
    }

    fn recursive_solve_stone(&mut self, stone: u64, depth_limit: u32, cur_depth: u32) -> u64 {
        if let Some(depth_cache) = self.solved_stones_cache.get(&cur_depth) {
            if let Some(solution) = depth_cache.get(&stone) {
                // this stone has already been solved for this level of depth, so stop calculating
                return *solution;
            }
        }
        let next_stones = FunStones::apply_rules(stone);
        // reached limit so stop and return current answer, this will be summed as we go back up the stack
        if cur_depth == depth_limit {
            let total_stones = next_stones.len() as u64;
            // cache this result for future searches
            if let Some(depth_cache) = self.solved_stones_cache.get_mut(&cur_depth) {
                depth_cache.insert(stone, total_stones);
            } else {
                self.solved_stones_cache
                    .insert(cur_depth, HashMap::from([(stone, total_stones)]));
            }
            return total_stones;
        }
        // search one stone first to max depth, add up totals from both searches
        let mut total_stones = 0;
        total_stones += self.recursive_solve_stone(next_stones[0], depth_limit, cur_depth + 1);
        // then search second one if exists
        if next_stones.len() > 1 {
            total_stones += self.recursive_solve_stone(next_stones[1], depth_limit, cur_depth + 1);
        }
        // cache this result for future searches
        if let Some(depth_cache) = self.solved_stones_cache.get_mut(&cur_depth) {
            depth_cache.insert(stone, total_stones);
        } else {
            self.solved_stones_cache
                .insert(cur_depth, HashMap::from([(stone, total_stones)]));
        }
        return total_stones;
    }

    fn apply_rules(stone: u64) -> Vec<u64> {
        if stone == 0 {
            return vec![1];
        } else if stone.to_string().chars().count() % 2 == 0 {
            // do the split
            let digits = stone.to_string();
            let mid = digits.len() / 2;
            let (d1, d2) = digits.split_at(mid);
            return vec![d1.parse().unwrap(), d2.parse().unwrap()];
        } else {
            return vec![stone * 2024];
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut stones = FunStones::from_input(input);
    let dfs_answer = stones.find_total_dfs(25);
    Some(dfs_answer)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut stones = FunStones::from_input(input);
    let dfs_answer = stones.find_total_dfs(75);
    Some(dfs_answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
