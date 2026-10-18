#[derive(Debug, Clone, PartialEq)]
pub struct AnnAssign {
    pub target: Box<crate::ast::expr::Expr>,
    pub annotation: Box<crate::ast::expr::Expr>,
    pub value: Option<Box<crate::ast::expr::Expr>>,
    pub simple: i32,
    pub lineno: Option<i64>,
    pub col_offset: Option<i64>,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
