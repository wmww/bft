use runtime;
use source::Spanned;

#[derive(PartialEq)]
pub enum Section {
    Bf(Vec<Spanned<runtime::Op>>),
    Comment(Spanned<String>),
}

impl runtime::CodeSource for Section {
    fn append_code_to(&self, code: &mut Vec<Spanned<runtime::Op>>) {
        match self {
            Section::Bf(bf) => {
                code.extend(bf.clone());
            }
            Section::Comment(_) => (),
        }
    }
}

impl std::fmt::Debug for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Section::Bf(bf) => write!(f, "Bf {:#?}", bf),
            Section::Comment(comment) => write!(f, "{:?}", comment),
        }
    }
}
