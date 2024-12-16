advent_of_code::solution!(7);

struct TestValue {
    target: u64,
    values: Vec<u64>,
}

impl TestValue {
    pub fn is_solveable(&self) -> bool {
        let ops = self.values.len() - 1;
        // try all possible combinations of ops
        for combo in 0..(1 << ops) {
            // 0000 = + + + +
            // 0003 = * * + +
            let mut test_sum = self.values[0];
            for (i, num) in self.values.iter().skip(1).enumerate() {
                if ((combo >> i) & 1) == 0 {
                    test_sum += num;
                } else {
                    test_sum *= num;
                }
            }
            if test_sum == self.target {
                return true;
            }
        }
        false
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut total = 0;
    for line in input.lines() {
        let mut split = line.split(':');
        let test = TestValue {
            target: split.next().unwrap().parse::<u64>().unwrap(),
            values: split
                .next()
                .unwrap()
                .trim()
                .split(' ')
                .map(|d| d.parse::<u64>().unwrap())
                .collect(),
        };
        if test.is_solveable() {
            total += test.target;
        }
    }
    Some(total)
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
