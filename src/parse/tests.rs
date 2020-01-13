use ast::*;
use source;
use runtime::Op;

fn assert_str_parses_to_ast(code: &str, expected: Vec<Section>) {
    let file = source::File::from_string(code.into());
    let actual = super::parse(&file);
    assert_eq!(actual, Ok(Root::new(expected)));
}

/// TODO: remove this
struct StubSpan {
}

impl StubSpan {
    fn around<T>(self, v: T) -> source::Spanned<T> {
        source::Spanned { s: None, v }
    }
}

/// TODO: remove this
#[derive(Clone)]
struct StubSpanBuilder {
}

impl StubSpanBuilder{
    pub fn span(&self, len: usize) -> StubSpan {
        StubSpan{}
    }

    pub fn skip<'a>(&'a self, len: usize) -> &'a StubSpanBuilder {
        self
    }
}

#[test]
fn empty() {
    let code = "";
    let expected = vec![];
    assert_str_parses_to_ast(code, expected);
}

#[test]
fn plus_op() {
    let b = StubSpanBuilder{};
    let code = "+";
    let expected = vec![Section::Bf(vec![b.span(1).around(Op::Plus)])];
    assert_str_parses_to_ast(code, expected);
}

#[test]
fn all_ops() {
    let b = StubSpanBuilder{};
    let code = "[+--><.,]";
    let expected = vec![Section::Bf(
            vec![
                b.span(1).around(Op::Start),
                b.span(1).around(Op::Plus),
                b.span(1).around(Op::Minus),
                b.span(1).around(Op::Minus),
                b.span(1).around(Op::Right),
                b.span(1).around(Op::Left),
                b.span(1).around(Op::Output),
                b.span(1).around(Op::Input),
                b.span(1).around(Op::End),
            ],
        )];
    assert_str_parses_to_ast(code, expected);
}

#[test]
fn comment() {
    let b = StubSpanBuilder{};
    let code = "test_comment";
    let expected = vec![
            Section::Comment(b.span(12).around("test_comment".to_string())),
        ];
    assert_str_parses_to_ast(code, expected);

}

#[test]
fn comment_and_code() {
    let b = StubSpanBuilder{};
    let code = "+comment>-another.";
    let expected = vec![
            Section::Bf(vec![b.span(1).around(Op::Plus)]),
            Section::Comment(b.span(7).around("comment".to_string())),
            Section::Bf(vec![
                b.span(1).around(Op::Right),
                b.span(1).around(Op::Minus),
            ]),
            Section::Comment(b.span(7).around("another".to_string())),
            Section::Bf(vec![b.span(1).around(Op::Output)]),
        ];
    assert_str_parses_to_ast(code, expected);
}

