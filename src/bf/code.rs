use bf::ops::Op;
use source;

#[derive(Debug, Clone, PartialEq)]
pub enum Code<'s> {
    Op(Op, source::Span<'s>),
    Loop(Vec<Code<'s>>, source::Span<'s>),
}
