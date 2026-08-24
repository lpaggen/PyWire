use crate::ir::nodes::{
    AttributeIR, AwaitIR, BinOpIR, BoolOpIR, BooleanIR, BytesIR, CallIR, CompareIR, ComplexIR,
    DictCompIR, DictIR, EllipsisIR, FloatIR, FormattedValueIR, GeneratorExpIR, IfExpIR, IntegerIR,
    InterpolationIR, JoinedStrIR, ListCompIR, ListIR, NameIR, NamedExprIR, NoneIR, SetCompIR,
    SetIR, SliceIR, StarredIR, StringIR, SubscriptIR, TemplateStrIR, TupleIR, UnaryOpIR,
    YieldFromIR, YieldIR,
};

#[derive(Debug, Clone)]
pub enum ExprIR {
    Constant(ConstantIR),
    Name(NameIR),
    JoinedStr(JoinedStrIR),
    FormattedValue(FormattedValueIR),
    TemplateStr(TemplateStrIR),

    ListExpr(ListIR),
    TupleExpr(TupleIR),
    SliceExpr(SliceIR),
    SubscriptExpr(SubscriptIR),
    Attribute(AttributeIR),
    SetExpr(SetIR),
    DictExpr(DictIR),

    AwaitExpr(AwaitIR),
    YieldExpr(YieldIR),
    YieldFromExpr(YieldFromIR),
    InterpolationExpr(InterpolationIR),

    BinOpExpr(BinOpIR),
    BoolOpExpr(BoolOpIR),
    UnaryOpExpr(UnaryOpIR),
    CompareExpr(CompareIR),
    Call(CallIR),
    NamedExpr(NamedExprIR), // := , named NamedExpr in Python for some reason

    IfExp(IfExpIR),

    GeneratorExp(GeneratorExpIR),
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
