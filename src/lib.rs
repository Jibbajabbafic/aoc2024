pub mod template;

// Use this file to add helper functions and additional modules.
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

    pub fn get(&self, x: i32, y: i32) -> Option<char> {
        if self.is_in_bounds(x, y) {
            Some(self.data[self.get_index(x, y)])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: i32, y: i32, value: char) {
        if !self.is_in_bounds(x, y) {
            return;
        }
        let i = self.get_index(x, y);
        self.data[i] = value;
    }

    pub fn find(&self, in_char: char) -> Option<(i32, i32)> {
        for (i, cur_char) in self.data.iter().enumerate() {
            if *cur_char == in_char {
                return Some(((i % self.width) as i32, (i / self.width) as i32));
            }
        }
        None
    }

    pub fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        return x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32;
    }

    fn get_index(&self, x: i32, y: i32) -> usize {
        (x + y * self.width as i32) as usize
    }
}
