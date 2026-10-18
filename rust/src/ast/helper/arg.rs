#[derive(Debug, Clone, PartialEq)]
pub struct Arg {
    pub arg: String,
    pub annotation: Option<Box<crate::ast::expr::Expr>>,
    pub type_comment: Option<String>,
    pub lineno: Option<i64>,
    pub col_offset: Option<i64>,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
