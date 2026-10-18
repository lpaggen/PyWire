#[derive(Debug, Clone, PartialEq)]
pub struct Match {
    pub subject: Box<crate::ast::expr::Expr>,
    pub cases: Vec<crate::ast::helper::MatchCase>,
    pub lineno: Option<i64>,
    pub col_offset: Option<i64>,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
