mod elem;
mod op;

use runtime;
use source::File;
use source::Span;
use source::Spanned;
use std::rc::Rc;

use self::elem::Elem;
use self::op::Op;
use self::op::Loop;

type Seq<T> = Vec<T>;

impl<T: runtime::CodeSource> runtime::CodeSource for Seq<T> {
    fn append_code_to(&self, code: &mut Vec<Spanned<runtime::Op>>) {
        for elem in self {
            elem.append_code_to(code);
        }
    }
}

pub type Root = Seq<Elem>;

pub fn parse(file: Rc<File>) -> Root {
    vec![]
}

