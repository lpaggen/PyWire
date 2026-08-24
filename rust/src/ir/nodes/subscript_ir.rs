use crate::ir::{expr_ir::ExprIR, span_ir::SourceSpan};

#[derive(Debug, Clone)]
pub struct SubscriptIR {
    pub target: Box<ExprIR>,
    pub subscript: Box<ExprIR>,
    pub span: Option<SourceSpan>,
}
