pub mod debug;
mod op;

pub use self::op::Op;

pub trait CodeSource {
    fn get_code(&self) -> Vec<super::source::Spanned<Op>>;
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
