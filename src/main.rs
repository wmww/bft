
#[macro_use]
mod log;
mod options;
mod source;

fn main() {
    let options = options::Options::new_default().with_cmd_line();
    if let Some(ref file) = options.filepath {
        let _source = match source::File::open(file, &options) {
            Ok(s) => s,
            Err(e) => bft_error!(options, "{}", e),
        };
    }
}
