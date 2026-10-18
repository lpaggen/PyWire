#[derive(Debug, Clone, PartialEq)]
pub struct Name {
    pub id: String,
    pub ctx: crate::ast::operator::ExprContext,
    pub lineno: Option<i64>,
    pub col_offset: Option<i64>,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
