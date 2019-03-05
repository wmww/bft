use runtime;
use source::Spanned;

impl ::source::Parsable<()> for runtime::Op {
    fn parse(p: &mut ::source::Parser, _: ()) -> ::source::ParseResult<Self> {
        if let Some(c) = p.next_char() {
            if let Some(op) = runtime::Op::new(c) {
                return Ok(op);
            }
        }
        Err(None)
    }
}

// A comment made of non-bf characters on a line
type ImplicitComment = String;

impl ::source::Parsable<()> for ImplicitComment {
    fn parse(p: &mut ::source::Parser, _: ()) -> ::source::ParseResult<Self> {
        let start = p.clone();
        loop {
            // Break if at the end of line or end of file
            if let None /*| Some('\n')*/ = p.try_next_char() {
                break;
            }
            // Break if at a valid bf op
            if let Ok(_) = p.try_parse::<runtime::Op, ()>(()) {
                break;
            }
            p.next_char();
        }
        if start != *p {
            Ok(start.string_between(p))
        } else {
            Err(None)
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Node {
    Bf(Vec<Spanned<runtime::Op>>),
    Comment(Spanned<String>),
}

impl ::source::Parsable<()> for Node {
    fn parse(p: &mut ::source::Parser, _: ()) -> ::source::ParseResult<Self> {
        if let Ok(s) = p.parse::<Spanned<ImplicitComment>, ()>(()) {
            Ok(Node::Comment(s))
        } else if let Ok(ops) = p.parse(()) {
            Ok(Node::Bf(ops))
        } else {
            Err(None)
        }
    }
}

impl ::runtime::CodeSource for Node {
    fn append_code_to(&self, code: &mut Vec<Spanned<runtime::Op>>) {
        match self {
            Node::Bf(bf) => {
                for i in bf {
                    code.push(i.clone());
                }
            }
            Node::Comment(_) => (),
        }
    }
}
