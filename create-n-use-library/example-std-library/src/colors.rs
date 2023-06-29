//! Provides the colorized output for the terminal using ANSI escape codes.
//! # Examples:
//! ```
//! use examplestdlibrary::colors::*;
//! println!("{}", red("This text will be printed in red"));
//! println!("{}", green("This text will be printed in green"));
//! ```

/// Returns a string with the ANSI escape code for red.
/// # Examples:
/// ```
/// use examplestdlibrary::colors::*;
/// println!("{}", red("I am red color")); 
pub fn red(inputstring: &str) -> String{
    format!("\x1b[31m{}\x1b[0m", inputstring)
}
/// Returns a string with the ANSI escape code for green.
/// # Examples:
/// ```
/// use examplestdlibrary::colors::*;
/// println!("{}", green("I am green color")); 
pub fn green(inputstring: &str) -> String{
    format!("\x1b[32m{}\x1b[0m", inputstring)
}
/// Returns a string with the ANSI escape code for bold text.
/// # Examples:
/// ```
/// use examplestdlibrary::colors::*;
/// println!("{}", bold("I am bold text")); 
pub fn bold(inputstring: &str) -> String{
    format!("\x1b[1m{}\x1b[0m", inputstring)
}

/// Returns a string with the ANSI escape code to reset the formatting.
/// # Examples:
/// ```
/// use examplestdlibrary::colors::*;
/// println!("{}", reset("reset the formatting")); 
pub fn reset(inputstring: &str) -> String{
    format!("\x1b[0m{}\x1b[0m", inputstring)
}

pub enum Color{
    Red,
    Green,
    Bold,
}

pub struct ColorString {
    pub color: Color,
    pub string: String,
    pub colored: String
}

impl ColorString {
    pub fn paint(&mut self) {
        match self.color {
            Color::Red => self.colored = red(&self.string),
            Color::Green => self.colored = green(&self.string),
            Color::Bold => self.colored = bold(&self.string),
        };
    }

    pub fn reset(&mut self) {
        self.colored = reset(&self.string);
    }

}