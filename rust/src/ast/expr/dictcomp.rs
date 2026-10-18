#[derive(Debug, Clone, PartialEq)]
pub struct DictComp {
    pub key: Box<crate::ast::expr::Expr>,
    pub value: Box<crate::ast::expr::Expr>,
    pub generators: Vec<crate::ast::helper::Comprehension>,
    pub lineno: Option<i64>,
    pub col_offset: Option<i64>,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
