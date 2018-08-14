
#[macro_use]
mod log;
mod options;

use std::io::Read;

fn main() {
    let options = options::Options::new_default().with_cmd_line();
    if let Some(file) = options.filepath {
        bft_log!(options, "Loading {}", file);
        let mut f = match std::fs::File::open(file.clone()) {
            Result::Ok(v) => v,
            Result::Err(e) => {
                bft_error!(options, "file {} not found: {}", file, e);
            }
        };
        bft_log!(options, "Reading {}", file);
        let mut contents = String::new();
        match f.read_to_string(&mut contents) {
            Result::Ok(_) => (),
            Result::Err(e) => {
                bft_error!(options, "error reading {}: {}", file, e);
            }
        }
    }
}
