use bf;
use source;
use std::str::CharIndices;
use token;
use token::Token;

use std;

pub struct Tokens<'s> {
    source: &'s source::File,
    chars: CharIndices<'s>,
}

impl<'s> Tokens<'s> {
    fn offset_of(&self, i: &CharIndices) -> usize {
        match i.clone().next() {
            Some((i, _)) => i,
            None => self.source.contents.len(),
        }
    }

    fn span_between(&self, start: &CharIndices, end: &CharIndices) -> source::Span<'s> {
        let start = self.offset_of(start);
        let end = self.offset_of(end);
        source::Span::new(self.source, start, end - start)
    }

    fn lex_bf(&self) -> Option<(Token<'s>, CharIndices<'s>)> {
        let mut chars = self.chars.clone();
        let (_, c) = chars.next()?;
        let op = bf::Op::new(c)?;
        let span = self.span_between(&self.chars, &chars);
        Some((Token::Bf { span: span, op: op }, chars))
    }

    fn lex_ident(&self) -> Option<(Token<'s>, CharIndices<'s>)> {
        None
    }
}

impl<'s> Iterator for Tokens<'s> {
    type Item = Token<'s>;

    fn next(&mut self) -> Option<Token<'s>> {
        loop {
            match None.or_else(|| self.lex_bf()).or_else(|| self.lex_ident()) {
                Some((t, i)) => {
                    self.chars = i;
                    return Some(t);
                }
                None => self.chars.next()?,
            };
        }
        /*loop {

            if let Some(t) = {
                let (mut o, c) = self.chars.next()?;
                if let Some(token) = self.lex_bf() {
                    token
                } else {
                    match c {
                        '\n' => Some(Token::Linebreak {
                            span: self.advance_to(o),
                            newline: true,
                        }),
                        ';' => Some(Token::Linebreak {
                            span: self.advance_to(o),
                            newline: false,
                        }),
                        '0'...'9' | 'a'...'z' | 'A'...'Z' => Some({
                            let mut next = self.chars.clone();
                            let mut ident = c.to_string();
                            while let Some((i, c)) = next.next() {
                                match c {
                                    '0'...'9' | 'a'...'z' | 'A'...'Z' => {
                                        ident.push(c);
                                        o = i;
                                        self.chars = next.clone();
                                    }
                                    _ => break,
                                }
                            }
                            Token::Ident {
                                span: self.advance_to(o),
                                value: ident,
                            }
                        }),
                        _ => None,
                    }
                    None
                }
            } {
                return Some(t);
            }
        }*/
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
    fn lex_bf_complex_a() {
        let source = source::File::new(",[>>+<<-].".to_string());
        let tokens = token::Seq::lex(&source);
        assert_eq!(
            tokens.tokens,
            vec![
                Token::Bf {
                    span: source::Span::new(&source, 0, 1),
                    op: bf::Op::Input,
                },
                Token::Bf {
                    span: source::Span::new(&source, 1, 1),
                    op: bf::Op::Open,
                },
                Token::Bf {
                    span: source::Span::new(&source, 2, 1),
                    op: bf::Op::Right,
                },
                Token::Bf {
                    span: source::Span::new(&source, 3, 1),
                    op: bf::Op::Right,
                },
                Token::Bf {
                    span: source::Span::new(&source, 4, 1),
                    op: bf::Op::Plus,
                },
                Token::Bf {
                    span: source::Span::new(&source, 5, 1),
                    op: bf::Op::Left,
                },
                Token::Bf {
                    span: source::Span::new(&source, 6, 1),
                    op: bf::Op::Left,
                },
                Token::Bf {
                    span: source::Span::new(&source, 7, 1),
                    op: bf::Op::Minus,
                },
                Token::Bf {
                    span: source::Span::new(&source, 8, 1),
                    op: bf::Op::Close,
                },
                Token::Bf {
                    span: source::Span::new(&source, 9, 1),
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
                span: source::Span::new(&source, 0, 1),
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
                span: source::Span::new(&source, 0, 3),
                value: "1xY".to_string(),
            }],
        );
    }

    #[test]
    fn lex_idents_a() {
        let source = source::File::new("Test of 1 iD3NT1fi3r".to_string());
        let tokens = token::Seq::lex(&source);
        assert_eq!(
            tokens.tokens,
            vec![
                Token::Ident {
                    span: source::Span::new(&source, 0, 4),
                    value: "Test".to_string(),
                },
                Token::Ident {
                    span: source::Span::new(&source, 5, 2),
                    value: "if".to_string(),
                },
                Token::Ident {
                    span: source::Span::new(&source, 8, 1),
                    value: "1".to_string(),
                },
                Token::Ident {
                    span: source::Span::new(&source, 10, 10),
                    value: "iD3NT1fi3r".to_string(),
                },
            ],
        );
    }
}
