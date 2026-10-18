#[derive(Debug, Clone, PartialEq)]
pub struct If {
    pub test: Box<crate::ast::expr::Expr>,
    pub body: Vec<crate::ast::stmt::Stmt>,
    pub orelse: Vec<crate::ast::stmt::Stmt>,
    pub lineno: Option<i64>,
    pub col_offset: Option<i64>,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
