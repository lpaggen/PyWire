#[derive(Debug, Clone, PartialEq)]
pub struct ListComp {
    pub elt: Box<crate::ast::expr::Expr>,
    pub generators: Vec<crate::ast::helper::Comprehension>,
    pub lineno: Option<i64>,
    pub col_offset: Option<i64>,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
