//! renderer agnostic bitmap representation

/// rgb representation of a color
#[derive(Copy, Clone, PartialEq)]
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
