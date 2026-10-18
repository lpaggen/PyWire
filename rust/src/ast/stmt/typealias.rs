#[derive(Debug, Clone, PartialEq)]
pub struct TypeAlias {
    pub name: Box<crate::ast::expr::Expr>,
    pub type_params: Vec<crate::ast::type_param::TypeParam>,
    pub value: Box<crate::ast::expr::Expr>,
    pub lineno: Option<i64>,
    pub col_offset: Option<i64>,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
