use bf;
use source;
use token;
use token::Token;

use std;

pub struct Tokens<'s> {
    source: &'s source::File,
    chars: std::str::CharIndices<'s>,
}

impl<'s> Tokens<'s> {
    fn advance_to(&mut self, chars: std::str::CharIndices<'s>) -> source::Span<'s> {
        let start = match self.chars.clone().next() {Some((i, _)) => i, None => self.source.contents.len() };
        let end = match chars.clone().next() { Some((i, _)) => i, None => self.source.contents.len() };
        assert!(start <= end);
        self.chars = chars;
        source::Span::new(self.source, start, end - start)
    }

    fn lex_bf(&mut self) -> Option<Token<'s>> {
        let mut chars = self.chars.clone();
        let (_, c) = chars.next()?;
        let op = bf::Op::new(c)?;
        let span = self.advance_to(chars);
        Some(Token::Bf{span: span, op: op})
    }
}

impl<'s> Iterator for Tokens<'s> {
    type Item = Token<'s>;

    fn next(&mut self) -> Option<Token<'s>> {
        loop {
            match self.lex_bf() {
                Some(t) => return Some(t),
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
            vec![
                Token::Ident {
                    span: source::Span::new(&source, 0, 1),
                    value: "k".to_string(),
                },
            ],
        );
    }

    #[test]
    fn lex_single_ident() {
        let source = source::File::new("1xY".to_string());
        let tokens = token::Seq::lex(&source);
        assert_eq!(
            tokens.tokens,
            vec![
                Token::Ident {
                    span: source::Span::new(&source, 0, 3),
                    value: "1xY".to_string(),
                },
            ],
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
