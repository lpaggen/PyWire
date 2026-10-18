#[derive(Debug, Clone, PartialEq)]
pub struct BinOp {
    pub left: Box<crate::ast::expr::Expr>,
    pub op: crate::ast::operator::Operator,
    pub right: Box<crate::ast::expr::Expr>,
    pub lineno: Option<i64>,
    pub col_offset: Option<i64>,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
