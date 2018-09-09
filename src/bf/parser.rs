use bf::code::Code;
use bf::ops::Op;
use log;
use source;
use source::Token;

type ParseResult<'s> = Result<Vec<Code<'s>>, (Vec<Code<'s>>, Vec<log::Issue<'s>>)>;

fn parse_section<'s>(tokens: &'s Vec<Token<'s>>, start: usize, end: usize) -> ParseResult<'s> {
    let mut code = Vec::new();
    let mut issues = Vec::new();
    let mut ops: Vec<(Op, source::Span<'s>)> = Vec::new();
    let mut i = start;
    while i < end {
        match &tokens[i] {
            Token::Bf { op, span } => {
                ops.push((op.clone(), span.clone()));
            }
            Token::CloseLoop(span) => {
                issues.push(span.issue(log::Severity::Error, "Extraneous closing bracket"));
            }
            Token::OpenLoop(open_span) => {
                let mut j = i + 1;
                let mut level = 1;
                while j < end {
                    match tokens[j] {
                        Token::OpenLoop(_) => {
                            level += 1;
                        }
                        Token::CloseLoop(_) => {
                            level -= 1;
                            if level <= 0 {
                                break;
                            }
                        }
                        _ => (),
                    }
                    j += 1;
                }
                if level <= 0 {
                    if !ops.is_empty() {
                        code.push(Code::Ops(ops));
                        ops = Vec::new();
                    }
                    let contents = parse_section(tokens, i + 1, j)?;
                    let span = source::Span::between(tokens[i].span(), tokens[j].span());
                    code.push(Code::Loop(contents, span));
                    i = j;
                } else {
                    issues.push(open_span.issue(log::Severity::Error, "Loop not terminated"));
                }
            }
            _ => (),
        }
        i += 1;
    }
    if !ops.is_empty() {
        code.push(Code::Ops(ops));
    }
    if issues.is_empty() {
        Ok(code)
    } else {
        Err((code, issues))
    }
}

pub fn parse<'s>(tokens: &'s Vec<Token<'s>>) -> ParseResult<'s> {
    parse_section(tokens, 0, tokens.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_parse(tokens: &Vec<Token>, expected: ParseResult) {
        match expected {
            Ok(expected) => match parse(tokens) {
                Ok(code) => {
                    if code != expected {
                        panic!(
                            "got unexpeceted result:\n     input: {:?}\n  expected: {:?}\n    output: {:?}",
                            tokens,
                            expected,
                            code
                        );
                    }
                }
                Err(e) => {
                    panic!(
                        "error parsing:\n    issues: {}\n     input: {:?}\n  expected: {:?}\n    output: {:?}",
                        e.1.iter().fold(
                            String::new(), | sum, val | format!(
                                "{}{}{:?}",
                                sum,
                                if sum.is_empty() { "" } else { "\n            " },
                                val
                            ),
                        ),
                        tokens,
                        expected,
                        e.0
                    );
                }
            },
            Err(expected) => match parse(tokens) {
                Ok(code) => {
                    panic!(
                        "parsing succeeded when it shouldn't have:\n     input: {:?}\n  expected: {:?}\n    output: {:?}",
                        tokens,
                        expected,
                        code
                    );
                }
                Err(e) => {
                    if e.0 != expected.0 || e.1 != expected.1 {
                        panic!(
                            "parsing gave incorrect error:\n     input: {:?}\n  expected: {:?}\n    output: {:?}\n  expected errors: {:?}\n    output errors: {:?}",
                            tokens,
                            expected.0,
                            e.0,
                            expected.1,
                            e.1
                        );
                    }
                }
            },
        }
    }

    #[test]
    fn simple_bf() {
        let file = source::File::new(String::new());
        let mut s = source::span::Generator::new(&file);
        let tokens = vec![
            Op::Plus.token(s.span(1)),
            Op::Minus.token(s.span(1)),
            Op::Minus.token(s.span(1)),
            Op::Input.token(s.span(1)),
            Op::Output.token(s.span(1)),
            Op::Left.token(s.span(1)),
            Op::Right.token(s.span(1)),
        ];
        s.reset();
        let code = vec![Code::Ops(vec![
            (Op::Plus, s.span(1)),
            (Op::Minus, s.span(1)),
            (Op::Minus, s.span(1)),
            (Op::Input, s.span(1)),
            (Op::Output, s.span(1)),
            (Op::Left, s.span(1)),
            (Op::Right, s.span(1)),
        ])];
        test_parse(&tokens, Ok(code));
    }

    #[test]
    fn simple_loop() {
        let file = source::File::new(String::new());
        let mut s = source::span::Generator::new(&file);
        let tokens = vec![
            Op::Right.token(s.span(1)),
            Op::Plus.token(s.span(1)),
            Token::OpenLoop(s.span(1)),
            Op::Plus.token(s.span(1)),
            Op::Left.token(s.span(1)),
            Op::Output.token(s.span(1)),
            Token::CloseLoop(s.span(1)),
            Op::Minus.token(s.span(1)),
        ];
        s.reset();
        let code = vec![
            Code::Ops(vec![(Op::Right, s.span(1)), (Op::Plus, s.span(1))]),
            Code::Loop(
                vec![Code::Ops(vec![
                    (Op::Plus, s.skip(1).span(1)),
                    (Op::Left, s.span(1)),
                    (Op::Output, s.span(1)),
                ])],
                s.skip(-4).span(5),
            ),
            Code::Ops(vec![(Op::Minus, s.span(1))]),
        ];
        test_parse(&tokens, Ok(code));
    }

    #[test]
    fn nested_loops() {
        let file = source::File::new(String::new());
        let mut s = source::span::Generator::new(&file);
        let tokens = vec![
            Op::Right.token(s.span(1)),
            Op::Plus.token(s.span(1)),
            Token::OpenLoop(s.span(1)),
            Op::Left.token(s.span(1)),
            Op::Left.token(s.span(1)),
            Token::OpenLoop(s.span(1)),
            Op::Plus.token(s.span(1)),
            Token::CloseLoop(s.span(1)),
            Op::Left.token(s.span(1)),
            Token::CloseLoop(s.span(1)),
            Op::Output.token(s.span(1)),
            Token::OpenLoop(s.span(1)),
            Token::CloseLoop(s.span(1)),
            Op::Minus.token(s.span(1)),
        ];
        s.reset();
        let code = vec![
            Code::Ops(vec![(Op::Right, s.span(1)), (Op::Plus, s.span(1))]),
            Code::Loop(
                vec![
                    Code::Ops(vec![(Op::Left, s.skip(1).span(1)), (Op::Left, s.span(1))]),
                    Code::Loop(
                        vec![Code::Ops(vec![(Op::Plus, s.skip(1).span(1))])],
                        s.skip(-2).span(3),
                    ),
                    Code::Ops(vec![(Op::Left, s.span(1))]),
                ],
                s.skip(-7).span(8),
            ),
            Code::Ops(vec![(Op::Output, s.span(1))]),
            Code::Loop(vec![], s.span(2)),
            Code::Ops(vec![(Op::Minus, s.span(1))]),
        ];
        test_parse(&tokens, Ok(code));
    }

    #[test]
    fn touching_loops() {
        let file = source::File::new(String::new());
        let mut s = source::span::Generator::new(&file);
        let tokens = vec![
            Token::OpenLoop(s.span(1)),
            Token::CloseLoop(s.span(1)),
            Token::OpenLoop(s.span(1)),
            Token::OpenLoop(s.span(1)),
            Token::OpenLoop(s.span(1)),
            Token::CloseLoop(s.span(1)),
            Token::OpenLoop(s.span(1)),
            Token::CloseLoop(s.span(1)),
            Token::CloseLoop(s.span(1)),
            Token::CloseLoop(s.span(1)),
            Token::OpenLoop(s.span(1)),
            Token::CloseLoop(s.span(1)),
        ];
        s.reset();
        let code = vec![
            Code::Loop(vec![], s.span(2)),
            Code::Loop(
                vec![Code::Loop(
                    vec![
                        Code::Loop(vec![], s.skip(2).span(2)),
                        Code::Loop(vec![], s.span(2)),
                    ],
                    s.skip(-5).span(6),
                )],
                s.skip(-7).span(8),
            ),
            Code::Loop(vec![], s.span(2)),
        ];
        test_parse(&tokens, Ok(code));
    }
}
