#[macro_use]
mod io;
mod runtime;
mod source;

fn main() {
    let options = io::Options::new_default().with_cmd_line();
    if let Some(ref path) = options.filepath {
        let source = match source::File::open(path, &options) {
            Ok(s) => std::rc::Rc::new(s),
            Err(i) => {
                io::message(io::Error, &i);
                ::std::process::exit(1);
            }
        };
        let tokens = source::lex(source);
        let mut runtime = runtime::debug::Runtime::<u8>::new();
        runtime.add_tokens(&tokens);
        runtime.run(None, &mut |c| print!("{}", c));
    }
}
