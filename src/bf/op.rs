use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Plus,
    Minus,
    Left,
    Right,
    Output,
    Input,
    Start,
    End,
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
            '[' => Some(Op::Start),
            ']' => Some(Op::End),
            _ => None,
        }
    }

    pub fn get_char(&self) -> char {
        match self {
            Op::Plus => '+',
            Op::Minus => '-',
            Op::Left => '<',
            Op::Right => '>',
            Op::Output => '.',
            Op::Input => ',',
            Op::Start => '[',
            Op::End => ']',
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_char())
    }
}
