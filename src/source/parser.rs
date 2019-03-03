#[derive(Clone, PartialEq)]
pub struct Parser {
    file: std::rc::Rc<::source::File>,
    byte: usize,
}

impl Parser {
    fn parse<T, U>(&mut self, arg: T) -> ParseResult<::source::Spanned<U>>
    where
        U: Parsable<T>,
    {
        let mut child = self.clone();
        match U::parse(&mut child, arg) {
            Ok(value) => {
                let span = ::source::Span {
                    src: self.file.clone(),
                    start_byte: self.byte,
                    end_byte: child.byte,
                    line: 0,
                    col: 0,
                    width: 0,
                    line_start_byte: self.byte,
                    line_end_byte: child.byte,
                };
                Ok(::source::Spanned { span, value })
            }
            Err(err) => Err(err),
        }
    }
}

type ParseResult<T> = Result<T, Option<::io::Issue>>;

trait Parsable<T>
where
    Self: std::marker::Sized,
{
    fn parse(p: &mut Parser, arg: T) -> ParseResult<Self>;
}

impl Parsable<String> for () {
    fn parse(p: &mut Parser, s: String) -> ParseResult<Self> {
        Ok(p.parse(&s as &str)?.value)
    }
}

impl Parsable<&str> for () {
    fn parse(p: &mut Parser, s: &str) -> ParseResult<Self> {
        if p.file.contents[p.byte..].starts_with(s) {
            p.byte += s.len();
            Ok(())
        } else {
            Err(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parser_from(source: &str) -> Parser {
        Parser {
            file: std::rc::Rc::new(::source::File::from_string(source.to_string())),
            byte: 0,
        }
    }

    #[test]
    fn parse_str() {
        let mut p = parser_from("abc");
        let r = p.parse("abc");
        assert_eq!(
            r,
            Ok(::source::Spanned {
                span: ::source::Span {
                    src: p.file.clone(),
                    start_byte: 0,
                    end_byte: 3,
                    line: 0,
                    col: 0,
                    width: 0,
                    line_start_byte: 0,
                    line_end_byte: 3,
                },
                value: ()
            })
        );
    }
}
