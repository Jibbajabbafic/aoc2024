use std::ops::{Add, Mul, Sub};

pub mod template;

// Use this file to add helper functions and additional modules.

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn take_step(&self, dir: Direction) -> Position {
        match dir {
            Direction::Up => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Position {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

impl Add for Position {
    type Output = Position;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Position {
    type Output = Position;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<i32> for Position {
    type Output = Position;
    fn mul(self, rhs: i32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    pub fn rotate_clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Matrix {
    data: Vec<char>,
    width: usize,
    height: usize,
}

impl Matrix {
    pub fn new(data: Vec<char>, width: usize, height: usize) -> Self {
        Self {
            data,
            width,
            height,
        }
    }

    pub fn from_input(input: &str) -> Self {
        Self {
            data: input.lines().flat_map(|x| x.chars()).collect(),
            width: input.lines().next().unwrap().chars().count(),
            height: input.lines().count(),
        }
    }

    pub fn print(&self) {
        let s = self
            .data
            .chunks(self.width)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");
        println!("{}", s);
    }

    pub fn get(&self, pos: Position) -> Option<char> {
        if self.is_in_bounds(&pos) {
            Some(self.data[self.get_index(&pos)])
        } else {
            None
        }
    }

    pub fn set(&mut self, pos: &Position, value: char) {
        if !self.is_in_bounds(&pos) {
            return;
        }
        let i = self.get_index(&pos);
        self.data[i] = value;
    }

    pub fn find(&self, in_char: char) -> Option<Position> {
        for (i, cur_char) in self.data.iter().enumerate() {
            if *cur_char == in_char {
                return Some(self.get_position(i));
            }
        }
        None
    }

    pub fn is_in_bounds(&self, pos: &Position) -> bool {
        return pos.x >= 0 && pos.x < self.width as i32 && pos.y >= 0 && pos.y < self.height as i32;
    }

    fn get_index(&self, pos: &Position) -> usize {
        (pos.x as usize + pos.y as usize * self.width) as usize
    }

    fn get_position(&self, index: usize) -> Position {
        Position {
            x: (index % self.width) as i32,
            y: (index / self.width) as i32,
        }
    }
}
