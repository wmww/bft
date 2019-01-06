use super::*;

#[derive(Clone, Copy)]
pub enum Op {
    Plus,
    Minus,
    Left,
    Right,
    Output,
    Input,
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

impl runtime::CodeSource for Spanned<Op> {
    fn append_code_to(&self, code: &mut Vec<Spanned<runtime::Op>>) {
        code.push(Spanned::new(self.span.clone(), self.value.runtime_op()));
    }
}

pub struct Loop {
    start: Span,
    body: Seq<Elem>,
    end: Span,
}

impl runtime::CodeSource for Loop {
    fn append_code_to(&self, code: &mut Vec<Spanned<runtime::Op>>) {
        code.push(Spanned::new(self.start.clone(), runtime::Op::Start));
        self.body.append_code_to(code);
        code.push(Spanned::new(self.end.clone(), runtime::Op::End));
    }
}
