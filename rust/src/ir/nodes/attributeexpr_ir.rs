use crate::ir::{expr_ir::ExprIR, span_ir::SourceSpan};

#[derive(Debug, Clone)]
pub struct AttributeExprIR {
    pub target: Box<ExprIR>,
    pub attr: String,
    pub span: Option<SourceSpan>,
}
