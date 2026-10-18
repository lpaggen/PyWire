#[derive(Debug, Clone, PartialEq)]
pub struct AsyncWith {
    pub items: Vec<crate::ast::helper::WithItem>,
    pub body: Vec<crate::ast::stmt::Stmt>,
    pub type_comment: Option<String>,
    pub lineno: Option<i64>,
    pub col_offset: Option<i64>,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
