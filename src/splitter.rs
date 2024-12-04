use std::str::Chars;

use tracing::{debug, info};
use tracing_subscriber::field::debug;

use crate::StringExt;

pub struct Splitter {
    source: String,
    current: usize,
    parts: Vec<String>,
}

impl Splitter {
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            current: 0,
            parts: Vec::new(),
        }
    }

    pub fn get_splitted(&mut self) -> Vec<String> {
        self.parts.clear();
        self.current = 0;

        let source = self.source.clone();
        let bytes = source.as_bytes();

        debug!("source: '{}'", source);

        while self.current < source.len() {
            let c = bytes.get(self.current).map(|c| *c as char);

            debug!("current: '{}'", self.current);
            debug!("c: '{:?}'", c);

            if let Some(c) = c {
                match c {
                    '\'' => self.single_quoted(),
                    '"' => self.double_quoted(),
                    ' ' => self.current += 1,
                    _ => self.other(),
                };
            } else {
                break;
            }
        }

        self.parts.clone()
    }

    fn push(&mut self, part: String) {
        self.parts.push(part);
    }

    fn other(&mut self) {
        debug!("-- other");

        let searching = &self.source[self.current..];
        let index = searching
            .find(|c| c == '"' || c == '\'')
            .unwrap_or(self.source.len());

        let start = self.current;
        let end = self.current + start + index;

        let substring = self.source.substring(start, end);

        self.current = end;

        for part in substring.split_whitespace() {
            self.push(part.to_string());
        }
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

        self.push(substring);

        self.current = end + 1;
    }
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    type Error = Box<dyn std::error::Error>;
    type Result<T> = core::result::Result<T, Error>; // For tests.

    use super::*;

    #[test]
    fn split_input_simple_ok() -> Result<()> {
        let mut splitter = Splitter::new("a   b    c");

        assert_eq!(splitter.get_splitted(), vec!["a", "b", "c"]);
        Ok(())
    }

    #[test]
    fn split_input_empty_ok() -> Result<()> {
        let mut splitter = Splitter::new("");

        assert!(splitter.get_splitted().is_empty());
        Ok(())
    }

    #[test]
    fn split_input_empty_single_quoted_ok() -> Result<()> {
        let mut splitter = Splitter::new("''");

        assert!(splitter.get_splitted().is_empty());
        Ok(())
    }

    #[test]
    fn split_input_single_quoted_ok() -> Result<()> {
        let mut splitter = Splitter::new("'a   b'    c");

        assert_eq!(splitter.get_splitted(), vec!["a   b", "c"]);
        Ok(())
    }

    #[test]
    fn split_input_empty_double_quoted_ok() -> Result<()> {
        let mut splitter = Splitter::new("\"\"");
        assert!(splitter.get_splitted().is_empty());
        Ok(())
    }

    #[test]
    fn split_input_double_quoted_ok() -> Result<()> {
        let mut splitter = Splitter::new("a \"b   c\"    d");

        assert_eq!(splitter.get_splitted(), vec!["a", "\"b c\"", "d"]);
        Ok(())
    }

    #[test]
    fn split_input_double_quoted_with_apostrophe_ok() -> Result<()> {
        let mut splitter = Splitter::new("\"shell's    b\"    c");

        assert_eq!(splitter.get_splitted(), vec!["\"shell's b\"", "c"]);
        Ok(())
    }
}

// endregion: --- Tests
