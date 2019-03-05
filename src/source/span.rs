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

    pub fn around<T>(self, v: T) -> Spanned<T> {
        Spanned { s: Some(self), v }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let len = self.file.contents.len();
        if self.start_byte > len || self.end_byte > len {
            write!(
                f,
                "Invalid span [{}—{}] (file length is {})",
                self.start_byte, self.end_byte, len,
            )
        } else {
            write!(
                f,
                "…{}…",
                &self.file.contents[self.start_byte..self.end_byte],
            )
        }
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}[{}—{}]: {}",
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

#[cfg(test)]
pub struct TestBuilder {
    pub file: ::std::rc::Rc<::source::File>,
    pub span_byte: usize,
}

#[cfg(test)]
impl TestBuilder {
    pub fn new(code: &'static str) -> TestBuilder {
        TestBuilder {
            file: ::std::rc::Rc::new(::source::File::from_string(code.to_string())),
            span_byte: 0,
        }
    }

    pub fn span(&mut self, len: usize) -> Span {
        let s = Span {
            file: self.file.clone(),
            start_byte: self.span_byte,
            end_byte: self.span_byte + len,
        };
        self.span_byte += len;
        s
    }

    pub fn _skip(&mut self, len: usize) {
        self.span_byte += len;
    }
}
