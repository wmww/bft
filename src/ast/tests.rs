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
