#[derive(Debug, Clone, PartialEq)]
pub struct Global {
    pub names: Vec<String>,
    pub lineno: Option<i64>,
    pub col_offset: Option<i64>,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
