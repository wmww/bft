#[macro_use]
use log;
use options;

use std;
use std::io::Read;

pub struct File {
    pub path: String,
    pub contents: String
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
        Ok(File{path: path.to_string(), contents: contents})
    }
}