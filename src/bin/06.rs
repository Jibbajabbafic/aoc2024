use std::collections::HashSet;

use advent_of_code::Matrix;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    // println!("input:\n{}", input);
    // mtx origin is top left, and y goes top to bottom! (-1 y is up)
    let mut mtx = Matrix::from_input(input);
    // clockwise facing directions
    let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut facing = 0;
    let mut cur_pos: (i32, i32) = mtx.find('^').unwrap();
    let mut pos_set: HashSet<(i32, i32)> = HashSet::new();
    let mut marker = '^';
    // simulate guard moves
    loop {
        // store visited positions in set
        pos_set.insert(cur_pos);
        // debug view
        // println!("mtx:");
        // mtx.print();
        let next_x = cur_pos.0 + dirs[facing].0;
        let next_y = cur_pos.1 + dirs[facing].1;
        if let Some(next_char) = mtx.get(next_x, next_y) {
            if next_char == '#' {
                // rotate 90 degrees right
                facing = (facing + 1) % dirs.len();
                marker = match facing {
                    0 => '^',
                    1 => '>',
                    2 => 'v',
                    3 => '<',
                    _ => '-',
                };
                mtx.set(cur_pos.0, cur_pos.1, marker);
            } else {
                mtx.set(cur_pos.0, cur_pos.1, 'X');
                cur_pos = (next_x, next_y);
                mtx.set(cur_pos.0, cur_pos.1, marker);
            }
        } else {
            // guard left the area!
            break;
        }
    }
    Some(pos_set.len() as u32)
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
