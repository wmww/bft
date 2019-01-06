pub mod debug;
mod op;

pub use self::op::Op;

pub trait CodeSource {
    fn append_code_to(&self, code: &mut Vec<super::source::Spanned<Op>>);

    fn get_code(&self) -> Vec<super::source::Spanned<Op>> {
        let mut code = vec![];
        self.append_code_to(&mut code);
        code
    }
}

#[derive(PartialEq, Debug)]
pub enum Abort {
    Completed,
    InstrCapped,
    AwaitingInput,
    Error(::io::Issue),
}

#[cfg(test)]
mod tests;
