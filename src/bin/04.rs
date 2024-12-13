advent_of_code::solution!(4);

#[derive(Debug, Clone)]
struct Matrix {
    data: Vec<char>,
    width: usize,
    height: usize,
}

impl Matrix {
    fn new(data: Vec<char>, width: usize, height: usize) -> Self {
        Self {
            data,
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> char {
        self.data[x + y * self.width]
    }
}

// hardcoded offsets of each window because I'm lazy
const WINDOW_L_R: [(usize, usize); 4] = [(0, 0), (1, 0), (2, 0), (3, 0)];
const WINDOW_U_D: [(usize, usize); 4] = [(0, 0), (0, 1), (0, 2), (0, 3)];
const WINDOW_LU_RD: [(usize, usize); 4] = [(0, 0), (1, 1), (2, 2), (3, 3)];
const WINDOW_LD_RU: [(usize, usize); 4] = [(0, 3), (1, 2), (2, 1), (3, 0)];

const WORD: &str = "XMAS";
const REV_WORD: &str = "SAMX";

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    // println!("{}", input);
    // get forward and reverse version of the word
    const WORD: &str = "XMAS";
    // parse input into 2d matrix
    let mtx = Matrix::new(
        input.lines().map(|x| x.chars()).flatten().collect(),
        input.lines().next().unwrap().chars().count(),
        input.lines().count(),
    );
    // println!("{:?}", mtx);

    // left-right checking
    for y in 0..mtx.height {
        for x in 0..mtx.width - WORD.len() + 1 {
            let slice: String = WINDOW_L_R
                .iter()
                .map(|i| mtx.get(x + i.0, y + i.1))
                .collect();
            if is_word_match(&slice) {
                total += 1;
            }
        }
    }
    // assert_eq!(total, 5);

    // up-down checking
    for y in 0..mtx.height - WORD.len() + 1 {
        for x in 0..mtx.width {
            let slice: String = WINDOW_U_D
                .iter()
                .map(|i| mtx.get(x + i.0, y + i.1))
                .collect();
            if is_word_match(&slice) {
                total += 1;
            }
        }
    }
    // assert_eq!(total, 8);

    // leftup-rightdown checking
    for y in 0..mtx.height - WORD.len() + 1 {
        for x in 0..mtx.width - WORD.len() + 1 {
            let slice: String = WINDOW_LU_RD
                .iter()
                .map(|i| mtx.get(x + i.0, y + i.1))
                .collect();
            if is_word_match(&slice) {
                total += 1;
            }
        }
    }

    // leftdown-rightup checking
    for y in 0..mtx.height - WORD.len() + 1 {
        for x in 0..mtx.width - WORD.len() + 1 {
            let slice: String = WINDOW_LD_RU
                .iter()
                .map(|i| mtx.get(x + i.0, y + i.1))
                .collect();
            if is_word_match(&slice) {
                total += 1;
            }
        }
    }
    Some(total)
}

const MAS_WINDOW1: [(usize, usize); 3] = [(0, 0), (1, 1), (2, 2)];
const MAS_WINDOW2: [(usize, usize); 3] = [(0, 2), (1, 1), (2, 0)];

pub fn part_two(input: &str) -> Option<u32> {
    let mut total = 0;
    // println!("{}", input);
    // parse input into 2d matrix
    let mtx = Matrix::new(
        input.lines().map(|x| x.chars()).flatten().collect(),
        input.lines().next().unwrap().chars().count(),
        input.lines().count(),
    );
    // println!("{:?}", mtx);

    // leftdown-rightup checking
    for y in 0..mtx.height - 2 {
        for x in 0..mtx.width - 2 {
            let slice1: String = MAS_WINDOW1
                .iter()
                .map(|i| mtx.get(x + i.0, y + i.1))
                .collect();
            if slice1 == "MAS" || slice1 == "SAM" {
                let slice2: String = MAS_WINDOW2
                    .iter()
                    .map(|i| mtx.get(x + i.0, y + i.1))
                    .collect();
                if slice2 == "MAS" || slice2 == "SAM" {
                    total += 1;
                }
            }
        }
    }

    Some(total)
}

fn is_word_match(slice: &String) -> bool {
    slice == WORD || slice == REV_WORD
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
