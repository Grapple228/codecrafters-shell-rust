use std::{fmt::format, str::Chars};

use tracing::{debug, info};
use tracing_subscriber::field::debug;

use crate::StringExt;

pub struct Splitter {
    source: String,
    current: usize,
    parts: Vec<String>,
    is_escaped: bool,
}

impl Splitter {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            current: 0,
            parts: Vec::new(),
            is_escaped: false,
        }
    }

    pub fn get_splitted(&mut self) -> Vec<String> {
        self.parts.clear();
        self.current = 0;

        let source = self.source.clone();
        let bytes = source.as_bytes();

        while self.current < source.len() {
            let c = bytes.get(self.current).map(|c| *c as char);

            if let Some(c) = c {
                match c {
                    ' ' => self.current += 1,
                    '\'' => self.single_quoted(),
                    '"' => self.double_quoted(),
                    _ => self.no_quoted(),
                };
            } else {
                break;
            }
        }

        self.parts.clone()
    }

    fn push(&mut self, part: String) {
        if !part.is_empty() {
            self.parts.push(part);
        }
    }

    fn no_quoted(&mut self) {
        debug!("-- no quoted");

        let searching = &self.source[self.current..];
        let index = searching
            .find(|c| c == '"' || c == '\'')
            .unwrap_or(self.source.len());

        let start = self.current;
        let end = self.current + start + index;

        let substring = self.source.substring(start, end);

        for part in Self::process_no_quoted_escapes(&substring) {
            self.push(part);
        }

        self.current = end;
    }

    fn single_quoted(&mut self) {
        debug!("-- single quoted");

        let searching = &self.source[self.current + 1..];

        let quote_index = searching.find('\'').unwrap_or(self.current + 1);

        let start = self.current;
        let end = start + quote_index + 1;

        let substring = self.source.substring(start + 1, end);

        debug!("substring: '{}'", substring);

        self.push(substring);

        self.current = end + 1;
    }
    fn double_quoted(&mut self) {
        debug!("-- double quoted");

        let searching = &self.source[self.current + 1..];

        let quote_index = searching.find('\"').unwrap_or(self.current + 1);

        let start = self.current;
        let end = start + quote_index + 1;

        let substring = self.source.substring(start + 1, end);

        debug!("substring: '{}'", substring);

        let substring = Self::process_quoted_escapes(&substring);

        self.push(substring);

        self.current = end + 1;
    }

    fn process_no_quoted_escapes(substring: &str) -> Vec<String> {
        let bytes = substring.as_bytes();

        let mut result = Vec::new();
        let mut current = String::new();

        let mut i = 0;

        loop {
            let c = bytes.get(i).map(|c| *c as char);

            if let Some(c) = c {
                match c {
                    '\\' => {
                        let next = bytes.get(i + 1).map(|c| *c as char);

                        if let Some(next) = next {
                            match next {
                                ' ' => {
                                    current.push(' ');
                                }
                                other => {
                                    current.push(other);
                                }
                            }
                            i += 1;
                        }
                    }
                    ' ' => {
                        result.push(current);
                        current = String::new();
                    }
                    other => {
                        current.push(other);
                    }
                }
            } else {
                result.push(current);
                break;
            }

            i += 1;
        }

        result
    }

    fn process_quoted_escapes(substring: &str) -> String {
        let bytes = substring.as_bytes();
        let mut result = String::new();

        let mut i = 0;

        while let Some(c) = bytes.get(i).map(|c| *c as char) {
            if c == '\\' {
                if let Some(next) = bytes.get(i + 1).map(|c| *c as char) {
                    match next {
                        '\\' | '\n' | '\t' => {
                            result.push(next);
                            i += 1;
                        }
                        _ => {
                            result.push('\\');
                            result.push(next);
                            i += 1;
                        }
                    }
                }
            } else {
                result.push(c);
            }

            i += 1;
        }

        result
    }
}
