mod node;

use runtime;
use source;
use source::Spanned;

pub use self::node::Node;

impl<T: runtime::CodeSource> runtime::CodeSource for Vec<T> {
    fn append_code_to(&self, code: &mut Vec<Spanned<runtime::Op>>) {
        for elem in self {
            elem.append_code_to(code);
        }
    }
}

pub fn parse(file: std::rc::Rc<source::File>) -> Vec<Node> {
    let mut p = ::source::Parser::new(file);
    match p.parse(()) {
        Ok(v) => return v,
        Err(None) => println!("Parse failed"),
        Err(Some(issue)) => println!("Issue: {}", issue),
    }
    vec![]
}
