use crate::ir::nodes::{
    binding_ir::BindingIR, classdef_ir::ClassDefIR, functiondef_ir::FunctionDefIR,
};

#[derive(Debug, Clone)]
pub enum DeclIR {
    Binding(BindingIR),
    Function(FunctionDefIR),
    Class(ClassDefIR),
}
