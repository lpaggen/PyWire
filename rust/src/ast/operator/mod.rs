pub mod add;
pub mod and;
pub mod bitand;
pub mod bitor;
pub mod bitxor;
pub mod del;
pub mod div;
pub mod eq;
pub mod floordiv;
pub mod gt;
pub mod gte;
pub mod r#in;
pub mod invert;
pub mod is;
pub mod isnot;
pub mod load;
pub mod lshift;
pub mod lt;
pub mod lte;
pub mod matmult;
pub mod mod_;
pub mod mult;
pub mod not;
pub mod noteq;
pub mod notin;
pub mod or;
pub mod pow;
pub mod rshift;
pub mod store;
pub mod sub;
pub mod uadd;
pub mod usub;

pub use add::Add;
pub use and::And;
pub use bitand::BitAnd;
pub use bitor::BitOr;
pub use bitxor::BitXor;
pub use del::Del;
pub use div::Div;
pub use eq::Eq;
pub use floordiv::FloorDiv;
pub use gt::Gt;
pub use gte::GtE;
pub use r#in::In;
pub use invert::Invert;
pub use is::Is;
pub use isnot::IsNot;
pub use load::Load;
pub use lshift::LShift;
pub use lt::Lt;
pub use lte::LtE;
pub use matmult::MatMult;
pub use mod_::Mod;
pub use mult::Mult;
pub use not::Not;
pub use noteq::NotEq;
pub use notin::NotIn;
pub use or::Or;
pub use pow::Pow;
pub use rshift::RShift;
pub use store::Store;
pub use sub::Sub;
pub use uadd::UAdd;
pub use usub::USub;

#[derive(Debug, Clone, PartialEq)]
pub enum BoolOp {
    And(And),
    Or(Or),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add(Add),
    Sub(Sub),
    Mult(Mult),
    MatMult(MatMult),
    Div(Div),
    Mod(Mod),
    Pow(Pow),
    LShift(LShift),
    RShift(RShift),
    BitOr(BitOr),
    BitXor(BitXor),
    BitAnd(BitAnd),
    FloorDiv(FloorDiv),
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Invert(Invert),
    Not(Not),
    UAdd(UAdd),
    USub(USub),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CmpOp {
    Eq(Eq),
    NotEq(NotEq),
    Lt(Lt),
    LtE(LtE),
    Gt(Gt),
    GtE(GtE),
    Is(Is),
    IsNot(IsNot),
    In(In),
    NotIn(NotIn),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprContext {
    Load(Load),
    Store(Store),
    Del(Del),
}
