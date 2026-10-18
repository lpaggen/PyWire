#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub body: Vec<crate::ast::stmt::Stmt>,
    pub type_ignores: Vec<crate::ast::mod_::TypeIgnore>,
}
