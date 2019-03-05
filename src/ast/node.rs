use runtime;
use source::Spanned;
use std::fmt;

#[derive(PartialEq)]
pub struct Root {
    ast: Vec<Section>,
}

impl Root {
    pub fn empty() -> Root {
        Root { ast: vec![] }
    }

    pub fn new(ast: Vec<Section>) -> Root {
        Root { ast }
    }
}

impl ::source::Parsable<()> for Root {
    fn parse(p: &mut ::source::Parser, _: ()) -> ::source::ParseResult<Self> {
        Ok(Self::new(p.parse(())?))
    }
}

impl std::ops::Deref for Root {
    type Target = Vec<Section>;

    fn deref(&self) -> &Vec<Section> {
        &self.ast
    }
}

impl fmt::Debug for Root {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AST: {:#?}", self.ast)
    }
}

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

#[derive(PartialEq)]
pub enum Section {
    Line(Spanned<Vec<Segment>>),
    Block(Vec<Box<Section>>),
}

impl ::source::Parsable<()> for Section {
    fn parse(p: &mut ::source::Parser, _: ()) -> ::source::ParseResult<Self> {
        if let Ok(s) = p.parse(()) {
            Ok(Section::Line(s))
        }
        /*else if let Ok(ops) = p.parse(()) {
            Ok(Section::Block(ops))
        }*/
        else {
            Err(None)
        }
    }
}

impl ::runtime::CodeSource for Section {
    fn append_code_to(&self, code: &mut Vec<Spanned<runtime::Op>>) {
        match self {
            Section::Line(line) => {
                for segment in &line.v {
                    segment.append_code_to(code);
                }
            }
            Section::Block(block) => {
                for section in block {
                    section.append_code_to(code);
                }
            }
        }
    }
}

impl fmt::Debug for Section {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Section::Line(line) => {
                write!(f, "Line ")?;
                if let Some(s) = &line.s {
                    write!(f, "{}", s)?;
                }
                f.debug_list().entries(line.v.iter()).finish()
            }
            Section::Block(block) => write!(f, "{:?}", block),
        }
    }
}

#[derive(PartialEq)]
pub enum Segment {
    Bf(Vec<Spanned<runtime::Op>>),
    Comment(Spanned<String>),
}

impl ::source::Parsable<()> for Segment {
    fn parse(p: &mut ::source::Parser, _: ()) -> ::source::ParseResult<Self> {
        if let Ok(s) = p.parse::<Spanned<ImplicitComment>, ()>(()) {
            Ok(Segment::Comment(s))
        } else if let Ok(ops) = p.parse(()) {
            Ok(Segment::Bf(ops))
        } else {
            Err(None)
        }
    }
}

impl ::runtime::CodeSource for Segment {
    fn append_code_to(&self, code: &mut Vec<Spanned<runtime::Op>>) {
        match self {
            Segment::Bf(bf) => {
                code.extend(bf.clone());
            }
            Segment::Comment(_) => (),
        }
    }
}

impl fmt::Debug for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Segment::Bf(bf) => write!(
                f,
                "({})",
                bf.into_iter().map(|i| i.v.get_char()).collect::<String>()
            ),
            Segment::Comment(comment) => write!(f, "{:?}", comment.v),
        }
    }
}
