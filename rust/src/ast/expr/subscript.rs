#[derive(Debug, Clone, PartialEq)]
pub struct Subscript {
    pub value: Box<crate::ast::expr::Expr>,
    pub slice: Box<crate::ast::expr::Expr>,
    pub ctx: crate::ast::operator::ExprContext,
    pub lineno: Option<i64>,
    pub col_offset: Option<i64>,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
