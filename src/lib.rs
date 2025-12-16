//! # simple_input
//! 
//! `simple_input` is an attempt at simplifying some io utilities provided by std library. 
//! 
//! ## Example
//! 
//!  ```
//! use simple_input::Input;
//! 
//! fn main() {
//!     let promptless_input: String = Input::input();
//!     let prompted_input: String = Input::prompted_input("Sample Prompt");
//! }
//! ```

use std::io::{self, Write};
//// Structure to call the different input related functionality
pub struct Input {}

impl Input {

///Creates a String as a buffer, creates a handle to stdin and writes a single line of input to the buffer
///Will not fail gracefully yet

    pub fn input() -> String {
        let mut buffer: String = String::new();

        io::stdin()
            .read_line(&mut buffer)
            .expect("Could not read stdin");

        buffer
    }
///Takes a string literal as an argument to print a prompt before input. Relies on input()
/// # Example
/// 
/// ```
/// let example: String = prompted_input("Prompt here");
/// ```
    pub fn prompted_input(prompt: &str) -> String {
        print!("{}: ", prompt);
        io::stdout()
            .flush()
            .expect("Could not flush");

        Self::input()
    }
}



//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn it_works() {
//        let result = add(2, 2);
//        assert_eq!(result, 4);
//    }
//}
