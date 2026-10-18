#[derive(Debug, Clone, PartialEq)]
pub struct Await {
    pub value: Box<crate::ast::expr::Expr>,
    pub lineno: Option<i64>,
    pub col_offset: Option<i64>,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
