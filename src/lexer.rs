use bf;
use source;
use token;
use token::Token;

use std;

pub struct Tokens<'s> {
    source: &'s source::File,
    chars: std::str::CharIndices<'s>,
    offset: usize,
}

impl<'s> Tokens<'s> {
    fn advance_to(&mut self, new_offset: usize) -> source::Span<'s> {
        let ret = source::Span::new(self.source, self.offset, new_offset - self.offset);
        self.offset += new_offset;
        ret
    }
}

impl<'s> Iterator for Tokens<'s> {
    type Item = Token<'s>;

    fn next(&mut self) -> Option<Token<'s>> {
        let (mut o, c) = self.chars.next()?;
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
    }
}

impl<'src> IntoIterator for &'src source::File {
    type Item = Token<'src>;
    type IntoIter = Tokens<'src>;

    fn into_iter(self) -> Tokens<'src> {
        Tokens {
            source: self,
            chars: self.contents.char_indices(),
            offset: 0,
        }
    }
}
