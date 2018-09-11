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
        let code = match bf::parse(&tokens) {
            Ok(c) => c,
            Err(e) => bft_error!(options, "{:?}", e.1),
        };
        let mut runtime = bf::naive::Runtime::<u8>::new();
        runtime.add_code(&code);
        runtime.run(None, &| c | print!("{}", c));
    }
}
