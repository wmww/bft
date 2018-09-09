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
        let gen_code = match parse(&tokens) {
            Ok(c) => c,
            Err(e) => panic!("error parsing: {}", e.message),
        };
        assert_eq!(gen_code, code);
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
        let gen_code = match parse(&tokens) {
            Ok(c) => c,
            Err(e) => panic!("error parsing: {}", e.message),
        };
        assert_eq!(gen_code, code);
    }
}
