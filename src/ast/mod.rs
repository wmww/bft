mod node;

use runtime;
use source;
use source::Spanned;

pub use self::node::Root;

impl<T: runtime::CodeSource> runtime::CodeSource for Vec<T> {
    fn append_code_to(&self, code: &mut Vec<Spanned<runtime::Op>>) {
        for elem in self {
            elem.append_code_to(code);
        }
    }
}

pub fn parse(file: std::rc::Rc<source::File>) -> Root {
    let mut p = ::source::Parser::new(file);
    match p.parse(()) {
        Ok(v) => v,
        Err(issue) => {
            match issue {
                Some(issue) => println!("Issue: {}", issue),
                None => println!("Parse failed"),
            }
            Root::empty()
        }
    }
}

#[cfg(test)]
mod tests;
