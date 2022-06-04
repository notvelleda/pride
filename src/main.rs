pub mod render;
pub mod bitmap;
pub mod util;
pub mod flag;

use std::{
    fs,
    path::PathBuf,
};
use crate::render::ansi::AnsiRenderer;
use crate::flag::{Flag, render_flag};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// path to the flag to view
    #[clap(short, long)]
    flag: PathBuf,
}

fn main() {
    let args = Args::parse();

    // read flag from file
    let flag = fs::read_to_string(args.flag).unwrap();

    // convert yaml to our flag struct
    let flag: Flag = serde_yaml::from_str(&flag).unwrap();

    let mut renderer = AnsiRenderer {};

    render_flag(&mut renderer, &flag);
}
