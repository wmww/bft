#[macro_use]
mod io;
mod ast;
mod runtime;
mod source;

fn main() {
    let options = io::Options::new_default().with_cmd_line();
    if let Some(ref path) = options.filepath {
        let source = match source::File::open(path) {
            Ok(s) => std::rc::Rc::new(s),
            Err(e) => {
                println!("{}", e);
                ::std::process::exit(1);
            }
        };
        println!("Source code: {}", source);
        let ast = ast::parse(source);
        println!("{:#?}\n", ast);
        let mut runtime = runtime::debug::Runtime::<u8>::new();
        runtime.add_code(&*ast);
        runtime.run(None, &mut |c| print!("{}", c));
    }
}
