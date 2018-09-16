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
        let tokens = source::lex(&source);
        let mut runtime = bf::debug::Runtime::<u8>::new();
        runtime.add_tokens(&tokens);
        runtime.run(None, &mut |c| print!("Output: {} (ASC: {})\n", c, c as i32));
    }
}
