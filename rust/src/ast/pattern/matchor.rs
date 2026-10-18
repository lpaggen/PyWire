#[derive(Debug, Clone, PartialEq)]
pub struct MatchOr {
    pub patterns: Vec<crate::ast::pattern::Pattern>,
    pub lineno: Option<i64>,
    pub col_offset: Option<i64>,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
