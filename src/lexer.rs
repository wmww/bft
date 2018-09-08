use bf;
use source;
use std::str::CharIndices;
use token;
use token::Token;

pub struct Tokens<'s> {
    source: &'s source::File,
    chars: CharIndices<'s>,
}

impl<'s> Tokens<'s> {
    fn span_to(&self, end: CharIndices<'s>) -> source::Span<'s> {
        source::Span::from_indices(self.source, self.chars.clone(), end)
    }

    fn lex_bf(&self) -> Option<Token<'s>> {
        let mut chars = self.chars.clone();
        let (_, c) = chars.next()?;
        let op = bf::Op::new(c)?;
        let span = self.span_to(chars);
        Some(Token::Bf { span: span, op: op })
    }

    fn lex_single_char_token(&self) -> Option<Token<'s>> {
        let mut chars = self.chars.clone();
        let (_, c) = chars.next()?;
        let span = self.span_to(chars);
        match c {
            ';' => Some(Token::Linebreak {
                span,
                newline: false,
            }),
            '\n' => Some(Token::Linebreak {
                span,
                newline: true,
            }),
            '{' => Some(Token::OpenBrace(span)),
            '}' => Some(Token::CloseBrace(span)),
            ':' => Some(Token::Colon(span)),
            _ => None,
        }
    }

    fn lex_ident(&self) -> Option<Token<'s>> {
        let mut chars = self.chars.clone();
        let mut prev = chars.clone();
        let mut ident = None;
        while let Some((_, c)) = chars.next() {
            match c {
                '0'...'9' | 'a'...'z' | 'A'...'Z' | '_' => {
                    ident = {
                        let mut ident = ident.unwrap_or_else(|| String::new());
                        ident.push(c);
                        Some(ident)
                    };
                    prev = chars.clone();
                }
                _ => break,
            }
        }
        ident.map(|ident| {
            let span = self.span_to(prev);
            Token::Ident {
                span: span,
                value: ident,
            }
        })
    }
}

impl<'s> Iterator for Tokens<'s> {
    type Item = Token<'s>;

    fn next(&mut self) -> Option<Token<'s>> {
        loop {
            match None
                .or_else(|| self.lex_bf())
                .or_else(|| self.lex_single_char_token())
                .or_else(|| self.lex_ident())
            {
                Some(t) => {
                    while match self.chars.clone().next() {
                        Some((i, _)) => i,
                        None => self.source.contents.len(),
                    } < t.span().end()
                    {
                        self.chars.next();
                    }
                    return Some(t);
                }
                None => self.chars.next()?,
            };
        }
    }
}

impl<'src> IntoIterator for &'src source::File {
    type Item = Token<'src>;
    type IntoIter = Tokens<'src>;

    fn into_iter(self) -> Tokens<'src> {
        Tokens {
            source: self,
            chars: self.contents.char_indices(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_bf_complex_0() {
        let source = source::File::new(",[>>+<<-].".to_string());
        let tokens = token::Seq::lex(&source);
        assert_eq!(
            tokens.tokens,
            vec![
                Token::Bf {
                    span: source::Span::from_components(&source, 0, 1),
                    op: bf::Op::Input,
                },
                Token::Bf {
                    span: source::Span::from_components(&source, 1, 1),
                    op: bf::Op::Open,
                },
                Token::Bf {
                    span: source::Span::from_components(&source, 2, 1),
                    op: bf::Op::Right,
                },
                Token::Bf {
                    span: source::Span::from_components(&source, 3, 1),
                    op: bf::Op::Right,
                },
                Token::Bf {
                    span: source::Span::from_components(&source, 4, 1),
                    op: bf::Op::Plus,
                },
                Token::Bf {
                    span: source::Span::from_components(&source, 5, 1),
                    op: bf::Op::Left,
                },
                Token::Bf {
                    span: source::Span::from_components(&source, 6, 1),
                    op: bf::Op::Left,
                },
                Token::Bf {
                    span: source::Span::from_components(&source, 7, 1),
                    op: bf::Op::Minus,
                },
                Token::Bf {
                    span: source::Span::from_components(&source, 8, 1),
                    op: bf::Op::Close,
                },
                Token::Bf {
                    span: source::Span::from_components(&source, 9, 1),
                    op: bf::Op::Output,
                },
            ],
        );
    }

    #[test]
    fn lex_single_char_ident() {
        let source = source::File::new("k".to_string());
        let tokens = token::Seq::lex(&source);
        assert_eq!(
            tokens.tokens,
            vec![Token::Ident {
                span: source::Span::from_components(&source, 0, 1),
                value: "k".to_string(),
            }],
        );
    }

    #[test]
    fn lex_single_ident() {
        let source = source::File::new("1xY".to_string());
        let tokens = token::Seq::lex(&source);
        assert_eq!(
            tokens.tokens,
            vec![Token::Ident {
                span: source::Span::from_components(&source, 0, 3),
                value: "1xY".to_string(),
            }],
        );
    }

    #[test]
    fn lex_idents_0() {
        let source = source::File::new("Test of 1 iD3NT1fi3r".to_string());
        let tokens = token::Seq::lex(&source);
        assert_eq!(
            tokens.tokens,
            vec![
                Token::Ident {
                    span: source::Span::from_components(&source, 0, 4),
                    value: "Test".to_string(),
                },
                Token::Ident {
                    span: source::Span::from_components(&source, 5, 2),
                    value: "of".to_string(),
                },
                Token::Ident {
                    span: source::Span::from_components(&source, 8, 1),
                    value: "1".to_string(),
                },
                Token::Ident {
                    span: source::Span::from_components(&source, 10, 10),
                    value: "iD3NT1fi3r".to_string(),
                },
            ],
        );
    }

    #[test]
    fn lex_semicolon() {
        let source = source::File::new(";".to_string());
        let tokens = token::Seq::lex(&source);
        assert_eq!(
            tokens.tokens,
            vec![Token::Linebreak {
                span: source::Span::from_components(&source, 0, 1),
                newline: false,
            }],
        );
    }

    #[test]
    fn lex_newline() {
        let source = source::File::new("\n".to_string());
        let tokens = token::Seq::lex(&source);
        assert_eq!(
            tokens.tokens,
            vec![Token::Linebreak {
                span: source::Span::from_components(&source, 0, 1),
                newline: true,
            }],
        );
    }

    #[test]
    fn lex_open_brace() {
        let source = source::File::new("{".to_string());
        let tokens = token::Seq::lex(&source);
        assert_eq!(
            tokens.tokens,
            vec![Token::OpenBrace(source::Span::from_components(
                &source, 0, 1,
            ))],
        );
    }

    #[test]
    fn lex_close_brace() {
        let source = source::File::new("}".to_string());
        let tokens = token::Seq::lex(&source);
        assert_eq!(
            tokens.tokens,
            vec![Token::CloseBrace(source::Span::from_components(
                &source, 0, 1,
            ))],
        );
    }

    #[test]
    fn lex_colon() {
        let source = source::File::new(":".to_string());
        let tokens = token::Seq::lex(&source);
        assert_eq!(
            tokens.tokens,
            vec![Token::Colon(source::Span::from_components(&source, 0, 1))],
        );
    }

    #[test]
    fn lex_all_0() {
        let source = source::File::new("_abc: {{\n    [-]\n}^Xy1;}".to_string());
        let tokens = token::Seq::lex(&source);
        assert_eq!(
            tokens.tokens,
            vec![
                Token::Ident {
                    span: source::Span::from_components(&source, 0, 4),
                    value: "_abc".to_string(),
                },
                Token::Colon(source::Span::from_components(&source, 4, 1)),
                Token::OpenBrace(source::Span::from_components(&source, 6, 1)),
                Token::OpenBrace(source::Span::from_components(&source, 7, 1)),
                Token::Linebreak {
                    span: source::Span::from_components(&source, 8, 1),
                    newline: true,
                },
                Token::Bf {
                    span: source::Span::from_components(&source, 13, 1),
                    op: bf::Op::Open,
                },
                Token::Bf {
                    span: source::Span::from_components(&source, 14, 1),
                    op: bf::Op::Minus,
                },
                Token::Bf {
                    span: source::Span::from_components(&source, 15, 1),
                    op: bf::Op::Close,
                },
                Token::Linebreak {
                    span: source::Span::from_components(&source, 16, 1),
                    newline: true,
                },
                Token::CloseBrace(source::Span::from_components(&source, 17, 1)),
                Token::Ident {
                    span: source::Span::from_components(&source, 19, 3),
                    value: "Xy1".to_string(),
                },
                Token::Linebreak {
                    span: source::Span::from_components(&source, 22, 1),
                    newline: false,
                },
                Token::CloseBrace(source::Span::from_components(&source, 23, 1)),
            ],
        );
    }
}
