use std::cmp;
use std::fmt;
use std::rc::Rc;
use std::str::CharIndices;

use io;
use source::File;

#[derive(Clone, PartialEq)]
pub struct Span {
    pub src: Rc<File>,
    pub start: usize,
    pub len: u32,
}

impl Span {
    pub fn from_components(src: Rc<File>, start: usize, len: u32) -> Span {
        Span { src, start, len }
    }

    pub fn from_indices(
        src: Rc<File>,
        mut first: CharIndices,
        mut after_last: CharIndices,
    ) -> Span {
        let start = match first.next() {
            Some((i, _)) => i,
            None => src.contents.len(),
        };
        let end_exclusive = match after_last.next() {
            Some((i, _)) => i,
            None => src.contents.len(),
        };
        assert!(start <= end_exclusive);
        let len = (end_exclusive - start) as u32;
        Span { src, start, len }
    }

    pub fn end(&self) -> usize {
        return self.start + self.len as usize;
    }

    pub fn between(a: &Span, b: &Span) -> Span {
        assert_eq!(a.src, b.src);
        let start = cmp::min(a.start, b.start);
        let len = (cmp::max(a.start + a.len as usize, b.start + b.len as usize) - start) as u32;
        Span {
            src: a.src.clone(),
            start,
            len,
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
        write!(f, "{}", self.src.unwrap_path())
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end())
    }
}

pub struct Generator {
    src: Rc<File>,
    current: usize,
}

impl Generator {
    pub fn new(src: Rc<File>) -> Generator {
        Generator { src, current: 0 }
    }

    pub fn skip(&mut self, bytes: i32) -> &mut Generator {
        self.current = (self.current as i32 + bytes) as usize;
        self
    }

    pub fn jump_to(&mut self, byte: usize) -> &mut Generator {
        self.current = byte;
        self
    }

    pub fn reset(&mut self) -> &mut Generator {
        self.jump_to(0)
    }

    pub fn span(&mut self, bytes: usize) -> Span {
        let start = self.current;
        self.skip(bytes as i32);
        Span::from_components(self.src.clone(), start, (self.current - start) as u32)
    }
}
