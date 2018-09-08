#[macro_use]
mod log;
mod bf;
mod options;
mod source;

fn main() {
    let options = options::Options::new_default().with_cmd_line();
    if let Some(ref path) = options.filepath {
        let source = match source::File::open(path, &options) {
            Ok(s) => s,
            Err(e) => bft_error!(options, "{}", e),
        };
        println!("source: {}", source);
        println!();
        let tokens = source::lex(&source);
        println!("tokens: {}", tokens);
    }
}
