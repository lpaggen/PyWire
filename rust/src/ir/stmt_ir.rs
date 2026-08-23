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
    Delete(DeleteIR),
    Assert(AssertIR),
    Raise(RaiseIR),
    AsyncFor(AsyncForIR),
    AsyncFunctionDef(AsyncFunctionDefIR),

    Global(GlobalIR),
    Nonlocal(NonlocalIR),

    Pass(PassIR),
    Break(BreakIR),
    Continue(ContinueIR),

    With(WithIR),
    AsyncWith(AsyncWithIR),

    Try(TryIR),
    TryStar(TryStarIR),

    TypeAlias(TypeAliasIR),
}
