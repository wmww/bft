pub mod debug;
mod op;

pub use self::op::Op;

use log;

pub enum Abort<'s> {
    Completed,
    ExceededLimit,
    AwaitingInput,
    Error(log::Issue<'s>),
}
