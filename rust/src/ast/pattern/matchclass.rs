#[derive(Debug, Clone, PartialEq)]
pub struct MatchClass {
    pub cls: Box<crate::ast::expr::Expr>,
    pub patterns: Vec<crate::ast::pattern::Pattern>,
    pub kwd_attrs: Vec<String>,
    pub kwd_patterns: Vec<crate::ast::pattern::Pattern>,
    pub lineno: Option<i64>,
    pub col_offset: Option<i64>,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
