pub trait CodeSource {
    fn append_code_to(&self, code: &mut Vec<::source::Spanned<::runtime::Op>>);

    fn get_code(&self) -> Vec<::source::Spanned<::runtime::Op>> {
        let mut code = vec![];
        self.append_code_to(&mut code);
        code
    }
}

impl CodeSource for Vec<::source::Spanned<::runtime::Op>> {
    fn append_code_to(&self, code: &mut Vec<::source::Spanned<::runtime::Op>>) {
        code.extend(self.clone());
    }
}

impl CodeSource for Vec<::runtime::Op> {
    fn append_code_to(&self, code: &mut Vec<::source::Spanned<::runtime::Op>>) {
        code.extend(self.iter().map(|v| ::source::Spanned { s: None, v: *v }));
    }
}
