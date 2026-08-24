use crate::ir::{expr_ir::ExprIR, span_ir::SourceSpan};

#[derive(Debug, Clone)]
pub struct KeywordArgIR {
    pub name: String,
    pub value: Box<ExprIR>,
    pub span: Option<SourceSpan>,
}

#[derive(Debug, Clone)]
pub struct CallExprIR {
    pub callee: Box<ExprIR>,
    pub args: Vec<ExprIR>,
    pub kwargs: Vec<KeywordArgIR>,
    pub span: Option<SourceSpan>,
}
