pub trait StringExt {
    fn substring(&self, start: usize, end: usize) -> String;
    fn char_at(&self, index: usize) -> char;
    fn remove_double_spaces(&self) -> String;
}

impl StringExt for String {
    fn substring(&self, start: usize, end: usize) -> String {
        self.chars().skip(start).take(end - start).collect()
    }

    fn char_at(&self, index: usize) -> char {
        self.chars().nth(index).unwrap_or_default()
    }

    fn remove_double_spaces(&self) -> String {
        let mut result = String::new();

        let mut chars = self.trim().chars().peekable();

        loop {
            match chars.next() {
                Some(' ') => {
                    if chars.peek() != Some(&' ') {
                        result.push(' ');
                    }
                }
                Some(other) => {
                    result.push(other);
                }
                None => break,
            }
        }

        result
    }
}

pub trait CharExt {
    fn is_alpha(&self) -> bool;
    fn is_alpha_numeric(&self) -> bool;
}

impl CharExt for char {
    fn is_alpha_numeric(&self) -> bool {
        self.is_ascii_digit() || self.is_alpha()
    }
    fn is_alpha(&self) -> bool {
        self.is_ascii_alphabetic() || *self == '_'
    }
}
