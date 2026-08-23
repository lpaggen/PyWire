use crate::ir::nodes::{match_ir::MatchCaseIR, *};

#[derive(Debug, Clone)]
pub enum StmtIR {
    ExprStmt(ExprStmtIR),
    Binding(BindingIR),
    AugAssign(AugAssignIR),
    If(IfIR),
    WhileLoop(WhileLoopIR),
    ForLoop(ForLoopIR),
    Function(FunctionIR),
    Class(ClassIR),
    Import(ImportIR),
    Return(ReturnIR),
    Match(MatchIR),
}
