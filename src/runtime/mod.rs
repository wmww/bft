mod code_source;
pub mod debug;
mod op;

pub use self::code_source::CodeSource;
pub use self::op::Op;

#[derive(PartialEq, Debug)]
pub enum Abort {
    Completed,
    InstrCapped,
    AwaitingInput,
    Error(::io::Issue),
}

#[cfg(test)]
mod tests;
