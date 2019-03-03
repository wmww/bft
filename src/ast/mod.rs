mod node;

use runtime;
use source;
use source::Spanned;

pub use self::node::Node;

type ParseResult<T> = Result<T, Option<::io::Issue>>;

impl<T: runtime::CodeSource> runtime::CodeSource for Vec<T> {
    fn append_code_to(&self, code: &mut Vec<Spanned<runtime::Op>>) {
        for elem in self {
            elem.append_code_to(code);
        }
    }
}

pub fn parse(_file: std::rc::Rc<source::File>) -> Vec<Node> {
    vec![]
}
