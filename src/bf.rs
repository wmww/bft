use std::fmt;

#[derive(Debug)]
pub enum Op {
    Plus,
    Minus,
    Left,
    Right,
    Output,
    Input,
    Open,
    Close,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Op::Plus => write!(f, "+"),
            Op::Minus => write!(f, "-"),
            Op::Left => write!(f, "<"),
            Op::Right => write!(f, ">"),
            Op::Output => write!(f, "."),
            Op::Input => write!(f, ","),
            Op::Open => write!(f, "["),
            Op::Close => write!(f, "]"),
        }
    }
}
