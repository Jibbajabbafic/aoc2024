advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total_safe = 0;
    // Parse line
    // Split whitespace
    // Iterate numbers
    for line in input.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|reading| reading.parse::<i32>().expect("not a u32!"))
            .collect();
        // Check rules
        if find_unsafe_level(&levels) == None {
            total_safe += 1;
        }
    }
    // Increment count if passed
    Some(total_safe)
}

fn find_unsafe_level(levels: &Vec<i32>) -> Option<usize> {
    let mut last_sign = 0;
    // get diff, if sign of diff matches to last continue, else unsafe and skip
    for (i, num) in levels.iter().enumerate().skip(1) {
        let diff: i32 = num - levels[i - 1];
        let sign = diff.signum();
        if last_sign != 0 && sign != last_sign {
            // Not increasing/decreasing consistently!
            return Some(i);
        }
        last_sign = sign;
        if diff.abs() < 1 || diff.abs() > 3 {
            // Changing by less than 1 or more than 3
            return Some(i);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total_safe = 0;
    // Parse line
    // Split whitespace
    // Iterate numbers
    for line in input.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|reading| reading.parse::<i32>().expect("not a u32!"))
            .collect();
        // Check rules
        if is_report_safe_dapened(&levels) {
            total_safe += 1;
        }
    }
    // Increment count if passed
    Some(total_safe)
}

fn is_report_safe_dapened(levels: &Vec<i32>) -> bool {
    if let Some(i) = find_unsafe_level(&levels) {
        // Unsafe levels!
        // Try removing all previous values to see if it helps
        for x in 0..=i {
            // Check if removing value at index makes it safe
            let mut damped_levels_try = levels.clone();
            damped_levels_try.remove(x);
            if find_unsafe_level(&damped_levels_try) == None {
                // Removing value before unsafe one made it safe, all good!
                return true;
            }
        }
    } else {
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }
}
