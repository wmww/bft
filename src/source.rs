use log;
use options;

use std;
use std::fmt;
use std::io::Read;
use std::str::CharIndices;

#[derive(Debug, PartialEq)]
pub struct File {
    pub path: Option<String>,
    pub contents: String,
}

impl File {
    pub fn open(path: &str, options: &options::Options) -> Result<File, String> {
        bft_log!(options, "Loading {}", path);
        let mut f = match std::fs::File::open(path.clone()) {
            Result::Ok(v) => v,
            Result::Err(e) => return Err(e.to_string()),
        };
        bft_log!(options, "Reading {}", path);
        let mut contents = String::new();
        match f.read_to_string(&mut contents) {
            Result::Ok(_) => (),
            Result::Err(e) => return Err(e.to_string()),
        }
        Ok(File {
            path: Some(path.to_string()),
            contents: contents,
        })
    }

    pub fn new(contents: String) -> File {
        File {
            path: None,
            contents: contents,
        }
    }

    pub fn unwrap_path(&self) -> String {
        self.path.clone().unwrap_or("[UNKNOWN]".to_string())
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:\n{}", self.unwrap_path(), self.contents)
    }
}

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
}

impl<'s> fmt::Display for Span<'s> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.src.unwrap_path())
    }
}
