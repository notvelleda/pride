//! renderer that renders directly to the terminal with ANSI escape sequences

use crate::bitmap::Bitmap;
use std::{
    cmp::min,
    io::{Write, stdin, stdout},
};
use serde::{Serialize, Deserialize};
use super::Renderer;
use termion::{
    color, cursor, clear,
    event::Event,
    input::TermRead,
    raw::IntoRawMode,
    screen::AlternateScreen,
};

/// options for ANSI renderer
#[derive(Serialize, Deserialize)]
pub struct AnsiRendererOptions {
    #[serde(default)]
    pub true_color: bool,
}

impl Default for AnsiRendererOptions {
    fn default() -> Self {
        Self {
            true_color: false,
        }
    }
}

/// renderer that renders directly to the terminal
pub struct AnsiRenderer {
    pub options: AnsiRendererOptions,
}

impl AnsiRenderer {
    pub fn new(options: &str) -> Self {
        Self {
            options: serde_yaml::from_str(options).unwrap(),
        }
    }
}

impl Renderer for AnsiRenderer {
    /// draws a bitmap to the terminal with ANSI escape codes
    fn render(&mut self, bitmap: &Bitmap) {
        assert!(bitmap.height % 2 == 0, "bitmap height is not an even number");

        let (term_width, term_height) = self.get_size();

        let mut sequence = String::new();

        // clear screen
        sequence.push_str(cursor::Hide.as_ref());
        sequence.push_str(clear::All.as_ref());

        // last color values- used to speed up drawing since we can skip escape sequences for duplicates
        let mut last_upper_color: Option<String> = None;
        let mut last_lower_color: Option<String> = None;

        // convert bitmap to text characters and ANSI escape codes
        for y in (0..min(bitmap.height, term_height)).step_by(2) {
            // move cursor to start of line
            sequence.push_str(&cursor::Goto(1, (y / 2 + 1).try_into().unwrap()).to_string());

            for x in 0..min(bitmap.width, term_width) {
                // we're dividing each character cell vertically into two colors
                let upper_color = bitmap.get(x, y).unwrap();
                let lower_color = bitmap.get(x, y + 1).unwrap();

                // escape sequence strings for upper and lower colors of character cell (foreground and background with a special character)
                let upper_color_str;
                let lower_color_str;

                // convert colors into ANSI escape sequences
                if self.options.true_color {
                    upper_color_str = color::Rgb(upper_color.red, upper_color.green, upper_color.blue).fg_string();
                    lower_color_str = color::Rgb(lower_color.red, lower_color.green, lower_color.blue).bg_string();
                } else {
                    // alias to 216 colors
                    upper_color_str = color::AnsiValue::rgb(((upper_color.red as f64 / 256.0) * 5.0) as u8, ((upper_color.green as f64 / 256.0) * 5.0) as u8, ((upper_color.blue as f64 / 256.0) * 5.0) as u8).fg_string();
                    lower_color_str = color::AnsiValue::rgb(((lower_color.red as f64 / 256.0) * 5.0) as u8, ((lower_color.green as f64 / 256.0) * 5.0) as u8, ((lower_color.blue as f64 / 256.0) * 5.0) as u8).bg_string();
                }

                // add colors to sequence if they've changed at all
                if if let Some(last) = &last_upper_color { last != &upper_color_str } else { true } {
                    sequence.push_str(&upper_color_str);
                }

                if if let Some(last) = &last_lower_color { last != &lower_color_str } else { true } {
                    sequence.push_str(&lower_color_str);
                }

                // set last colors to current colors
                last_upper_color = Some(upper_color_str);
                last_lower_color = Some(lower_color_str);

                // lastly, write the character for the cell
                sequence.push('\u{2580}');
            }
        }

        let stdin = stdin();
        // create an alternate terminal buffer to write to so we can have a cleaner switch back
        let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap()); // stdout needs to be set to raw mode to read individual characters

        // put everything on screen
        if let Err(err) = write!(stdout, "{}", sequence) {
            reset_terminal();
            eprintln!("{}error writing to stdout: {}", color::Fg(color::Red), err);
            return;
        }

        if let Err(err) = stdout.flush() {
            reset_terminal();
            eprintln!("{}error flushing stdout: {}", color::Fg(color::Red), err);
            return;
        }

        // wait for a key to be pressed before exiting
        for evt in stdin.events() {
            match evt {
                Ok(evt) => if let Event::Key(_) = evt { break; }, // break out of loop if we get a key event
                Err(err) => {
                    reset_terminal();
                    eprintln!("{}error reading stdin: {}", color::Fg(color::Red), err);
                    return;
                },
            }
        }

        reset_terminal(); // terminal is reset just in case we don't support alternate buffers
    }

    /// gets max size of renderer
    fn get_size(&self) -> (usize, usize) {
        let (width, height) = termion::terminal_size().unwrap();
        
        (width as usize, height as usize * 2)
    }
}

/// revert any changes we've made while rendering
fn reset_terminal() {
    let mut stdout = stdout();
    let sequence = format!("{}{}{}", color::Fg(color::Reset), color::Bg(color::Reset), cursor::Show);

    if write!(stdout, "{}", sequence).is_err() { // attempt to recover if an error occurs
        print!("{}", sequence);
    }

    if stdout.flush().is_err() {
        print!("{}", sequence);
    }
}
