use runtime::Op;
use source;
use source::span::Span;
use source::token::Token;
use std::rc::Rc;
use std::str::CharIndices;

type TokenIter = Span;

impl TokenIter {
    fn char_indices<'a>(&'a self) -> CharIndices<'a> {
        self.src.contents[self.end_byte..].char_indices()
    }

    fn span_to(&self, mut end: CharIndices) -> Span {
        self.span_to_byte(match end.next() {
            Some((i, _)) => i,
            None => self.src.contents.len(),
        })
    }

    fn lex_bf(&self) -> Option<Token> {
        let mut chars = self.char_indices();
        let (_, c) = chars.next()?;
        let op = Op::new(c)?;
        let span = self.span_to(chars);
        Some(Token::Bf(op, span))
    }

    fn lex_single_char_token(&self) -> Option<Token> {
        let mut chars = self.char_indices();
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

    fn lex_ident(&self) -> Option<Token> {
        let mut chars = self.char_indices();
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
            Token::Ident(ident, span)
        })
    }
}

impl Iterator for TokenIter {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        loop {
            match None
                .or_else(|| self.lex_bf())
                .or_else(|| self.lex_single_char_token())
                .or_else(|| self.lex_ident())
            {
                Some(token) => {
                    *self = token.span().clone();
                    return Some(token);
                }
                None => {
                    let span = {
                        let mut chars = self.char_indices();
                        chars.next()?;
                        self.span_to(chars)
                    };
                    *self = span;
                }
            };
        }
    }
}

pub fn lex(file: Rc<source::File>) -> Vec<Token> {
    Span::at_start_of(file).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use source::span;

    fn ident<'s>(value: &str, span: span::Span) -> Token {
        Token::Ident(value.to_string(), span)
    }

    fn load(source: &str) -> (span::Generator, Vec<Token>) {
        let source = Rc::new(source::File::from_string(source.to_string()));
        let span = span::Generator::new(source.clone());
        let tokens = lex(source);
        (span, tokens)
    }

    #[test]
    fn bf_complex_0() {
        let (mut s, tokens) = load(",[>>+<<-].");
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
        let (mut s, tokens) = load("k");
        assert_eq!(tokens, vec![ident("k", s.span(1))],);
    }

    #[test]
    fn single_ident() {
        let (mut s, tokens) = load("1xY");
        assert_eq!(tokens, vec![ident("1xY", s.span(3))],);
    }

    #[test]
    fn idents_0() {
        let (mut s, tokens) = load("Test of 4  iD3nT1fi3rZ");
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
        let (mut s, tokens) = load(";");
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
        let (mut s, tokens) = load("\n");
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
        let (mut s, tokens) = load("{");
        assert_eq!(tokens, vec![Token::OpenBrace(s.span(1))],);
    }

    #[test]
    fn close_brace() {
        let (mut s, tokens) = load("}");
        assert_eq!(tokens, vec![Token::CloseBrace(s.span(1))],);
    }

    #[test]
    fn colon() {
        let (mut s, tokens) = load(":");
        assert_eq!(tokens, vec![Token::Colon(s.span(1))],);
    }

    #[test]
    fn all_0() {
        let (mut s, tokens) = load("_abc: {{\n    [-]\n}^Xy1;}");
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
                ident("Xy1", s.skip(1).span(3)),
                Token::Linebreak {
                    span: s.span(1),
                    newline: false,
                },
                Token::CloseBrace(s.span(1)),
            ]
        );
    }
}
