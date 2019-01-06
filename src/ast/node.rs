use runtime;
use source::File;
use source::Span;
use source::Spanned;
use std::rc::Rc;

#[derive(Clone, Copy)]
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

impl Op {
    fn runtime_op(&self) -> runtime::Op {
        match self {
            Op::Plus => runtime::Op::Plus,
            Op::Minus => runtime::Op::Minus,
            Op::Left => runtime::Op::Left,
            Op::Right => runtime::Op::Right,
            Op::Output => runtime::Op::Output,
            Op::Input => runtime::Op::Input,
        }
    }
}

impl runtime::CodeSource for Ops {
    fn append_code_to(&self, code: &mut Vec<Spanned<runtime::Op>>) {
        for op in &self.ops {
            code.push(Spanned::new(op.span.clone(), op.value.runtime_op()));
        }
    }
}

impl runtime::CodeSource for Loop {
    fn append_code_to(&self, code: &mut Vec<Spanned<runtime::Op>>) {
        code.push(Spanned::new(self.start.clone(), runtime::Op::Start));
        self.body.append_code_to(code);
        code.push(Spanned::new(self.end.clone(), runtime::Op::End));
    }
}

impl runtime::CodeSource for Node {
    fn append_code_to(&self, code: &mut Vec<Spanned<runtime::Op>>) {
        match self {
            Node::Ops(o) => o.append_code_to(code),
            Node::Loop(l) => l.append_code_to(code),
            Node::Comment(_) => (),
        }
    }
}

impl runtime::CodeSource for Nodes {
    fn append_code_to(&self, code: &mut Vec<Spanned<runtime::Op>>) {
        for node in &self.nodes {
            node.append_code_to(code);
        }
    }
}
