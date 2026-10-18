#[derive(Debug, Clone, PartialEq)]
pub struct Interpolation {
    pub value: Box<crate::ast::expr::Expr>,
    pub str: Option<String>,
    pub conversion: i32,
    pub format_spec: Option<Box<crate::ast::expr::Expr>>,
    pub lineno: Option<i64>,
    pub col_offset: Option<i64>,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
