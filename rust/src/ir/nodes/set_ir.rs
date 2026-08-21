use crate::ir::expr_ir::ExprIR;
use crate::ir::span_ir::SourceSpan;

#[derive(Debug, Clone)]
pub struct SetIR {
    pub elements: Vec<ExprIR>,
    pub span: Option<SourceSpan>,
}
