#[derive(Debug, Clone, PartialEq)]
pub struct ExceptHandler {
    pub r#type: Option<Box<crate::ast::expr::Expr>>,
    pub name: Option<String>,
    pub body: Vec<crate::ast::stmt::Stmt>,
    pub lineno: Option<i64>,
    pub col_offset: Option<i64>,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
