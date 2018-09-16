use bf;
use source;
use std::fmt;

#[derive(PartialEq, Clone)]
pub enum Token<'src> {
    Linebreak {
        span: source::Span<'src>,
        newline: bool, // false if the token is ';'
    },
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
    pub fn span(&self) -> &'src source::Span {
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

impl<'s> fmt::Debug for Token<'s> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({}..{})", self, self.span().start, self.span().end())
    }
}

impl bf::Op {
    pub fn token<'s>(self, span: source::Span<'s>) -> source::Token<'s> {
        source::Token::Bf { span, op: self }
    }
}
