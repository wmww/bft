use colored::*;

use source;
use std::fmt;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Severity {
    Debug,
    //InternalWarning,
    InternalError,
    //Warning,
    Error,
}

pub fn message(severity: Severity, msg: &str) {
    println!(
        "{}: {}",
        match severity {
            Severity::Debug => "Debug",
            //Severity::InternalWarning => "Internal warning",
            Severity::InternalError => "Internal error",
            //Severity::Warning => "Warning",
            Severity::Error => "Error",
        },
        msg
    );
}

struct SpanDisplay {
    filepath: String,
    line: u32,
    col: u32,
    len: u32,
    line_str: String,
    byte_start: usize,
    byte_end: usize,
}

impl SpanDisplay {
    fn new(span: &source::Span) -> SpanDisplay {
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

impl<'s> fmt::Display for SpanDisplay {
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

#[derive(PartialEq)]
pub struct Issue<'s> {
    pub severity: Severity,
    pub span: Option<source::Span<'s>>,
    pub message: String,
}

impl<'s> Issue<'s> {
    pub fn show(&self) {
        match &self.span {
            Some(s) => message(
                self.severity,
                &format!("{}:\n    {}", SpanDisplay::new(&s), self.message),
            ),
            None => message(self.severity, &self.message),
        }
    }
}

impl<'s> fmt::Debug for Issue<'s> {
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

#[macro_export]
macro_rules! bft_error {
    ($ops:ident, $($arg:tt)*) => ({
        log::message(log::Severity::InternalError, &format!($($arg)*));
        std::process::exit(1);
    })
}

#[macro_export]
macro_rules! bft_log {
    ($ops:ident, $($arg:tt)*) => ({
        if $ops.debug {
            log::message(log::Severity::Debug, &format!($($arg)*));
        }
    })
}
