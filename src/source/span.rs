use std::cmp;
use std::fmt;
use std::str::CharIndices;

use log;
use source::File;

#[derive(Debug, Clone, PartialEq)]
pub struct Span<'s> {
    pub src: &'s File,
    pub start: usize,
    pub len: u32,
}

impl<'s> Span<'s> {
    pub fn from_components(src: &'s File, start: usize, len: u32) -> Span<'s> {
        Span { src, start, len }
    }

    pub fn from_indices(
        src: &'s File,
        mut first: CharIndices<'s>,
        mut after_last: CharIndices<'s>,
    ) -> Span<'s> {
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

    pub fn between(a: &'s Span, b: &'s Span) -> Span<'s> {
        assert_eq!(a.src, b.src);
        let start = cmp::min(a.start, b.start);
        let len = (cmp::max(a.start + a.len as usize, b.start + b.len as usize) - start) as u32;
        Span {
            src: a.src,
            start,
            len,
        }
    }

    pub fn issue(&self, severity: log::Severity, message: &str) -> log::Issue {
        log::Issue {
            span: Some(self.clone()),
            severity,
            message: message.to_string(),
        }
    }
}

impl<'s> fmt::Display for Span<'s> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.src.unwrap_path())
    }
}
