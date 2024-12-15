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
    // println!("{:?}", rules);
    // println!("{:?}", updates);

    let mut total = 0;
    for u in updates {
        if is_order_valid(&u, &rules) {
            total += get_mid(&u);
        }
    }
    Some(total)
}

fn is_order_valid(update: &Update, rules: &Rules) -> bool {
    // skip first since nothing to check before that
    for (i, num) in update.iter().enumerate().skip(1) {
        if let Some(banned_nums) = rules.get(num) {
            let slice = &update[0..i];
            for b in banned_nums {
                if slice.contains(b) {
                    return false;
                }
            }
        }
    }
    true
}

fn get_mid(update: &Update) -> u32 {
    update[update.len() / 2]
}

fn fix_order<'a>(update: &'a mut Update, rules: &Rules) -> &'a mut Update {
    // println!("failing: {:?}", update);
    // skip first since nothing to swap with
    let mut i = 1;
    'outer: while i < update.len() {
        let num = update[i];
        if let Some(banned_nums) = rules.get(&num) {
            for j in 0..i {
                for banned in banned_nums {
                    if &update[j] == banned {
                        // swap values
                        update[i] = update[j];
                        update[j] = num;
                        // start checking rules again from subindex j
                        i = j;
                        continue 'outer;
                    }
                }
            }
        }
        i += 1;
    }
    // println!("fixed: {:?}", update);
    update
}

pub fn part_two(input: &str) -> Option<u32> {
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
    // sort each rule to make ordering consistent
    for afters in rules.values_mut() {
        afters.sort();
    }

    // parse updates
    let mut updates: Vec<Update> = vec![];
    if let Some(update_input) = iter.next() {
        updates = update_input
            .lines()
            .map(|line| line.split(",").map(|c| c.parse::<u32>().unwrap()).collect())
            .collect()
    }
    // println!("rules: {:?}", rules);
    // println!("updates: {:?}", updates);

    let mut total = 0;
    for u in updates.iter_mut() {
        if !is_order_valid(&u, &rules) {
            let fixed = fix_order(u, &rules);
            total += get_mid(&fixed);
        }
    }
    Some(total)
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
