#[derive(Debug, Clone, PartialEq)]
pub struct Interactive {
    pub body: Vec<crate::ast::stmt::Stmt>,
}
