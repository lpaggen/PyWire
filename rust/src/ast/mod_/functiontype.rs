#[derive(Debug, Clone, PartialEq)]
pub struct FunctionType {
    pub argtypes: Vec<crate::ast::expr::Expr>,
    pub returns: Box<crate::ast::expr::Expr>,
}
