use bf::Op;
use source;
use source::token::Token;
use std::str::CharIndices;

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
        let op = Op::new(c)?;
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

pub fn lex<'s>(file: &'s source::File) -> Vec<Token<'s>> {
    file.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use source::span;

    fn ident<'s>(value: &str, span: span::Span<'s>) -> Token<'s> {
        Token::Ident {
            span,
            value: value.to_string(),
        }
    }

    #[test]
    fn bf_complex_0() {
        let source = source::File::new(",[>>+<<-].".to_string());
        let mut s = span::Generator::new(&source);
        let tokens = lex(&source);
        assert_eq!(
            tokens,
            vec![
                Op::Input.token(s.span(1)),
                Op::Start.token(s.span(1)),
                Op::Right.token(s.span(1)),
                Op::Right.token(s.span(1)),
                Op::Plus.token(s.span(1)),
                Op::Left.token(s.span(1)),
                Op::Left.token(s.span(1)),
                Op::Minus.token(s.span(1)),
                Op::End.token(s.span(1)),
                Op::Output.token(s.span(1)),
            ],
        );
    }

    #[test]
    fn single_char_ident() {
        let source = source::File::new("k".to_string());
        let mut s = span::Generator::new(&source);
        let tokens = lex(&source);
        assert_eq!(tokens, vec![ident("k", s.span(1))],);
    }

    #[test]
    fn single_ident() {
        let source = source::File::new("1xY".to_string());
        let mut s = span::Generator::new(&source);
        let tokens = lex(&source);
        assert_eq!(
            tokens,
            vec![Token::Ident {
                span: s.span(3),
                value: "1xY".to_string(),
            }],
        );
    }

    #[test]
    fn idents_0() {
        let source = source::File::new("Test of 4  iD3nT1fi3rZ".to_string());
        let mut s = span::Generator::new(&source);
        let tokens = lex(&source);
        assert_eq!(
            tokens,
            vec![
                ident("Test", s.span(4)),
                ident("of", s.skip(1).span(2)),
                ident("4", s.skip(1).span(1)),
                ident("iD3nT1fi3rZ", s.skip(2).span(11)),
            ],
        );
    }

    #[test]
    fn semicolon() {
        let source = source::File::new(";".to_string());
        let mut s = span::Generator::new(&source);
        let tokens = lex(&source);
        assert_eq!(
            tokens,
            vec![Token::Linebreak {
                span: s.span(1),
                newline: false,
            }],
        );
    }

    #[test]
    fn newline() {
        let source = source::File::new("\n".to_string());
        let mut s = span::Generator::new(&source);
        let tokens = lex(&source);
        assert_eq!(
            tokens,
            vec![Token::Linebreak {
                span: s.span(1),
                newline: true,
            }],
        );
    }

    #[test]
    fn open_brace() {
        let source = source::File::new("{".to_string());
        let mut s = span::Generator::new(&source);
        let tokens = lex(&source);
        assert_eq!(tokens, vec![Token::OpenBrace(s.span(1))],);
    }

    #[test]
    fn close_brace() {
        let source = source::File::new("}".to_string());
        let mut s = span::Generator::new(&source);
        let tokens = lex(&source);
        assert_eq!(tokens, vec![Token::CloseBrace(s.span(1))],);
    }

    #[test]
    fn colon() {
        let source = source::File::new(":".to_string());
        let mut s = span::Generator::new(&source);
        let tokens = lex(&source);
        assert_eq!(tokens, vec![Token::Colon(s.span(1))],);
    }

    #[test]
    fn all_0() {
        let source = source::File::new("_abc: {{\n    [-]\n}^Xy1;}".to_string());
        let mut s = span::Generator::new(&source);
        let tokens = lex(&source);
        assert_eq!(
            tokens,
            vec![
                ident("_abc", s.span(4)),
                Token::Colon(s.span(1)),
                Token::OpenBrace(s.skip(1).span(1)),
                Token::OpenBrace(s.span(1)),
                Token::Linebreak {
                    span: s.span(1),
                    newline: true,
                },
                Op::Start.token(s.skip(4).span(1)),
                Op::Minus.token(s.span(1)),
                Op::End.token(s.span(1)),
                Token::Linebreak {
                    span: s.span(1),
                    newline: true,
                },
                Token::CloseBrace(s.span(1)),
                Token::Ident {
                    span: s.skip(1).span(3),
                    value: "Xy1".to_string(),
                },
                Token::Linebreak {
                    span: s.span(1),
                    newline: false,
                },
                Token::CloseBrace(s.span(1)),
            ]
        );
    }
}
