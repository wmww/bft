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

impl runtime::CodeSource for Spanned<runtime::Op> {
    fn append_code_to(&self, code: &mut Vec<Spanned<runtime::Op>>) {
        code.push(self.clone());
    }
}

pub fn parse(_file: std::rc::Rc<source::File>) -> Vec<Node> {
    vec![]
}
