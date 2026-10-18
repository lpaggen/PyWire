#[derive(Debug, Clone, PartialEq)]
pub struct Compare {
    pub left: Box<crate::ast::expr::Expr>,
    pub ops: Vec<crate::ast::operator::CmpOp>,
    pub comparators: Vec<crate::ast::expr::Expr>,
    pub lineno: Option<i64>,
    pub col_offset: Option<i64>,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
