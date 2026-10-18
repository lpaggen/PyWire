#[derive(Debug, Clone, PartialEq)]
pub struct Slice {
    pub lower: Option<Box<crate::ast::expr::Expr>>,
    pub upper: Option<Box<crate::ast::expr::Expr>>,
    pub step: Option<Box<crate::ast::expr::Expr>>,
    pub lineno: Option<i64>,
    pub col_offset: Option<i64>,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
