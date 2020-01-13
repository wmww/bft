#[macro_use]
extern crate combine;

#[macro_use]
mod io;
mod ast;
mod parse;
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
        match parse::parse(&source) {
            Ok(ast) => {
                println!("{:#?}\n", ast);
                let mut runtime = runtime::debug::Runtime::<u8>::new();
                runtime.add_code(&*ast);
                runtime.run(None, &mut |c| print!("{}", c));
            },
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
}
