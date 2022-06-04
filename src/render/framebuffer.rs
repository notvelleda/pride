//! renderer that writes to a linux framebuffer

use crate::bitmap::{Bitmap, Color};
use framebuffer::{Framebuffer, KdMode};
use serde::{Serialize, Deserialize};
use std::{
    io::{Write, stdin, stdout},
    path::PathBuf,
};
use super::Renderer;
use termion::{
    color,
    event::Event,
    input::TermRead,
    raw::IntoRawMode,
};

/// options for image renderer
#[derive(Serialize, Deserialize)]
pub struct FramebufferRendererOptions {
    #[serde(default = "default_device")]
    pub device: PathBuf,
}

fn default_device() -> PathBuf { PathBuf::from("/dev/fb0") }

impl Default for FramebufferRendererOptions {
    fn default() -> Self {
        Self {
            device: default_device(),
        }
    }
}

/// renderer that renders to an image file
pub struct FramebufferRenderer {
    pub options: FramebufferRendererOptions,
}

impl FramebufferRenderer {
    /// create a new FramebufferRenderer with the given options
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

impl Renderer for FramebufferRenderer {
    fn render(&mut self, bitmap: &Bitmap) {
        // open framebuffer
        let mut framebuffer = Framebuffer::new(&self.options.device).unwrap();

        // get framebuffer info
        let height = framebuffer.var_screen_info.yres;
        let line_length = framebuffer.fix_screen_info.line_length;
        let bytes_per_pixel = framebuffer.var_screen_info.bits_per_pixel / 8;

        // create temporary back buffer to write to
        let mut frame = vec![0u8; (line_length * height) as usize];

        // copy image to temporaray buffer
        for (y, line) in frame.chunks_mut(line_length as usize).enumerate() {
            for (x, p) in line.chunks_mut(bytes_per_pixel as usize).enumerate() {
                let pixel = bitmap.get(x as usize, y as usize).unwrap_or(Color::new(0, 0, 0));

                p[0] = pixel.blue;
                p[1] = pixel.green;
                p[2] = pixel.red;
            }
        }

        // set tty to graphics mode
        let _ = Framebuffer::set_kd_mode(KdMode::Graphics).unwrap();

        // draw image
        let _ = framebuffer.write_frame(&frame);

        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap(); // stdout needs to be set to raw mode to read individual characters
		
        // we need to flush stdout for this to work
        stdout.flush().unwrap();

        // wait for a key to be pressed before exiting
        for evt in stdin.events() {
            match evt {
                Ok(evt) => if let Event::Key(_) = evt { break; }, // break out of loop if we get a key event
                Err(err) => {
                    let _ = Framebuffer::set_kd_mode(KdMode::Text).unwrap();
                    eprintln!("{}error reading stdin: {}", color::Fg(color::Red), err);
                    return;
                },
            }
        }

        // switch back to text mode
        let _ = Framebuffer::set_kd_mode(KdMode::Text).unwrap();
    }

    fn get_size(&self) -> (usize, usize) {
        let framebuffer = Framebuffer::new(&self.options.device).unwrap();

        (framebuffer.var_screen_info.xres as usize, framebuffer.var_screen_info.yres as usize)
    }
}
