pub mod debug;
mod op;

pub use self::op::Op;

use log;

#[derive(PartialEq, Debug)]
pub enum Abort<'s> {
    Completed,
    ExceededLimit,
    AwaitingInput,
    Error(log::Issue<'s>),
}

#[cfg(test)]
mod tests;
