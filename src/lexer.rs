use bf;
use source;
use std;

#[derive(Debug)]
pub enum Token<'src> {
    Linebreak {
        span: source::Span<'src>,
        newline: bool,
    }, // if its a `;`, newline is false
    Ident {
        span: source::Span<'src>,
        value: String,
    },
    String {
        span: source::Span<'src>,
        value: String,
    },
    OpenBrace(source::Span<'src>),
    CloseBrace(source::Span<'src>),
    Colon(source::Span<'src>),
    Bf {
        span: source::Span<'src>,
        op: bf::Op,
    },
}

impl<'src> Token<'src> {
    fn next(file: &source::File, mut iter: std::str::Chars) -> Option<Token<'src>> {
        match iter.next().unwrap_or('\0') {
            '\0' => None,
            c => {
                println!("{}", c);
                None
            }
        }
    }

    fn span(&self) -> &'src source::Span {
        match self {
            Token::Linebreak { span, newline: _ } => span,
            Token::Ident { span, value: _ } => span,
            Token::String { span, value: _ } => span,
            Token::OpenBrace(span) => span,
            Token::CloseBrace(span) => span,
            Token::Colon(span) => span,
            Token::Bf { span, op: _ } => span,
        }
    }
}

pub struct Tokens<'src> {
    span: source::Span<'src>,
    chars: std::str::Chars<'src>,
}

impl<'src> Iterator for Tokens<'src> {
    type Item = Token<'src>;

    fn next(&mut self) -> Option<Token<'src>> {
        match self.chars.next()? {
            '0'...'9' | 'a'...'z' | 'A'...'Z' => Some(Token::OpenBrace(self.span.clone())),
            _ => None,
        }
    }
}

impl<'src> IntoIterator for &'src source::File {
    type Item = Token<'src>;
    type IntoIter = Tokens<'src>;

    fn into_iter(self) -> Tokens<'src> {
        Tokens {
            chars: self.contents.chars(),
            span: source::Span {
                source: self,
                offset: 0,
                length: 0,
                line: 1,
                character: 1,
            },
        }
    }
}

#[derive(Debug)]
pub struct Seq<'src> {
    pub tokens: Vec<Token<'src>>,
    pub file: &'src source::File,
}

impl<'src> Seq<'src> {
    pub fn new(file: &source::File) -> Seq {
        Seq {
            tokens: file.into_iter().collect(),
            file: file,
        }
    }
}
