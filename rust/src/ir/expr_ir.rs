use crate::ir::nodes::{AttributeExprIR, BinOpIR, BoolOpIR, BooleanIR, CallExprIR, CompareIR, DictIR, EllipsisIR, FloatIR, IdentifierIR, IfExprIR, IntegerIR, ListIR, NoneIR, SetIR, SliceIR, StringIR, SubscriptIR, TupleIR, UnaryOpIR};

#[derive(Debug, Clone)]
pub enum ExprIR {
    IdentifierExpr(IdentifierIR),
    IntegerExpr(IntegerIR),
    FloatExpr(FloatIR),
    BoolExpr(BooleanIR),
    StringExpr(StringIR),
    NoneExpr(NoneIR),

    ListExpr(ListIR),
    TupleExpr(TupleIR),
    SliceExpr(SliceIR),
    SubscriptExpr(SubscriptIR),
    AttributeExpr(AttributeExprIR),
    SetExpr(SetIR),
    DictExpr(DictIR),

    BinOpExpr(BinOpIR),
    BoolOpExpr(BoolOpIR),
    UnaryOpExpr(UnaryOpIR),
    CompareExpr(CompareIR),
    CallExpr(CallExprIR),

    IfExpr(IfExprIR),

    EllipsisExpr(EllipsisIR),
}
