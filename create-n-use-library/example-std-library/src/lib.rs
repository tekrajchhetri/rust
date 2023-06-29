//! This is a library that provides utility for command line tools.
//! # Examples:
//! ```
//! use examplestdlibrary::read_stdin;
//! use examplestdlibrary::colors::*;
//! let input = read_stdin();
//! println!("{}", red("This text will be printed in red"));
//! println!("{}", green("This text will be printed in green"));
//! ```
use std::io::{BufRead, BufReader};
// make colors module publicly available under examplestdlibrary
pub mod colors;
// make config module publicly available under examplestdlibrary
pub mod config;
/// This function reads a line from stdin and returns the read line as String.
/// In the event of the failure, it will panic with a message "Fail to read input line".
/// # Examples:
/// ```
/// use examplestdlibrary::read_stdin;
/// let input = read_stdin();
/// ```
pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut bufreader = BufReader::new(stdin.lock());
    _read_stdin(&mut bufreader)
}

//private function
fn _read_stdin<R: BufRead>(bufreader: &mut R) -> String {
    let mut line = String::new();
    bufreader
        .read_line(&mut line)
        .expect("Failed to read input line");
    line.trim().to_string()
}

#[cfg(test)]
mod tests {
   use super::_read_stdin;
   // allows to creater bufreader 
   use std::io::Cursor;

   #[test]
   fn test_input_newlinetrim() {
    //new line which should be trimmed
    let inputcmdline = "Welcome Rust\n";
    let expected_output = "Welcome Rust";
    let mut reader = Cursor::new(inputcmdline);
    let output = _read_stdin(&mut reader);
    assert_eq!(output, expected_output, "The text should be Welcome Rust not Welcome Rust\n"");
}

#[test]
fn test_input_extraspacetrim() {
    //extra space which should be trimmed
    let inputcmdline = " Welcome Rust ";
    let expected_output = "Welcome Rust";
    let mut reader = Cursor::new(inputcmdline);
    let output = _read_stdin(&mut reader);
    assert_eq!(output, expected_output);
}

   #[test]
   fn test_empty() {
       let inputcmdline = "";
       let expected_output = "";
       let mut reader = Cursor::new(inputcmdline);
       let output = _read_stdin(&mut reader);
       assert_eq!(output, expected_output);
   }
}