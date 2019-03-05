use super::Op;
use source::Spanned;

pub trait CodeSource {
    fn append_code_to(&self, code: &mut Vec<Spanned<Op>>);

    fn get_code(&self) -> Vec<Spanned<Op>> {
        let mut code = vec![];
        self.append_code_to(&mut code);
        code
    }
}

impl<T: CodeSource> CodeSource for Vec<T> {
    fn append_code_to(&self, code: &mut Vec<Spanned<Op>>) {
        for elem in self {
            elem.append_code_to(code);
        }
    }
}

impl CodeSource for Vec<Spanned<Op>> {
    fn append_code_to(&self, code: &mut Vec<Spanned<Op>>) {
        code.extend(self.clone());
    }
}

impl CodeSource for Vec<Op> {
    fn append_code_to(&self, code: &mut Vec<Spanned<Op>>) {
        code.extend(self.iter().map(|v| Spanned::new(*v)));
    }
}
