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
