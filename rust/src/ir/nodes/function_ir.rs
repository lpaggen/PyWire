use crate::ir::nodes::annotation_ir::AnnotationIR;
use crate::ir::{expr_ir::ExprIR, span_ir::SourceSpan, stmt_ir::StmtIR};

#[derive(Debug, Clone)]
pub enum ParamKind {
    PositionalOnly = 1,
    PositionalOrKeyword = 2,
    VarPositional = 3,
    KeywordOnly = 4,
    VarKeyword = 5,
}

impl TryFrom<i32> for ParamKind {
    type Error = String;

    fn try_from(int: i32) -> Result<Self, Self::Error> {
        match int {
            1 => Ok(ParamKind::PositionalOnly),
            2 => Ok(ParamKind::PositionalOrKeyword),
            3 => Ok(ParamKind::VarPositional),
            4 => Ok(ParamKind::KeywordOnly),
            5 => Ok(ParamKind::VarKeyword),
            _ => Err(format!("invalid ParamKind value: {}", int)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParamIR {
    pub symbol_id: i64,
    pub name: String,
    pub kind: ParamKind,
    pub annotation: Option<AnnotationIR>,
    pub default: Option<Box<ExprIR>>,
    pub span: Option<SourceSpan>,
}

#[derive(Debug, Clone)]
pub struct ReturnIR {
    pub value: Option<Box<ExprIR>>,
    pub span: Option<SourceSpan>,
}

#[derive(Debug, Clone)]
pub struct FunctionIR {
    pub id: i64,
    pub symbol_id: i64,
    pub name: String,
    pub scope_id: i64,
    pub body_scope_id: i64,
    pub params: Vec<ParamIR>,
    pub body: Vec<StmtIR>,
    pub returns: Option<AnnotationIR>,
    pub decorators: Vec<ExprIR>,
    pub span: Option<SourceSpan>,
}
