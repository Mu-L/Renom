use colored::*;
use std::io::{stdout, Write};

pub struct Log;

impl Log {
    /// Toggles coloring based on environment.
    /// For instance, colors do not work for `cmd`on Windows.
    pub fn check_support_for_colors() {
        let term = term::stdout().unwrap();
        if !term.supports_color() {
            colored::control::set_override(false);
        }
    }

    /// Print a header. Includes a preliminary newline.
    pub fn header<S: AsRef<str>>(text: S) {
        println!("\n[ {} ]", text.as_ref().green());
    }

    /// Print a process.
    pub fn process<S: AsRef<str>>(text: S) {
        println!("{}{}", "===".magenta(), text.as_ref());
    }

    /// Print the text without any frills.
    pub fn basic<S: AsRef<str>>(text: S) {
        println!("{}", text.as_ref());
    }

    /// Print a prompt without a new line.
    pub fn prompt<S: AsRef<str>>(text: S) {
        print!("{}: ", text.as_ref().blue());
        stdout().flush().unwrap();
    }

    /// Print a step in a process.
    pub fn step<S: AsRef<str>>(process: S, step: S) {
        println!("{}: {}", process.as_ref().magenta(), step.as_ref())
    }

    /// Print a newline.
    pub fn newline() {
        println!("");
    }

    /// Print an error.
    pub fn error<S: AsRef<str>>(text: S) {
        println!("\n\t[ Error ]\n\t{}\n", text.as_ref().red());
    }
}
