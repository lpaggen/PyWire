#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub body: Box<crate::ast::expr::Expr>,
}
