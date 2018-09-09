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
