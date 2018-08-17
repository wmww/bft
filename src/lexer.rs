use source;
use bf;

pub enum Token<'src> {
    Linebreak{span: source::Span<'src>, newline: bool}, // if its a `;`, newline is false
    Ident{span: source::Span<'src>, value: String},
    String{span: source::Span<'src>, value: String},
    OpenBrace(source::Span<'src>),
    CloseBrace(source::Span<'src>),
    Colon(source::Span<'src>),
    Bf{span: source::Span<'src>, op: bf::Op},
}

impl<'src> Token<'src> {
    fn span(&self) -> &'src source::Span {
        match self {
            Token::Linebreak{span: span, newline: _} => span,
            Token::Ident{span: span, value: _} => span,
            Token::String{span: span, value: _} => span,
            Token::OpenBrace(span) => span,
            Token::CloseBrace(span) => span,
            Token::Colon(span) => span,
            Token::Bf{span: span, op: _} => span,
        }
    }
}

pub struct Seq<'src> {
    pub tokens: Vec<Token<'src>>,
    pub file: &'src source::File,
}

impl<'src> Seq<'src> {
    pub fn new(file: &source::File) -> Seq {
        Seq{tokens: Vec::new(), file: file}
    }
}
