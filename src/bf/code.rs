use bf::ops::Op;
use source;

#[derive(Debug, Clone, PartialEq)]
pub enum Code<'s> {
    Ops(Vec<(Op, source::Span<'s>)>),
    Loop(Vec<Code<'s>>, source::Span<'s>),
}
