pub mod render;
pub mod bitmap;
pub mod util;
pub mod flag;

use crate::bitmap::Color;
use crate::flag::{Flag, render_flag};
use crate::render::{
    create_renderer,
    list_renderers,
    list_options,
    Renderers,
};
use clap::Parser;
use std::{
    fs,
    path::PathBuf,
    process::exit,
    str::FromStr,
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// path to the flag to view
    #[clap(short, long)]
    flag: PathBuf,

    /// which renderer to use (try "--renderer list" to list all available renderers)
    #[clap(short = 'r', long)]
    renderer: Option<String>,

    /// options to pass to the renderer (try "--renderer-options list" to list all available renderer options)
    #[clap(short = 'o', long)]
    renderer_options: Option<String>,

    /// background color in hex notation
    #[clap(short, long, default_value_t = Color::new(0, 0, 0))]
    background: Color,
}

fn main() {
    let args = Args::parse();

    // get renderer name from args- default to value set in renderer/mod.rs
    let renderer_name = args.renderer.unwrap_or_else(crate::render::default_renderer_name);

    // list available renderers if requested
    if renderer_name == *"list" {
        list_renderers();
        return;
    }

    // get valid renderer name from list
    let renderer_name = if let Ok(r) = Renderers::from_str(&renderer_name) { r } else {
        eprintln!("renderer {} doesn't exist! try \"--renderer list\" to list all available renderers", renderer_name);
        exit(1);
    };

    // list available options if requested
    if let Some(options) = &args.renderer_options {
        if options == "list" {
            list_options(renderer_name);
            return;
        }
    }

    // create a new renderer
    let mut renderer = create_renderer(renderer_name, &format!("{{{}}}", args.renderer_options.unwrap_or_else(|| "".to_string())));

    // read flag from file
    let flag = fs::read_to_string(args.flag).unwrap();

    // convert yaml to our flag struct
    let flag: Flag = match serde_yaml::from_str(&flag) {
        Ok(flag) => flag,
        Err(err) => {
            eprintln!("error parsing flag: {}", err);
            exit(1);
        },
    };

    render_flag(&mut renderer, &flag, args.background);
}
