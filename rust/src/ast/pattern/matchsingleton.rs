#[derive(Debug, Clone, PartialEq)]
pub struct MatchSingleton {
    pub value: crate::ast::ConstantValue,
    pub lineno: Option<i64>,
    pub col_offset: Option<i64>,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
