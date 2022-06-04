//! renderer trait and renderer implementations

pub mod ansi;

use crate::bitmap::Bitmap;
use enum_iterator::{all, Sequence};
use std::str::FromStr;
use ansi::{AnsiRenderer, AnsiRendererOptions};

/// describes how the rest of the program should interact with renderers
pub trait Renderer {
    /// render the specified bitmap
    fn render(&mut self, bitmap: &Bitmap);

    /// get size of renderer
    fn get_size(&self) -> (usize, usize);
}

/// list of all available renderers
#[derive(Debug, Sequence)]
pub enum Renderers {
    Ansi,
}

impl FromStr for Renderers {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_ref() {
            "ansi" => Ok(Self::Ansi),
            _ => Err(()),
        }
    }
}

/// get name of default renderer
pub fn default_renderer_name() -> String {
    "ansi".to_string()
}

/// list all available renderers
pub fn list_renderers() {
    println!("available renderers:");
    for renderer in all::<Renderers>() {
        println!("    - {:?}", renderer);
    }
}

/// create a new renderer given its name and options
pub fn create_renderer(name: Renderers, options: &str) -> Box<dyn Renderer> {
    Box::new(match name {
        Renderers::Ansi => AnsiRenderer::new(options),
    })
}

/// list options of renderer
pub fn list_options(name: Renderers) {
    let options = match name {
        Renderers::Ansi => {
            let options: AnsiRendererOptions = Default::default();
            serde_yaml::to_string(&options).unwrap()
        },
    };

    println!("default options for {:?}:", name);
    println!("{}", options);
}
