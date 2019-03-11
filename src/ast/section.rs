use super::Segment;
use source::Spanned;

#[derive(PartialEq)]
pub enum Section {
    Line(Spanned<Vec<Segment>>),
    Block(Vec<Box<Section>>),
}

impl ::source::Parsable<()> for Section {
    fn parse(p: &mut ::source::Parser, _: ()) -> ::source::ParseResult<Self> {
        if let Ok(s) = p.parse(()) {
            let _ = p.parse::<(), &str>("\n");
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
    fn append_code_to(&self, code: &mut Vec<Spanned<::runtime::Op>>) {
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

impl std::fmt::Debug for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
