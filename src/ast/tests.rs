use super::*;
use runtime::Op;
use source::span::TestBuilder;

fn check(file: ::std::rc::Rc<::source::File>, ast: Vec<Section>) {
    assert_eq!(parse(file), root::Root::new(ast))
}

#[test]
fn plus_op() {
    let mut b = TestBuilder::new("+");

    check(
        b.file.clone(),
        vec![Section::Line(
            b.clone()
                .span(1)
                .around(vec![Segment::Bf(vec![b.span(1).around(Op::Plus)])]),
        )],
    );
}

#[test]
fn all_ops() {
    let mut b = TestBuilder::new("[+--><.,]");

    check(
        b.file.clone(),
        vec![Section::Line(b.clone().span(9).around(vec![Segment::Bf(
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
        )]))],
    );
}

#[test]
fn comment() {
    let mut b = TestBuilder::new("test_comment");

    check(
        b.file.clone(),
        vec![Section::Line(b.clone().span(12).around(vec![
            Segment::Comment(b.span(12).around("test_comment".to_string())),
        ]))],
    );
}

#[test]
fn comment_and_code() {
    let mut b = TestBuilder::new("+comment>-another.");

    check(
        b.file.clone(),
        vec![Section::Line(b.clone().span(18).around(vec![
            Segment::Bf(vec![b.span(1).around(Op::Plus)]),
            Segment::Comment(b.span(7).around("comment".to_string())),
            Segment::Bf(vec![
                b.span(1).around(Op::Right),
                b.span(1).around(Op::Minus),
            ]),
            Segment::Comment(b.span(7).around("another".to_string())),
            Segment::Bf(vec![b.span(1).around(Op::Output)]),
        ]))],
    );
}

#[test]
fn multi_line() {
    let mut b = TestBuilder::new(
        "+[
hello
comment]",
    );

    let mut l = b.clone();

    check(
        b.file.clone(),
        vec![
            Section::Line(l.span(2).around(vec![Segment::Bf(vec![
                b.span(1).around(Op::Plus),
                b.span(1).around(Op::Start),
            ])])),
            Section::Line(l.skip(1).span(5).around(vec![Segment::Comment(
                b.skip(1).span(5).around("hello".to_string()),
            )])),
            Section::Line(l.skip(1).span(8).around(vec![
                Segment::Comment(b.skip(1).span(7).around("comment".to_string())),
                Segment::Bf(vec![b.span(1).around(Op::End)]),
            ])),
        ],
    );
}
