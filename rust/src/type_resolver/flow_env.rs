/// flow env is a procedural, temporary struct used in the type resolver
/// it serves to update symbol types dynamically, which can help us resolve
/// conditional branches

use std::collections::HashMap;

use crate::{linker::symbol_ref::SymbolRef, types::types::Type};

#[derive(Debug, Clone)]
pub struct FlowEnv {
    pub by_ref: HashMap<SymbolRef, FlowValue>,
}

#[derive(Debug, Clone, PartialEq)]
enum FlowValue {
    Bound(Type),
    MaybeUnbound(Type),
    Unbound,
}
