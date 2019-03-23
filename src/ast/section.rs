use super::Segment;
use source::Spanned;

struct Block {
    sections: Box<Vec<Section>>,
}

impl ::source::Parsable<()> for Block {
    fn parse(p: &mut ::source::Parser, _: ()) -> ::source::ParseResult<Self> {
        let _ = p.parse::<String, &str>(r"\s+")?;
        let sections = p.parse(())?;
        Ok(Block { sections })
    }
}

#[derive(PartialEq)]
pub enum Section {
    Line(Spanned<Vec<Segment>>),
    Block(Box<Vec<Section>>),
}

impl ::source::Parsable<()> for Section {
    fn parse(p: &mut ::source::Parser, _: ()) -> ::source::ParseResult<Self> {
        if let Ok(b) = p.parse::<Block, ()>(()) {
            Ok(Section::Block(b.sections))
        } else if let Ok(s) = p.parse(()) {
            let _ = p.parse::<(), &str>("\n"); // if it fails, that's fine (we might be at EOF)
            Ok(Section::Line(s))
        } else {
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
                for section in block.iter() {
                    section.append_code_to(code);
                }
            }
        }
    }
}

impl std::fmt::Debug for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Section::Line(line) => write!(f, "Line {:#?}", line),
            Section::Block(block) => write!(f, "Block {:#?}", block),
        }
    }
}
