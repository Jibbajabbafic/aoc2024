use itertools::Itertools;

advent_of_code::solution!(9);

struct Disk {
    uncompressed: Vec<i32>,
    structure: Vec<File>,
}

#[derive(Debug, Clone, Copy)]
struct File {
    id: u32,
    file_len: u32,
    free_len: u32,
}

impl Disk {
    fn parse_input(input: &str) -> Self {
        let mut uncompressed = vec![];
        let mut structure: Vec<File> = vec![];
        let mut id = 0;

        for mut chunk in &input.chars().map(|c| c.to_digit(10).unwrap()).chunks(2) {
            let file = File {
                id,
                file_len: chunk.next().unwrap(),
                free_len: chunk.next().unwrap_or(0),
            };
            for _ in 0..file.file_len {
                uncompressed.push(id as i32);
            }
            for _ in 0..file.free_len {
                uncompressed.push(-1);
            }
            id += 1;
            structure.push(file);
        }
        // println!("{:?}", uncompressed);
        Self {
            uncompressed,
            structure,
        }
    }
}

fn calculate_checksum(input: &Vec<i32>) -> u64 {
    let mut checksum: u64 = 0;
    for (i, &num) in input.iter().enumerate() {
        if num == -1 {
            // skip empty space
            continue;
        }
        checksum += i as u64 * num as u64;
    }
    checksum
}

pub fn part_one(input: &str) -> Option<u64> {
    let disk = Disk::parse_input(input);
    // reverse iterator
    let mut ri = disk.uncompressed.len() - 1;
    let mut compressed: Vec<i32> = vec![];
    for (i, &num) in disk.uncompressed.iter().enumerate() {
        if num == -1 {
            // get digit from end of map
            let mut last_digit = disk.uncompressed[ri];
            while last_digit == -1 {
                // skip over -1s at end
                ri -= 1;
                last_digit = disk.uncompressed[ri];
            }
            compressed.push(last_digit);
            ri -= 1;
        } else {
            compressed.push(num);
        }
        if i == ri {
            // last digit met iterator, so everything compressed
            break;
        }
    }
    // println!("{:?}", compressed);
    Some(calculate_checksum(&compressed))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut disk = Disk::parse_input(input);
    // iterate over files in descending file id (reverse iterator ri)
    for last_file_id in (0..disk.structure.len()).rev() {
        let (ri, &mut last_file) = disk
            .structure
            .iter_mut()
            .find_position(|f| f.id == last_file_id as u32)
            .unwrap();
        // find free space (forward iterator i)
        let mut i = 0;
        while i < ri {
            let file = &mut disk.structure[i];
            if file.free_len >= last_file.file_len {
                // found free space - move the last file
                // see if any leftover free space
                let mut moved_file = last_file.clone();
                moved_file.free_len = file.free_len - last_file.file_len;
                // compressed left, so no free space on old one
                file.free_len = 0;
                disk.structure.remove(ri);
                disk.structure.insert(i + 1, moved_file);
                // need to add on free space from moved file
                // can use original ri index again since it has moved left now
                let left_file = disk.structure.get_mut(ri).unwrap();
                left_file.free_len += last_file.file_len + last_file.free_len;
                // done with this file now, go to next last file
                break;
            }
            i += 1;
        }
    }
    let mut compressed: Vec<i32> = vec![];
    for file in disk.structure {
        for _ in 0..file.file_len {
            compressed.push(file.id as i32);
        }
        for _ in 0..file.free_len {
            compressed.push(-1);
        }
    }
    // println!("{:?}", compressed);
    Some(calculate_checksum(&compressed))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
