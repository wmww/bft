use bf::code::Code;
use bf::ops::Op;
use log;
use source;
use source::Token;

fn parse_section<'s>(
    tokens: &'s Vec<Token<'s>>,
    start: usize,
    end: usize,
) -> Result<Vec<Code<'s>>, log::Issue<'s>> {
    let mut code = Vec::new();
    let mut ops: Vec<(Op, source::Span<'s>)> = Vec::new();
    let mut i = start;
    while i < end {
        match &tokens[i] {
            Token::Bf { op, span } => {
                ops.push((op.clone(), span.clone()));
            }
            Token::CloseLoop(span) => {
                return Err(span.issue(log::Severity::Error, "extra closing bracket"));
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
                        let contents = parse_section(tokens, i + 1, j)?;
                        let span = source::Span::between(tokens[i].span(), tokens[j].span());
                        code.push(Code::Loop(contents, span));
                        i = j;
                    }
                } else {
                    return Err(open_span.issue(log::Severity::Error, "no closing bracket"));
                }
            }
            _ => (),
        }
        i += 1;
    }
    if !ops.is_empty() {
        code.push(Code::Ops(ops));
    }
    Ok(code)
}

pub fn parse<'s>(tokens: &'s Vec<Token<'s>>) -> Result<Vec<Code<'s>>, log::Issue<'s>> {
    parse_section(tokens, 0, tokens.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use source::Span;

    #[test]
    fn simple_bf() {
        let source = source::File::new(String::new());
        let tokens = vec![
            Token::Bf {
                op: Op::Plus,
                span: Span::from_components(&source, 0, 1),
            },
            Token::Bf {
                op: Op::Minus,
                span: Span::from_components(&source, 1, 1),
            },
            Token::Bf {
                op: Op::Input,
                span: Span::from_components(&source, 2, 1),
            },
            Token::Bf {
                op: Op::Output,
                span: Span::from_components(&source, 3, 1),
            },
            Token::Bf {
                op: Op::Left,
                span: Span::from_components(&source, 4, 1),
            },
            Token::Bf {
                op: Op::Right,
                span: Span::from_components(&source, 5, 1),
            },
        ];
        let code = vec![Code::Ops(vec![
            (Op::Plus, Span::from_components(&source, 0, 1)),
            (Op::Minus, Span::from_components(&source, 1, 1)),
            (Op::Input, Span::from_components(&source, 2, 1)),
            (Op::Output, Span::from_components(&source, 3, 1)),
            (Op::Left, Span::from_components(&source, 4, 1)),
            (Op::Right, Span::from_components(&source, 5, 1)),
        ])];
        let gen_code = match parse(&tokens) {
            Ok(c) => c,
            Err(e) => panic!("error parsing: {}", e.message),
        };
        assert_eq!(gen_code, code);
    }

    #[test]
    fn simple_loop() {
        let source = source::File::new(String::new());
        let tokens = vec![
            Token::Bf {
                    op: Op::Right,
                    span: Span::from_components(&source, 0, 1),
                },
            Token::Bf {
                    op: Op::Plus,
                    span: Span::from_components(&source, 1, 1),
                },
            Token::OpenLoop(Span::from_components(&source, 2, 1)),
            Token::Bf {
                op: Op::Plus,
                span: Span::from_components(&source, 3, 1),
            },
            Token::Bf {
                op: Op::Left,
                span: Span::from_components(&source, 4, 1),
            },
            Token::Bf {
                op: Op::Output,
                span: Span::from_components(&source, 5, 1),
            },
            Token::CloseLoop(Span::from_components(&source, 6, 1)),
            Token::Bf {
                    op: Op::Minus,
                    span: Span::from_components(&source, 7, 1),
                },
        ];
        let code = vec![Code::Ops(vec![
            (Op::Right, Span::from_components(&source, 0, 1)),
            (Op::Plus, Span::from_components(&source, 1, 1)),
        ]), Code::Loop(vec![Code::Ops(vec![
            (Op::Plus, Span::from_components(&source, 3, 1)),
            (Op::Left, Span::from_components(&source, 4, 1)),
            (Op::Output, Span::from_components(&source, 5, 1)),
            ])], Span::from_components(&source, 2, 5)),
        Code::Ops(vec![
            (Op::Minus, Span::from_components(&source, 7, 1)),
        ])];
        let gen_code = match parse(&tokens) {
            Ok(c) => c,
            Err(e) => panic!("error parsing: {}", e.message),
        };
        assert_eq!(gen_code, code);
    }
}
