#[derive(Clone, PartialEq)]
pub struct Parser {
    file: std::rc::Rc<::source::File>,
    byte: usize,
}

impl Parser {
    pub fn new(file: std::rc::Rc<::source::File>) -> Parser {
        Parser { file, byte: 0 }
    }

    pub fn parse<T, U>(&mut self, arg: U) -> ParseResult<T>
    where
        T: Parsable<U>,
    {
        let mut child = self.clone();
        let result = T::parse(&mut child, arg);
        if let Ok(_) = result {
            self.byte = child.byte;
        }
        result
    }

    pub fn try_parse<T, U>(&mut self, arg: U) -> ParseResult<T>
    where
        T: Parsable<U>,
    {
        T::parse(&mut self.clone(), arg)
    }

    pub fn string_between(&self, other: &Parser) -> String {
        assert_eq!(self.file, other.file);
        self.file.contents[self.byte..other.byte].to_string()
    }

    pub fn next_char(&mut self) -> Option<char> {
        let (c, advance_by) = {
            let slice = self.remaining_slice();
            let mut indicies = slice.char_indices();
            let (_, c) = indicies.next()?;
            let advance_by = match indicies.next() {
                Some((i, _)) => i,
                None => slice.len(),
            };
            (c, advance_by)
        };
        self.byte += advance_by;
        Some(c)
    }

    pub fn try_next_char(&mut self) -> Option<char> {
        self.remaining_slice().chars().next()
    }

    pub fn remaining_slice<'a>(&'a self) -> &'a str {
        assert!(self.byte <= self.file.contents.len());
        &self.file.contents[self.byte..]
    }
}

pub type ParseResult<T> = Result<T, Option<::io::Issue>>;

pub trait Parsable<T>
where
    Self: std::marker::Sized,
{
    fn parse(p: &mut Parser, arg: T) -> ParseResult<Self>;
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

impl<T, U> Parsable<U> for Box<T>
where
    T: Parsable<U>,
    U: Clone,
{
    fn parse(p: &mut Parser, arg: U) -> ParseResult<Self> {
        Ok(Box::new(p.parse(arg)?))
    }
}

impl<T, U> Parsable<U> for ::source::Spanned<T>
where
    T: Parsable<U>,
    U: Clone,
{
    fn parse(p: &mut Parser, arg: U) -> ParseResult<Self> {
        let start_byte = p.byte;
        let v = p.parse(arg)?;
        let end_byte = p.byte;
        assert!(start_byte < p.file.contents.len());
        assert!(end_byte <= p.file.contents.len());
        let span = ::source::Span {
            file: p.file.clone(),
            start_byte,
            end_byte,
        };
        Ok(::source::Spanned { s: Some(span), v })
    }
}

impl<T, U> Parsable<U> for Vec<T>
where
    T: Parsable<U>,
    U: Clone,
{
    fn parse(p: &mut Parser, arg: U) -> ParseResult<Self> {
        let mut vec = Vec::new();
        loop {
            vec.push(match p.parse(arg.clone()) {
                Ok(v) => v,
                Err(None) => break,
                Err(Some(issue)) => return Err(Some(issue)),
            })
        }
        if vec.len() > 0 {
            Ok(vec)
        } else {
            println!("Length is 0");
            Err(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use source::span::TestBuilder;

    #[test]
    fn parse_str() {
        let b = TestBuilder::new("abc");
        let mut p = Parser::new(b.file.clone());
        let r = p.parse("abc");
        assert_eq!(r, Ok(()));
    }

    #[test]
    fn parse_str_only_once() {
        let b = TestBuilder::new("abc");
        let mut p = Parser::new(b.file.clone());
        let r = p.parse("abc");
        assert_eq!(r, Ok(()));
        let r = p.parse::<(), &str>("abc");
        assert_eq!(r, Err(None));
    }

    #[test]
    fn try_parse_str() {
        let b = TestBuilder::new("abc");
        let mut p = Parser::new(b.file.clone());
        let r = p.try_parse("abc");
        assert_eq!(r, Ok(()));
        let r = p.try_parse("abc");
        assert_eq!(r, Ok(()));
    }

    #[test]
    fn parse_failed_str() {
        let b = TestBuilder::new("abc");
        let mut p = Parser::new(b.file.clone());
        let r = p.parse::<(), &str>("xyz");
        assert_eq!(r, Err(None));
    }

    #[test]
    fn parse_spanned_str() {
        let mut b = TestBuilder::new("abc");
        let mut p = Parser::new(b.file.clone());
        let r = p.parse("abc");
        assert_eq!(r, Ok(b.span(3).around(())));
    }

    #[test]
    fn parse_failed_spanned_str() {
        let b = TestBuilder::new("abc");
        let mut p = Parser::new(b.file.clone());
        let r = p.parse::<::source::Spanned<()>, &str>("xyz");
        assert_eq!(r, Err(None));
    }

    #[test]
    fn parse_str_vec() {
        let b = TestBuilder::new("abc_abc_abc_abc_");
        let mut p = Parser::new(b.file.clone());
        let r = p.parse("abc_");
        assert_eq!(r, Ok(vec![(), (), (), ()]),);
    }

    #[test]
    fn parse_failed_str_vec() {
        let b = TestBuilder::new("abc_abc_abc_abc_");
        let mut p = Parser::new(b.file.clone());
        let r = p.parse::<Vec<()>, &str>("xyz_");
        assert_eq!(r, Err(None));
    }

    #[test]
    fn parse_partial_str_vec() {
        let b = TestBuilder::new("abc_abc_aXY_abc_");
        let mut p = Parser::new(b.file.clone());
        let r = p.parse("abc_");
        assert_eq!(r, Ok(vec![(), ()]),);
    }

    #[test]
    fn next_char_first() {
        let b = TestBuilder::new("ab");
        let mut p = Parser::new(b.file.clone());
        assert_eq!(p.next_char(), Some('a'));
    }

    #[test]
    fn try_next_char_first() {
        let b = TestBuilder::new("ab");
        let mut p = Parser::new(b.file.clone());
        assert_eq!(p.try_next_char(), Some('a'));
    }

    #[test]
    fn next_char_advances() {
        let b = TestBuilder::new("ab");
        let mut p = Parser::new(b.file.clone());
        assert_eq!(p.next_char(), Some('a'));
        assert_eq!(p.next_char(), Some('b'));
    }

    #[test]
    fn try_next_char_doesnt_advance() {
        let b = TestBuilder::new("ab");
        let mut p = Parser::new(b.file.clone());
        assert_eq!(p.try_next_char(), Some('a'));
        assert_eq!(p.try_next_char(), Some('a'));
    }

    #[test]
    fn next_char_stops_at_end() {
        let b = TestBuilder::new("ab");
        let mut p = Parser::new(b.file.clone());
        assert_eq!(p.next_char(), Some('a'));
        assert_eq!(p.next_char(), Some('b'));
        assert_eq!(p.next_char(), None);
        assert_eq!(p.next_char(), None);
    }

    #[test]
    fn try_next_char_stops_at_end() {
        let b = TestBuilder::new("ab");
        let mut p = Parser::new(b.file.clone());
        assert_eq!(p.next_char(), Some('a'));
        assert_eq!(p.next_char(), Some('b'));
        assert_eq!(p.try_next_char(), None);
    }

    #[test]
    fn next_char_handles_unicode() {
        let b = TestBuilder::new("☺a");
        let mut p = Parser::new(b.file.clone());
        assert_eq!(p.next_char(), Some('☺'));
        assert_eq!(p.next_char(), Some('a'));
        assert_eq!(p.next_char(), None);
    }
}
