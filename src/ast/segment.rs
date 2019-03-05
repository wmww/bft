use source::Spanned;

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
            if let Ok(_) = p.try_parse::<::runtime::Op, ()>(()) {
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
pub enum Segment {
    Bf(Vec<Spanned<::runtime::Op>>),
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
    fn append_code_to(&self, code: &mut Vec<Spanned<::runtime::Op>>) {
        match self {
            Segment::Bf(bf) => {
                code.extend(bf.clone());
            }
            Segment::Comment(_) => (),
        }
    }
}

impl std::fmt::Debug for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
