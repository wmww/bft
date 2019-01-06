use runtime;
use source::File;
use source::Span;
use source::Spanned;
use std::rc::Rc;

pub enum Op {
    Plus,
    Minus,
    Left,
    Right,
    Output,
    Input,
}

pub struct Ops {
    ops: Vec<Spanned<Op>>,
}

pub struct Loop {
    start: Span,
    body: Nodes,
    end: Span,
}

pub enum Node {
    Ops(Ops),
    Loop(Loop),
    Comment(Spanned<String>),
}

pub struct Nodes {
    nodes: Vec<Node>,
}

pub fn parse(file: Rc<File>) -> Nodes {
    Nodes { nodes: vec![] }
}

impl runtime::CodeSource for Nodes {
    fn get_code(&self) -> Vec<Spanned<runtime::Op>> {
        vec![]
    }
}
