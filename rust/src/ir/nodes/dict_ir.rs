use crate::ir::expr_ir::ExprIR;
use crate::ir::span_ir::SourceSpan;

#[derive(Debug, Clone)]
pub struct DictIR {
    pub elements: Vec<DictEntryIR>,
    pub span: Option<SourceSpan>,
}

#[derive(Debug, Clone)]
pub struct DictEntryIR {
    pub key: Option<ExprIR>, // {**something} is allowed
    pub value: ExprIR,
    pub span: Option<SourceSpan>,
}
