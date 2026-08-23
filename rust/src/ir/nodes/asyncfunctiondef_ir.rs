use crate::ir::{expr_ir::ExprIR, nodes::ParamIR, span_ir::SourceSpan, stmt_ir::StmtIR};

#[derive(Debug, Clone)]
pub struct AsyncFunctionDefIR {
    pub name: String,
    pub args: Vec<ParamIR>,
    pub body: Vec<StmtIR>,
    pub decorators: Vec<ExprIR>,
    pub returns: Option<ExprIR>,
    pub type_comment: Option<String>,
    pub scope_id: u64,
    pub type_params: Vec<ParamIR>,
    pub span: Option<SourceSpan>,
}
