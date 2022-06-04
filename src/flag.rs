//! flag structure and rendering

use serde::Deserialize;
use crate::bitmap::Bitmap;
use crate::util::PartialSize;
use crate::render::Renderer;

/// our flag struct
#[derive(Debug, Deserialize)]
pub struct Flag {
    /// aspect ratio of flag
    pub aspect: PartialSize,

    /// vec of horizontal sections
    pub sections: Vec<FlagSection>,
}

/// horizontal section of flag
#[derive(Debug, Deserialize)]
pub struct FlagSection {
    /// width of flag section
    pub width: PartialSize,

    /// vec of vertical sub-sections
    pub subsections: Vec<FlagSubSection>,
}

/// vertical section of flag
#[derive(Debug, Deserialize)]
pub struct FlagSubSection {
    /// width relative to parent section
    pub width: PartialSize,

    /// height relative to flag height
    pub height: PartialSize,

    /// color of section
    pub color: [u8; 3],
}

/// render the given flag with the given renderer
pub fn render_flag(renderer: &mut impl Renderer, flag: &Flag) {
    // get size we can render to
    let (width, height) = renderer.get_size();
    let aspect = width as f64 / height as f64;

    // size of flag in bitmap
    let flag_width;
    let flag_height;
    let flag_aspect = flag.aspect.as_number().expect("invalid flag aspect ratio");

    // where the flag should be positioned on the bitmap
    let flag_x;
    let flag_y;

    // calculate flag size and position
    if aspect > flag_aspect {
        flag_width = (flag_aspect * height as f64).round() as usize;
        flag_height = height;
        flag_x = (width as f64 / 2.0 - flag_width as f64 / 2.0).round() as usize;
        flag_y = 0;
    } else if aspect < flag_aspect {
        flag_width = width;
        flag_height = (1.0 / flag_aspect * width as f64).round() as usize;
        flag_x = 0;
        flag_y = (height as f64 / 2.0 - flag_height as f64 / 2.0).round() as usize;
    } else {
        flag_width = width;
        flag_height = height;
        flag_x = 0;
        flag_y = 0;
    }

    // create a new bitmap
    let mut bitmap = Bitmap::new(width, height);

    // x position of current section- used to keep track of where we are and make sure we don't exceed the valid flag width
    let mut section_x = 0;

    // iterate over all flag sections (horizontal)
    for section in flag.sections.iter() {
        // raw float value of section width- used to calculate subsection relative width
        let section_width_raw = section.width.as_number().expect("invalid section width");

        // calculate width of section
        let mut section_width = (section_width_raw * flag_width as f64).round() as usize;

        // clamp section width to edge of flag
        if section_x + section_width > flag_width {
            section_width = flag_width - section_x;
        }

        // y position of current subsection, serves same purpose as section_x
        let mut section_y = 0;

        // iterate over all flag subsections (vertical)
        for sub in section.subsections.iter() {
            // calculate height of subsection
            let mut sub_height = (sub.height.as_number().expect("invalid subsection height") * flag_height as f64).round() as usize;

            // clamp section height to edge of flag
            if section_y + sub_height > flag_height {
                sub_height = flag_height - section_y;
            }

            // calculate width of subsection
            let mut sub_width = (sub.width.as_number().expect("invalid subsection width") * section_width_raw * flag_width as f64).round() as usize;

            // clamp section height to edge of flag
            if section_x + sub_width > flag_width {
                sub_width = flag_width - section_x;
            }

            // render part of flag
            bitmap.draw_rect(flag_x + section_x, flag_y + section_y, sub_width, sub_height, sub.color.into());

            // increment y position
            section_y += sub_height;
        }

        // increment x position
        section_x += section_width;
    }

    // render bitmap to screen
    renderer.render(&bitmap);
}
