use log;
use options;

use std;
use std::fmt;
use std::io::Read;

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