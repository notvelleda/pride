//! renderer that writes to an image

use crate::bitmap::{Bitmap, Color};
use image::ImageBuffer;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use super::Renderer;

/// options for image renderer
#[derive(Serialize, Deserialize)]
pub struct ImageRendererOptions {
    /// file to write the image to
    pub output: PathBuf,

    /// width of the image
    #[serde(default = "default_width")]
    pub width: u32,

    /// height of the image
    #[serde(default = "default_height")]
    pub height: u32,
}

fn default_width() -> u32 { 640 }
fn default_height() -> u32 { 480 }

impl Default for ImageRendererOptions {
    fn default() -> Self {
        Self {
            output: PathBuf::from(""),
            width: default_width(),
            height: default_height(),
        }
    }
}

/// renderer that renders to an image file
pub struct ImageRenderer {
    pub options: ImageRendererOptions,
}

impl ImageRenderer {
    /// create a new ImageRenderer with the given options
    pub fn new(options: &str) -> Self {
        Self {
            options: match serde_yaml::from_str(options) {
                Ok(options) => options,
                Err(err) => {
                    eprintln!("failed to parse renderer options: {err}");
                    std::process::exit(1);
                }
            },
        }
    }
}

impl Renderer for ImageRenderer {
    fn render(&mut self, bitmap: &Bitmap) {
        // convert internal bitmap format to ImageBuffer
        let img = ImageBuffer::from_fn(self.options.width, self.options.height, |x, y| {
            let pixel = bitmap.get(x as usize, y as usize).unwrap_or(Color::new(0, 0, 0));
            image::Rgb([pixel.red, pixel.green, pixel.blue])
        });

        // save image
        img.save(&self.options.output).unwrap();
    }

    fn get_size(&self) -> (usize, usize) {
        (self.options.width as usize, self.options.height as usize)
    }
}
