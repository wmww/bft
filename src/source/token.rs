use runtime;
use source;
use std::fmt;

#[derive(PartialEq, Clone)]
pub enum Token {
    Linebreak {
        newline: bool, // false if the token is ';'
        span: source::Span,
    },
    Ident(String, source::Span),
    String(String, source::Span),
    OpenBrace(source::Span),
    CloseBrace(source::Span),
    Colon(source::Span),
    Bf(runtime::Op, source::Span),
}

impl Token {
    pub fn span(&self) -> &source::Span {
        match self {
            Token::Linebreak { newline: _, span } => span,
            Token::Ident(_, span) => span,
            Token::String(_, span) => span,
            Token::OpenBrace(span) => span,
            Token::CloseBrace(span) => span,
            Token::Colon(span) => span,
            Token::Bf(_, span) => span,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Linebreak { newline, span: _ } => {
                write!(f, "{}", if *newline { "\\n" } else { ";" })
            }
            Token::Ident(value, _) => write!(f, "${}", value),
            Token::String(value, _) => {
                write!(f, "\"{}\"", value.chars().map(|c| c).collect::<String>())
            }
            Token::OpenBrace(_) => write!(f, "{{"),
            Token::CloseBrace(_) => write!(f, "}}"),
            Token::Colon(_) => write!(f, ":"),
            Token::Bf(op, _) => write!(f, "{}", op),
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({}..{})", self, self.span().start, self.span().end())
    }
}

impl runtime::Op {
    pub fn token<'s>(self, span: source::Span) -> source::Token {
        source::Token::Bf(self, span)
    }
}
