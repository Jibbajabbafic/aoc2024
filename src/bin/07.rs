advent_of_code::solution!(7);

struct Equation {
    target: u64,
    values: Vec<u64>,
}

impl Equation {
    pub fn parse_line(line: &str) -> Self {
        let (target, values) = line.split_once(':').unwrap();
        Self {
            target: target.parse().unwrap(),
            values: values
                .split_whitespace()
                .map(|d| d.parse().unwrap())
                .collect(),
        }
    }

    pub fn is_solveable_part1(&self) -> bool {
        self.is_solvable(self.target, &self.values)
    }

    fn is_solvable(&self, target: u64, vec: &Vec<u64>) -> bool {
        if vec.len() == 1 {
            return target == vec[0];
        }
        let ops = vec.len() - 1;
        // try all possible combinations of ops
        for combo in 0..(1 << ops) {
            // 0000 = + + + +
            // 0003 = * * + +
            let mut test_result = vec[0];
            for (i, num) in vec.iter().skip(1).enumerate() {
                if ((combo >> i) & 1) == 0 {
                    test_result += num;
                } else {
                    test_result *= num;
                }
            }
            if test_result == target {
                return true;
            }
        }
        false
    }

    pub fn is_solveable_part2(&self) -> bool {
        self.eval(self.values[0], self.values[1..].to_vec())
    }

    fn eval(&self, prev: u64, nums: Vec<u64>) -> bool {
        if nums.is_empty() {
            return prev == self.target;
        }
        return self.eval(prev + nums[0], nums[1..].to_vec())
            || self.eval(prev * nums[0], nums[1..].to_vec())
            || self.eval(concat(prev, nums[0]), nums[1..].to_vec());
    }
}

fn concat(a: u64, b: u64) -> u64 {
    let digits = (b).checked_ilog10().unwrap_or(0) + 1;
    b + (a * 10_u64.pow(digits))
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut total = 0;
    for line in input.lines() {
        let test = Equation::parse_line(line);
        if test.is_solveable_part1() {
            total += test.target;
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total = 0;
    for line in input.lines() {
        let test = Equation::parse_line(line);
        if test.is_solveable_part2() {
            total += test.target;
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }

    #[test]
    fn test_part_two_parts() {
        let result = part_two("156: 15 6");
        assert_eq!(result, Some(156));
        let result = part_two("7290: 6 8 6 15");
        assert_eq!(result, Some(7290));
        let result = part_two("192: 17 8 14");
        assert_eq!(result, Some(192));
    }

    #[test]
    fn test_concat() {
        let result = concat(123, 456);
        assert_eq!(result, 123456);
        let result = concat(1, 234);
        assert_eq!(result, 1234);
    }
}
