#[derive(Debug, Clone, PartialEq)]
pub struct AsyncFor {
    pub target: Box<crate::ast::expr::Expr>,
    pub iter: Box<crate::ast::expr::Expr>,
    pub body: Vec<crate::ast::stmt::Stmt>,
    pub orelse: Vec<crate::ast::stmt::Stmt>,
    pub type_comment: Option<String>,
    pub lineno: Option<i64>,
    pub col_offset: Option<i64>,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
