#[derive(Debug, Clone, PartialEq)]
pub struct MatchStar {
    pub name: Option<String>,
    pub lineno: Option<i64>,
    pub col_offset: Option<i64>,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
