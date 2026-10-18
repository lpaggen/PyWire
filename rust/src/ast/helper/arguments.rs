#[derive(Debug, Clone, PartialEq)]
pub struct Arguments {
    pub posonlyargs: Vec<crate::ast::helper::Arg>,
    pub args: Vec<crate::ast::helper::Arg>,
    pub vararg: Option<crate::ast::helper::Arg>,
    pub kwonlyargs: Vec<crate::ast::helper::Arg>,
    pub kw_defaults: Vec<Option<crate::ast::expr::Expr>>,
    pub kwarg: Option<crate::ast::helper::Arg>,
    pub defaults: Vec<crate::ast::expr::Expr>,
}
