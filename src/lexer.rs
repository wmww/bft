use bf;
use source;
use std;
use std::fmt;

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

impl<'s> fmt::Display for Token<'s> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Linebreak { span: _, newline } => {
                write!(f, "{}", if *newline { "\\n" } else { ";" })
            }
            Token::Ident { span: _, value } => write!(f, "${}", value),
            Token::String { span: _, value } => {
                write!(f, "\"{}\"", value.chars().map(|c| c).collect::<String>())
            }
            Token::OpenBrace(_) => write!(f, "{{"),
            Token::CloseBrace(_) => write!(f, "}}"),
            Token::Colon(_) => write!(f, ":"),
            Token::Bf { span: _, op } => write!(f, "{}", op),
        }
    }
}

pub struct Tokens<'s> {
    source: &'s source::File,
    chars: std::str::CharIndices<'s>,
    offset: usize,
}

impl<'s> Iterator for Tokens<'s> {
    type Item = Token<'s>;

    fn next(&mut self) -> Option<Token<'s>> {
        let (o, c) = self.chars.next()?;
        match c {
            '\n' => Some(Token::Linebreak {
                span: source::Span::new(self.source, self.offset, o - self.offset),
                newline: true,
            }),
            '0'...'9' | 'a'...'z' | 'A'...'Z' => Some(Token::Ident {
                span: source::Span::new(self.source, self.offset, o - self.offset),
                value: c.to_string(),
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

impl<'s> fmt::Display for Seq<'s> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.tokens.iter().fold(String::new(), |s, i| format!(
                "{}{}{}",
                s,
                if s.is_empty() { "" } else { " " },
                i
            ))
        )
    }
}
