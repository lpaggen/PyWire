#[derive(Debug, Clone, PartialEq)]
pub struct TypeVar {
    pub name: String,
    pub bound: Option<Box<crate::ast::expr::Expr>>,
    pub default_value: Option<Box<crate::ast::expr::Expr>>,
    pub lineno: Option<i64>,
    pub col_offset: Option<i64>,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
