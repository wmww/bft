use io;

use std;
use std::fmt;
use std::io::Read;

#[derive(Debug, PartialEq)]
pub struct File {
    pub path: Option<String>,
    pub contents: String,
}

impl File {
    pub fn open(path: &str, options: &io::Options) -> Result<File, String> {
        options.debug(&format!("Reading {}", path));
        let mut f = match std::fs::File::open(path.clone()) {
            Result::Ok(v) => v,
            Result::Err(e) => return Err(format!("'{}': {}", path, e.to_string())),
        };
        let mut contents = String::new();
        match f.read_to_string(&mut contents) {
            Result::Ok(_) => (),
            Result::Err(e) => return Err(format!("'{}': {}", path, e.to_string())),
        }
        Ok(File {
            path: Some(path.to_string()),
            contents: contents,
        })
    }

    pub fn from_string(contents: String) -> File {
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
