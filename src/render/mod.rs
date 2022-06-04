//! renderer trait and renderer implementations

pub mod ansi;

use crate::bitmap::Bitmap;

/// describes how the rest of the program should interact with renderers
pub trait Renderer {
    /// render the specified bitmap
    fn render(&mut self, bitmap: &Bitmap);

    /// get size of renderer
    fn get_size(&self) -> (usize, usize);
}
