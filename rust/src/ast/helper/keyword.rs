#[derive(Debug, Clone, PartialEq)]
pub struct Keyword {
    pub arg: Option<String>,
    pub value: Box<crate::ast::expr::Expr>,
    pub lineno: Option<i64>,
    pub col_offset: Option<i64>,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
