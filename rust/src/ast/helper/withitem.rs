#[derive(Debug, Clone, PartialEq)]
pub struct WithItem {
    pub context_expr: Box<crate::ast::expr::Expr>,
    pub optional_vars: Option<Box<crate::ast::expr::Expr>>,
}
