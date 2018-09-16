pub mod debug;
mod op;

pub use self::op::Op;

#[derive(PartialEq, Debug)]
pub enum Abort<'s> {
    Completed,
    InstrCapped,
    AwaitingInput,
    Error(::io::Issue<'s>),
}

#[cfg(test)]
mod tests;
