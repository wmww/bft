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
