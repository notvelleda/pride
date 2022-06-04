//! renderer agnostic bitmap representation

use std::{
    fmt,
    str::FromStr,
};

/// rgb representation of a color
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    /// lets us create a color with less typing
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

impl From<[u8; 3]> for Color {
    fn from(f: [u8; 3]) -> Self {
        Self {
            red: f[0],
            green: f[1],
            blue: f[2],
        }
    }
}

impl FromStr for Color {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // get iterator over string
        let mut chars = input.chars();

        // get first character
        if let Some(c) = chars.next() {
            if c != '#' { // make sure it's valid
                Err("invalid color".to_string())
            } else {
                // parse rest of string as a number in hex notation
                let num = match usize::from_str_radix(chars.as_str(), 16) {
                    Ok(num) => num,
                    Err(err) => return Err(err.to_string()),
                };
        
                // extract red, green, blue components from number
                if num > 0xffffff {
                    Err("number too big".to_string())
                } else {
                    Ok(Self {
                        red: ((num >> 16) & 0xff) as u8,
                        green: ((num >> 8) & 0xff) as u8,
                        blue: (num & 0xff) as u8,
                    })
                }
            }
        } else {
            Err("invalid color".to_string())
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let num =
            ((self.red as usize) << 16) |
            ((self.green as usize) << 8) |
            (self.blue as usize);
        write!(f, "#{:06x}", num)
    }
}

/// simple bitmap, used to store the flag as a grid of squares
pub struct Bitmap {
    /// data of the bitmap, stored as a 2d vec
    pub data: Vec<Vec<Color>>,

    /// width of the bitmap
    pub width: usize,

    /// height of the bitmap
    pub height: usize,
}

impl Bitmap {
    /// create a new bitmap of the provided size
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![vec![Color::new(0, 0, 0); width]; height],
            width, height,
        }
    }

    /// set a cell in the bitmap
    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            self.data[y][x] = color;
        }
    }

    /// get a cell in the bitmap
    pub fn get(&self, x: usize, y: usize) -> Option<Color> {
        if x < self.width && y < self.height {
            Some(self.data[y][x])
        } else {
            None
        }
    }

    /// draw a rectangle
    pub fn draw_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: Color) {
        // make sure we're not out of bounds
        if x >= self.width || y >= self.height {
            return;
        }

        for y2 in y..y + height {
            for x2 in x..x + width {
                self.set(x2, y2, color);
            }
        }
    }
}
