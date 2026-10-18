#[derive(Debug, Clone, PartialEq)]
pub struct MatchMapping {
    pub keys: Vec<crate::ast::expr::Expr>,
    pub patterns: Vec<crate::ast::pattern::Pattern>,
    pub rest: Option<String>,
    pub lineno: Option<i64>,
    pub col_offset: Option<i64>,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
