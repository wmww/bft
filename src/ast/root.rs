use super::Section;

#[derive(PartialEq)]
pub struct Root {
    ast: Vec<Section>,
}

impl Root {
    pub fn empty() -> Root {
        Root { ast: vec![] }
    }

    pub fn new(ast: Vec<Section>) -> Root {
        Root { ast }
    }
}

impl ::source::Parsable<()> for Root {
    fn parse(p: &mut ::source::Parser, _: ()) -> ::source::ParseResult<Self> {
        Ok(Self::new(p.parse(())?))
    }
}

impl std::ops::Deref for Root {
    type Target = Vec<Section>;

    fn deref(&self) -> &Vec<Section> {
        &self.ast
    }
}

impl std::fmt::Debug for Root {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "AST: {:#?}", self.ast)
    }
}

pub fn parse(file: std::rc::Rc<::source::File>) -> Root {
    let mut p = ::source::Parser::new(file);
    match p.parse(()) {
        Ok(v) => v,
        Err(issue) => {
            match issue {
                Some(issue) => println!("Issue: {}", issue),
                None => println!("Parse failed"),
            }
            Root::empty()
        }
    }
}
