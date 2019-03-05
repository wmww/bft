mod root;
mod section;
mod segment;

use runtime;
use source::Spanned;

pub use self::root::parse;
use self::section::Section;
use self::segment::Segment;

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

#[cfg(test)]
mod tests;
