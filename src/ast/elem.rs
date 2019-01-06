use super::*;

pub enum Elem {
    Bf(Seq<Spanned<Op>>),
    Loop(Loop),
    Comment(Spanned<String>),
}

impl runtime::CodeSource for Elem {
    fn append_code_to(&self, code: &mut Vec<Spanned<runtime::Op>>) {
        match self {
            Elem::Bf(bf) => bf.append_code_to(code),
            Elem::Loop(l) => l.append_code_to(code),
            Elem::Comment(_) => (),
        }
    }
}

