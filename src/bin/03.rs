use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    Some(parse_muls(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(parse_input(input))
}

fn parse_input(input: &str) -> u32 {
    let mut instr_en = true;
    let mut sum = 0;
    // One named capture group per instruction
    let instr_patterns = [
        r"(?<mul>mul\((?<num1>\d{1,3}),(?<num2>\d{1,3})\))",
        r"(?<do>do\(\))",
        r"(?<dont>don't\(\))",
    ];
    let re_pattern = format!(r"({})", instr_patterns.join("|"));
    let re = Regex::new(&re_pattern);
    match re {
        Err(e) => eprintln!("regex pattern error: {}", e),
        Ok(re) => {
            for m in re.captures_iter(input) {
                if m.name("do") != None {
                    instr_en = true;
                } else if m.name("dont") != None {
                    instr_en = false;
                } else if instr_en && m.name("mul") != None {
                    sum += &m["num1"].parse::<u32>().unwrap() * &m["num2"].parse::<u32>().unwrap();
                }
            }
        }
    }
    sum
}

fn parse_muls(input: &str) -> u32 {
    let mut total = 0;
    let mut state = 0;
    let mut num1_buff = String::new();
    let mut num2_buff = String::new();
    for c in input.chars() {
        match state {
            0 => {
                if c == 'm' {
                    state += 1;
                } else {
                    state = 0;
                }
            }
            1 => {
                if c == 'u' {
                    state += 1;
                } else {
                    state = 0;
                }
            }
            2 => {
                if c == 'l' {
                    state += 1;
                } else {
                    state = 0;
                }
            }
            3 => {
                if c == '(' {
                    state += 1;
                } else {
                    state = 0;
                }
            }
            4 => {
                // First digit of num1
                if c.is_numeric() {
                    // Clear buffer 1 ready for new digits
                    num1_buff.clear();
                    num1_buff.push(c);
                    state += 1;
                } else {
                    state = 0;
                }
            }
            5 => {
                if num1_buff.len() < 3 && c.is_numeric() {
                    num1_buff.push(c);
                } else if c == ',' {
                    // Got the first full number!
                    state += 1;
                } else {
                    state = 0;
                }
            }
            6 => {
                // First digit of num2
                if c.is_numeric() {
                    // Clear buffer 2 ready for new digits
                    num2_buff.clear();
                    num2_buff.push(c);
                    state += 1;
                } else {
                    state = 0;
                }
            }
            7 => {
                if num2_buff.len() < 3 && c.is_numeric() {
                    num2_buff.push(c);
                } else if c == ')' {
                    // Got the second full number!
                    // Do the multiplication!
                    total += num1_buff.parse::<u32>().unwrap() * num2_buff.parse::<u32>().unwrap();
                    // Can reset the state again now
                    state = 0;
                } else {
                    state = 0;
                }
            }
            _ => state = 0,
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
