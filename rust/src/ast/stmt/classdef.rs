#[derive(Debug, Clone, PartialEq)]
pub struct ClassDef {
    pub name: String,
    pub bases: Vec<crate::ast::expr::Expr>,
    pub keywords: Vec<crate::ast::helper::Keyword>,
    pub body: Vec<crate::ast::stmt::Stmt>,
    pub decorator_list: Vec<crate::ast::expr::Expr>,
    pub type_params: Vec<crate::ast::type_param::TypeParam>,
    pub lineno: Option<i64>,
    pub col_offset: Option<i64>,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
