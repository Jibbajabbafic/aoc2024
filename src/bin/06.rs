use std::collections::HashSet;

use advent_of_code::{Direction, Matrix, Position};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    // println!("input:\n{}", input);
    // mtx origin is top left, and y goes top to bottom! (-1 y is up)
    let mut mtx = Matrix::from_input(input);
    let mut facing = Direction::Up;
    let mut cur_pos: Position = mtx.find('^').unwrap();
    let mut next_pos: Position;
    let mut pos_set: HashSet<Position> = HashSet::new();
    let mut marker = '^';
    // simulate guard moves
    loop {
        // store visited positions in set
        pos_set.insert(cur_pos);
        // debug view
        // println!("mtx:");
        // mtx.print();
        next_pos = cur_pos.take_step(facing);
        if let Some(next_char) = mtx.get(next_pos) {
            if next_char == '#' {
                // rotate 90 degrees right
                facing = facing.rotate_clockwise();
                marker = match facing {
                    Direction::Up => '^',
                    Direction::Right => '>',
                    Direction::Down => 'v',
                    Direction::Left => '<',
                };
                mtx.set(&cur_pos, marker);
            } else {
                mtx.set(&cur_pos, 'X');
                cur_pos = next_pos;
                mtx.set(&cur_pos, marker);
            }
        } else {
            // guard left the area!
            break;
        }
    }
    Some(pos_set.len() as u32)
}

// see if turning right from current facing would join up with any previous positions + facings eventually
fn joins_up_to_loop(
    matrix: &Matrix,
    prev_collisions: &PreviousCollisions,
    start_pos: Position,
    start_facing: Direction,
) -> bool {
    let mut test_pos = start_pos;
    // start off turning assuming from new obstacle
    let mut test_facing = start_facing.rotate_clockwise();
    let mut test_colls = prev_collisions.clone();
    loop {
        // check if loop completed
        if test_colls.positions.contains(&(test_pos, test_facing))
            || (test_pos == start_pos && test_facing == start_facing)
        {
            // back to a previous collision, so it's a loop!
            return true;
        }
        // check next pos
        let next_pos = test_pos.take_step(test_facing);
        match matrix.get(next_pos) {
            None => {
                // out of bounds - not a loop, so stop checking
                return false;
            }
            Some('#') => {
                // store pos and facing before collision, can see if it joins up
                test_colls.positions.insert((test_pos, test_facing));
                // obstacle - turn right again and continue
                test_facing = test_facing.rotate_clockwise();
            }
            Some(_) => {
                // no obstacle
                // take the step
                test_pos = next_pos;
            }
        }
    }
}

#[derive(Default, Clone)]
struct PreviousCollisions {
    positions: HashSet<(Position, Direction)>,
}

pub fn part_two(input: &str) -> Option<u32> {
    // mtx origin is top left, and y goes top to bottom! (-1 y is up)
    let mut mtx = Matrix::from_input(input);
    let mut facing = Direction::Up;
    let start_pos: Position = mtx.find('^').unwrap();
    let mut cur_pos = start_pos;
    let mut obstacle_options: Vec<Position> = vec![];
    let mut marker = '^';
    let mut _move_iter = 0;
    let mut prev_colls: PreviousCollisions = Default::default();
    let mut pos_set: HashSet<Position> = HashSet::new();
    // simulate guard moves
    loop {
        // debug view
        // println!("-------- mtx {_move_iter} --------");
        // mtx.print();

        // store visited positions in set
        pos_set.insert(cur_pos);

        let next_pos = cur_pos.take_step(facing);
        match mtx.get(next_pos) {
            None => {
                // guard left area
                break;
            }
            Some('#') => {
                // detected obstacle
                // store pos and facing before collision, can see if it joins up
                prev_colls.positions.insert((cur_pos, facing));
                // do the rotation for current pos
                facing = facing.rotate_clockwise();
                marker = match facing {
                    Direction::Up => '^',
                    Direction::Right => '>',
                    Direction::Down => 'v',
                    Direction::Left => '<',
                };
                mtx.set(&cur_pos, marker);
            }
            Some(_) => {
                // path clear
                // check for loops
                // cannot use starting pos!
                if next_pos != start_pos && !pos_set.contains(&next_pos) {
                    let mut test_mtx = mtx.clone();
                    test_mtx.set(&next_pos, '#');
                    // println!("original:");
                    // mtx.print();
                    // println!("test:");
                    // test_mtx.print();
                    if joins_up_to_loop(&test_mtx, &prev_colls, cur_pos, facing) {
                        // obstacle at next pos would make a loop, so save value
                        obstacle_options.push(next_pos);
                        // mtx.set(&next_pos, 'O');
                        // println!("LOOP {} FOUND!:", obstacle_options.len());
                        // mtx.print();
                    }
                }

                // proceed to next pos
                let pos_marker = match mtx.get(cur_pos) {
                    Some('|') | Some('-') => '+',
                    _ => match facing {
                        Direction::Up | Direction::Down => '|',
                        Direction::Left | Direction::Right => '-',
                    },
                };
                mtx.set(&cur_pos, pos_marker);
                cur_pos = next_pos;
                mtx.set(&cur_pos, marker);
            }
        }
        _move_iter += 1;
    }
    let solution = obstacle_options.len() as u32;
    assert_ne!(solution, 1872, "previous wrong answer!");
    assert_ne!(solution, 202, "previous wrong answer!");
    Some(solution)
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
        assert_eq!(result, Some(6));
    }
    #[test]
    fn test_part_two1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(1));
    }
    #[test]
    fn test_part_two2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(0));
    }
}
