use std::fmt;

use io;

#[derive(Clone, PartialEq)]
pub struct Span {
    pub file: std::rc::Rc<::source::File>,
    pub start_byte: usize,
    pub end_byte: usize,
}

impl Span {
    pub fn between(a: &Span, b: &Span) -> Span {
        assert_eq!(a.file, b.file);
        Span {
            file: a.file.clone(),
            start_byte: std::cmp::min(a.start_byte, b.start_byte),
            end_byte: std::cmp::max(a.end_byte, b.end_byte),
        }
    }

    pub fn issue(&self, severity: io::Severity, message: &str) -> io::Issue {
        io::Issue {
            span: Some(self.clone()),
            severity,
            message: message.to_string(),
        }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "…{}…",
            &self.file.contents[self.start_byte..self.end_byte]
        )
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}[{}…{}]: {}",
            self.file.unwrap_path(),
            self.start_byte,
            self.end_byte,
            self
        )
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Spanned<T> {
    pub s: Option<Span>,
    pub v: T, // Value
}
