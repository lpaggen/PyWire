use crate::ir::nodes::{BooleanIR, FloatIR, IdentifierIR, IntegerIR, ListIR, NoneIR, SliceIR, StringIR, TupleIR, SubscriptIR, AttributeExprIR, BinOpIR, BoolOpIR, UnaryOpIR, CompareIR, CallExprIR};

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

    BinOpExpr(BinOpIR),
    BoolOpExpr(BoolOpIR),
    UnaryOpExpr(UnaryOpIR),
    CompareExpr(CompareIR),
    CallExpr(CallExprIR),
}
