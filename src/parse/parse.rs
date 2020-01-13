use combine::error::StringStreamError;
use combine::stream::*;
use combine::*;

use ast::Root;
use ast::Section;
use runtime::Op;
use source::Spanned;

fn program<I>() -> impl Parser<Input = I, Output = Root>
where
    I: Stream<Item = char>,
    // Necessary due to rust-lang/rust#24159
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let not_bf = none_of("+-<>.,[]".chars());

    let op = satisfy_map(Op::new);
    let bf = many1(op);
    let section_bf =
        bf.map(|ops: Vec<Op>| Section::Bf(ops.iter().map(|op| Spanned::new(op.clone())).collect()));

    let comment = many1(not_bf);
    let section_comment = comment.map(|text: String| Section::Comment(Spanned::new(text)));

    let section = choice((section_bf, section_comment));
    let program = many(section);

    program.map(|sections: Vec<Section>| Root::new(sections))
}

pub fn parse<'a>(file: &'a::source::File) -> Result<Root, StringStreamError> {
    Ok(program().parse(&file.contents as &str)?.0)
}
