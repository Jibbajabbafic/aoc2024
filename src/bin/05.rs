use std::collections::HashMap;

advent_of_code::solution!(5);

type Rules = HashMap<u32, Vec<u32>>;
type Update = Vec<u32>;

pub fn part_one(input: &str) -> Option<u32> {
    // split into rules and updates
    let mut iter = input.split("\n\n");

    // parse rules
    let mut rules = Rules::new();
    if let Some(rule_input) = iter.next() {
        for line in rule_input.lines() {
            if let [before, after] = line
                .split("|")
                .map(|c| c.parse::<u32>().unwrap())
                .collect::<Vec<_>>()[..]
            {
                if let Some(v) = rules.get_mut(&before) {
                    v.push(after);
                } else {
                    rules.insert(before, vec![after]);
                }
            }
        }
    }
    // parse updates
    let mut updates: Vec<Update> = vec![];
    if let Some(update_input) = iter.next() {
        updates = update_input
            .lines()
            .map(|line| line.split(",").map(|c| c.parse::<u32>().unwrap()).collect())
            .collect()
    }
    println!("{:?}", rules);
    println!("{:?}", updates);

    let mut total = 0;
    for u in updates {
        if is_order_valid(&u, &rules) {
            total += get_mid(u);
        }
    }
    Some(total)
}

fn is_order_valid(update: &Update, rules: &Rules) -> bool {
    for (i, num) in update.iter().enumerate() {
        if let Some(banned_nums) = rules.get(num) {
            for b in banned_nums {
                if update[0..i].contains(b) {
                    return false;
                }
            }
        }
    }
    true
}

fn get_mid(update: Update) -> u32 {
    update[update.len() / 2]
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
