use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    // Input string parsing boilerplate
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        left.push(iter.next()?.parse().expect("Not an i32"));
        right.push(iter.next()?.parse().expect("Not an i32"));
    }

    left.sort();
    right.sort();

    let mut sum = 0;
    for i in 0..left.len() {
        sum += left[i].abs_diff(right[i]);
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Input string parsing boilerplate
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        left.push(iter.next()?.parse().expect("Not an i32"));
        right.push(iter.next()?.parse().expect("Not an i32"));
    }

    let mut prev_sim_scores = HashMap::<i32, i32>::new();
    let mut total_sim_score: u32 = 0;

    // Iterate left list
    for lnum in left {
        // See if score already calculated for this number
        if !prev_sim_scores.contains_key(&lnum) {
            // Calculate similarity score for this number
            let mut mult = 0;
            for rnum in &right {
                if *rnum == lnum {
                    mult += 1;
                }
            }
            // Cache the score for future loops
            prev_sim_scores.insert(lnum, lnum * mult);
        }
        // Increment score
        total_sim_score += *(prev_sim_scores.get(&lnum)?) as u32;
    }
    Some(total_sim_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
