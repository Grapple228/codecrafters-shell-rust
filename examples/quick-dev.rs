#![allow(unused)] // For beginning only.

use std::{
    fs,
    io::{self, stderr, Write},
    process,
};

use shell::{Result, Shell, Splitter};
use tracing::debug;

fn main() -> Result<()> {
    shell::init()?;

    let command = fs::read_to_string("command.txt")?;
    let command = command.trim();
    debug!("command: '{}'", command);

    let command2 = split_with_quotes(command);
    debug!("command2: '{:?}", command2);

    let mut shell = Shell::default()?;

    shell.init()?;

    match shell.process_command(&command) {
        Ok(_) => (),
        Err(error) => shell::report(error),
    }

    Ok(())
}

fn split_with_quotes(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_word = String::new();
    let mut chars = input.chars().peekable();
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;

    while let Some(c) = chars.next() {
        match c {
            // Handle backslashes
            '\\' => {
                if let Some(&next_char) = chars.peek() {
                    // In double quotes, only certain characters are escaped
                    if in_double_quotes {
                        match next_char {
                            '\\' | '$' | '"' | '\n' => {
                                chars.next(); // Consume the escaped character
                                current_word.push(next_char);
                            }
                            _ => {
                                current_word.push('\\');
                                current_word.push(next_char);
                                chars.next();
                            }
                        }
                    } else if in_single_quotes {
                        current_word.push('\\');
                        current_word.push(next_char);
                        chars.next();
                    } else {
                        // Outside quotes, preserve the literal value of the next character
                        chars.next(); // Consume the escaped character
                        current_word.push(next_char);
                    }
                }
            }
            // Handle quotes
            '\'' if !in_double_quotes => {
                in_single_quotes = !in_single_quotes;
            }
            '"' if !in_single_quotes => {
                in_double_quotes = !in_double_quotes;
            }
            // Handle spaces
            ' ' if !in_single_quotes && !in_double_quotes => {
                if !current_word.is_empty() {
                    result.push(current_word);
                    current_word = String::new();
                }
            }
            // Handle all other characters
            _ => {
                current_word.push(c);
            }
        }
    }

    // Add the last word if there is one
    if !current_word.is_empty() {
        result.push(current_word);
    }

    result
}
