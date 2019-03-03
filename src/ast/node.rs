use runtime;
use source::Spanned;

pub enum Node {
    Bf(Vec<Spanned<runtime::Op>>),
    Comment(Spanned<String>),
}

impl ::runtime::CodeSource for Node {
    fn append_code_to(&self, code: &mut Vec<Spanned<runtime::Op>>) {
        match self {
            Node::Bf(bf) => {
                for i in bf {
                    code.push(i.clone());
                }
            }
            Node::Comment(_) => (),
        }
    }
}
