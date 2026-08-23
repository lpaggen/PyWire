use crate::ir::nodes::{AttributeExprIR, BinOpIR, BoolOpIR, BooleanIR, BytesIR, CallExprIR, CompareIR, ComplexIR, DictCompIR, DictIR, EllipsisIR, FloatIR, FormattedValueIR, GeneratorExprIR, IdentifierIR, IfExprIR, IntegerIR, JoinedStrIR, ListCompIR, ListIR, NamedExprIR, NoneIR, SetCompIR, SetIR, SliceIR, StarredIR, StringIR, SubscriptIR, TupleIR, UnaryOpIR};

#[derive(Debug, Clone)]
pub enum ExprIR {
    Constant(ConstantIR),
    IdentifierExpr(IdentifierIR),
    JoinedStr(JoinedStrIR),
    FormattedValue(FormattedValueIR),

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
    NamedExpr(NamedExprIR),  // := , named NamedExpr in Python for some reason

    IfExpr(IfExprIR),

    GeneratorExpr(GeneratorExprIR),
    ListComp(ListCompIR),
    DictComp(DictCompIR),
    SetComp(SetCompIR),

    StarredExpr(StarredIR),
}

#[derive(Debug, Clone)]
pub enum ConstantIR {
    IntegerLit(IntegerIR),
    FloatLit(FloatIR),
    StringLit(StringIR),
    BooleanLit(BooleanIR),
    BytesLit(BytesIR),
    ComplexLit(ComplexIR),
    NoneLit(NoneIR),
    EllipsisLit(EllipsisIR),
}
