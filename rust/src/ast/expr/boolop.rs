#[derive(Debug, Clone, PartialEq)]
pub struct BoolOp {
    pub op: crate::ast::operator::BoolOp,
    pub values: Vec<crate::ast::expr::Expr>,
    pub lineno: Option<i64>,
    pub col_offset: Option<i64>,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
