use std::{fmt::format, iter::Peekable, str::Chars};

use tracing::{debug, info};
use tracing_subscriber::field::debug;

use crate::StringExt;

#[derive(Debug, Default)]
pub struct Splitter {
    current: String,
    parts: Vec<String>,
}

impl Splitter {
    pub fn split(&mut self, source: impl Into<String>) -> Vec<String> {
        self.parts.clear();

        let source: String = source.into();
        let mut chars = source.chars().peekable();

        while let Some(c) = chars.peek() {
            match c {
                '\'' => self.single_quoted(&mut chars),
                '"' => self.double_quoted(&mut chars),
                _ => self.no_quoted(&mut chars),
            };
        }

        self.parts.clone()
    }

    fn push(&mut self, part: String) {
        if !part.is_empty() {
            self.parts.push(part);
        }
    }

    fn no_quoted(&mut self, chars: &mut Peekable<Chars>) {
        debug!("-- no quoted");

        let mut result = Vec::new();
        let mut current = String::new();
        let mut insert_to_prev = chars.peek() != Some(&' ');

        loop {
            if let Some(&c) = chars.peek() {
                match c {
                    '"' | '\'' => {
                        if !current.is_empty() {
                            result.push(current);
                        }
                        break;
                    }
                    _ => {
                        if let Some(c) = chars.next() {
                            match c {
                                // handle backslashes
                                '\\' => {
                                    if let Some(&next) = chars.peek() {
                                        current.push(next);
                                        chars.next();
                                    }
                                }
                                ' ' => {
                                    if !current.is_empty() {
                                        result.push(current);
                                        current = String::new();
                                    }
                                }
                                other => {
                                    current.push(other);
                                }
                            }
                        }
                    }
                }
            } else {
                if !current.is_empty() {
                    result.push(current);
                }
                break;
            }
        }

        for mut part in result {
            if insert_to_prev {
                if let Some(mut prev) = self.parts.pop() {
                    prev.push_str(&part);
                    part = prev;
                }

                insert_to_prev = false;
            }

            self.push(part);
        }
    }

    fn single_quoted(&mut self, chars: &mut Peekable<Chars>) {
        debug!("-- single quoted");

        chars.next(); // consume quote

        let mut substring = String::new();

        while let Some(c) = chars.peek() {
            match c {
                '\'' => {
                    chars.next();
                    break;
                }
                _ => {
                    if let Some(ch) = chars.next() {
                        substring.push(ch);
                    }
                }
            }
        }

        debug!("substring: '{}'", substring);

        self.push(substring);
    }

    fn double_quoted(&mut self, chars: &mut Peekable<Chars>) {
        debug!("-- double quoted");

        chars.next(); // consume quote

        let mut substring = String::new();

        while let Some(c) = chars.next() {
            match c {
                '"' => {
                    break;
                }
                _ => {
                    // handle backslashes
                    if c == '\\' {
                        if let Some(&next) = chars.peek() {
                            match next {
                                '\\' | '"' | '\n' | '\t' => {
                                    substring.push(next);
                                }
                                _ => {
                                    substring.push('\\');
                                    substring.push(next);
                                }
                            }
                            chars.next();
                        }
                    } else {
                        substring.push(c);
                    }
                }
            }
        }

        debug!("substring: '{}'", substring);

        self.push(substring);
    }
}
