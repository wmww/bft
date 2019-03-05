use super::*;
use runtime::Op;
use source::span::TestBuilder;

#[test]
fn plus_op() {
    let mut b = TestBuilder::new("+");

    assert_eq!(
        parse(b.file.clone()),
        vec![Node::Bf(vec![b.span(1).around(Op::Plus)])]
    );
}

#[test]
#[ignore]
fn all_ops() {
    let mut b = TestBuilder::new("[+--><.,]");

    assert_eq!(
        parse(b.file.clone()),
        vec![Node::Bf(vec![
            b.span(1).around(Op::Start),
            b.span(1).around(Op::Plus),
            b.span(1).around(Op::Minus),
            b.span(1).around(Op::Minus),
            b.span(1).around(Op::Right),
            b.span(1).around(Op::Left),
            b.span(1).around(Op::Output),
            b.span(1).around(Op::Input),
            b.span(1).around(Op::End),
        ])],
    );
}
