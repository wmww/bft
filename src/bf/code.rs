use bf::ops::Op;
use source;

pub enum Code<'s> {
    Ops(Vec<(Op, source::Span<'s>)>),
    Loop(Vec<Code<'s>>, source::Span<'s>),
}
