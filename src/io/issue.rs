extern crate colored;

use self::colored::*;
use std::fmt;

use super::*;
use source;

/*
struct SpanDisplayData {
    filepath: String,
    line: u32,
    col: u32,
    len: u32,
    line_str: String,
    byte_start: usize,
    byte_end: usize,
}

impl SpanDisplayData {
    fn new(span: &source::Span) -> SpanDisplayData {
        let line_str = span.src.contents[span.start..span.end()].to_string();
        let byte_end = line_str.len();
        let len = line_str.chars().count() as u32;
        SpanDisplay {
            filepath: span.src.unwrap_path(),
            line: 0,
            col: 0,
            len,
            line_str,
            byte_start: 0,
            byte_end,
        }
    }
}

impl fmt::Display for SpanDisplayData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let location_info = format!("{}:{}:{} ", self.filepath, self.line, self.col);
        let indicator = " ".repeat(location_info.chars().count() + self.col as usize)
            + &"^".repeat(self.len as usize);
        let range = self.byte_start..self.byte_end;
        let mut line_str = self.line_str.clone();
        line_str.replace_range(range.clone(), &self.line_str[range].bright_red().bold());
        write!(f, "{}{}\n{}", location_info, line_str, indicator)
    }
}
*/

#[derive(PartialEq)]
pub struct Issue {
    pub severity: Severity,
    pub span: Option<source::Span>,
    pub message: String,
}

impl Issue {
    pub fn new(severity: Severity, message: &str) -> Issue {
        Issue {
            severity,
            span: None,
            message: message.to_string(),
        }
    }
}

impl fmt::Display for Issue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.span {
            Some(s) => write!(f, "{}: {}:\n    {}", self.severity, s, self.message),
            None => write!(f, "{}: {}", self.severity, &self.message),
        }
    }
}

impl fmt::Debug for Issue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}: {}{}",
            self.severity,
            self.message,
            match &self.span {
                Some(span) => format!(" at {:?}", span),
                None => "".to_string(),
            }
        )
    }
}
