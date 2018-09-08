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
    fn span_to(&mut self, end: CharIndices<'s>) -> source::Span<'s> {
        let start = self.chars.clone();
        self.chars = end.clone();
        source::Span::from_indices(self.source, start, end)
    }

    fn lex_bf(&mut self) -> Option<Token<'s>> {
        let mut chars = self.chars.clone();
        let (_, c) = chars.next()?;
        let op = bf::Op::new(c)?;
        let span = self.span_to(chars);
        Some(Token::Bf { span: span, op: op })
    }

    fn lex_ident(&mut self) -> Option<Token<'s>> {
        let mut chars = self.chars.clone();
        let mut prev = chars.clone();
        let mut ident = None;
        while let Some((_, c)) = chars.next() {
            match c {
                '0'...'9' | 'a'...'z' | 'A'...'Z' => {
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
            match None.or_else(|| self.lex_bf()).or_else(|| self.lex_ident()) {
                t @ Some(_) => return t,
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
}
