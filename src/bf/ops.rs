use source;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Plus,
    Minus,
    Left,
    Right,
    Output,
    Input,
}

impl Op {
    pub fn new(c: char) -> Option<Op> {
        match c {
            '+' => Some(Op::Plus),
            '-' => Some(Op::Minus),
            '<' => Some(Op::Left),
            '>' => Some(Op::Right),
            '.' => Some(Op::Output),
            ',' => Some(Op::Input),
            _ => None,
        }
    }

    pub fn token<'s>(self, span: source::Span<'s>) -> source::Token<'s> {
        source::Token::Bf { span, op: self }
    }
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
        }
    }
}
