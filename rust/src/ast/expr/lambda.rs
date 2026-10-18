#[derive(Debug, Clone, PartialEq)]
pub struct Lambda {
    pub args: crate::ast::helper::Arguments,
    pub body: Box<crate::ast::expr::Expr>,
    pub lineno: Option<i64>,
    pub col_offset: Option<i64>,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
