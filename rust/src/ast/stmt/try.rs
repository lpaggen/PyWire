#[derive(Debug, Clone, PartialEq)]
pub struct Try {
    pub body: Vec<crate::ast::stmt::Stmt>,
    pub handlers: Vec<crate::ast::excepthandler::ExceptHandlerNode>,
    pub orelse: Vec<crate::ast::stmt::Stmt>,
    pub finalbody: Vec<crate::ast::stmt::Stmt>,
    pub lineno: Option<i64>,
    pub col_offset: Option<i64>,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
