#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDef {
    pub name: String,
    pub args: crate::ast::helper::Arguments,
    pub body: Vec<crate::ast::stmt::Stmt>,
    pub decorator_list: Vec<crate::ast::expr::Expr>,
    pub returns: Option<Box<crate::ast::expr::Expr>>,
    pub type_comment: Option<String>,
    pub type_params: Vec<crate::ast::type_param::TypeParam>,
    pub lineno: Option<i64>,
    pub col_offset: Option<i64>,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
