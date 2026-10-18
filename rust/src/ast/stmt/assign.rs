#[derive(Debug, Clone, PartialEq)]
pub struct Assign {
    pub targets: Vec<crate::ast::expr::Expr>,
    pub value: Box<crate::ast::expr::Expr>,
    pub type_comment: Option<String>,
    pub lineno: Option<i64>,
    pub col_offset: Option<i64>,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
