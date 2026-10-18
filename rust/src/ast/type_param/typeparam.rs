#[derive(Debug, Clone, PartialEq)]
pub enum TypeParam {
    TypeVar(crate::ast::type_param::TypeVar),
    ParamSpec(crate::ast::type_param::ParamSpec),
    TypeVarTuple(crate::ast::type_param::TypeVarTuple),
}
