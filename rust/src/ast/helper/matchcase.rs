#[derive(Debug, Clone, PartialEq)]
pub struct MatchCase {
    pub pattern: Box<crate::ast::pattern::Pattern>,
    pub guard: Option<Box<crate::ast::expr::Expr>>,
    pub body: Vec<crate::ast::stmt::Stmt>,
}
