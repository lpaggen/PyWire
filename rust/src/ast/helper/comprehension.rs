#[derive(Debug, Clone, PartialEq)]
pub struct Comprehension {
    pub target: Box<crate::ast::expr::Expr>,
    pub iter: Box<crate::ast::expr::Expr>,
    pub ifs: Vec<crate::ast::expr::Expr>,
    pub is_async: i32,
}
