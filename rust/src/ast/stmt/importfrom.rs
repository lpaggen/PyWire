#[derive(Debug, Clone, PartialEq)]
pub struct ImportFrom {
    pub module: Option<String>,
    pub names: Vec<crate::ast::helper::Alias>,
    pub level: Option<i32>,
    pub lineno: Option<i64>,
    pub col_offset: Option<i64>,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
