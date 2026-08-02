/// flow env is a procedural, temporary struct used in the type resolver
/// it serves to update symbol types dynamically, which can help us resolve
/// conditional branches
use std::collections::HashMap;

use crate::{linker::symbol_ref::SymbolRef, types::types::Type};

#[derive(Debug, Clone)]
pub struct FlowEnv {
    pub by_ref: HashMap<SymbolRef, FlowValue>,
}

impl FlowEnv {
    pub fn new() -> Self {
        Self {
            by_ref: HashMap::new(),
        }
    }

    pub fn get(&self, k: &SymbolRef) -> FlowValue {
        self.by_ref.get(k).cloned().unwrap_or(FlowValue::Unbound) // incl error ?
    }

    pub fn merge(left: &FlowEnv, right: &FlowEnv) -> FlowEnv {
        let mut merged = FlowEnv::new();

        // Merge every symbol present on the left. A symbol missing from the
        // right-hand branch is unbound on that branch.
        for (symbol, left_value) in &left.by_ref {
            let right_value = right.get(symbol);
            let value = FlowValue::merge_values(left_value, &right_value);

            // Absence is the canonical representation of Unbound, so there is
            // no need to store explicit Unbound entries.
            if value != FlowValue::Unbound {
                merged.by_ref.insert(*symbol, value);
            }
        }

        // Add symbols which only occur on the right. Symbols already handled
        // above must not be merged twice.
        for (symbol, right_value) in &right.by_ref {
            if left.by_ref.contains_key(symbol) {
                continue;
            }

            let value = FlowValue::merge_values(&FlowValue::Unbound, right_value);
            if value != FlowValue::Unbound {
                merged.by_ref.insert(*symbol, value);
            }
        }

        merged
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FlowValue {
    Bound(Type),
    MaybeUnbound(Type),
    Unbound,
}

impl FlowValue {
    fn union_types(left: &Type, right: &Type) -> Type {
        let mut members = Vec::new();

        for ty in [left, right] {
            match ty {
                Type::Union(types) => {
                    for member in types {
                        if !members.contains(member) {
                            members.push(member.clone());
                        }
                    }
                }
                other if !members.contains(other) => members.push(other.clone()),
                _ => {}
            }
        }

        match members.as_slice() {
            [only] => only.clone(),
            _ => Type::Union(members),
        }
    }

    pub fn merge_values(left: &FlowValue, right: &FlowValue) -> FlowValue {
        match (left, right) {
            (FlowValue::Unbound, FlowValue::Unbound) => FlowValue::Unbound,

            (FlowValue::Bound(left), FlowValue::Bound(right)) => {
                FlowValue::Bound(Self::union_types(left, right))
            }

            (FlowValue::Bound(ty), FlowValue::Unbound)
            | (FlowValue::Unbound, FlowValue::Bound(ty)) => FlowValue::MaybeUnbound(ty.clone()),

            (FlowValue::MaybeUnbound(left), FlowValue::Bound(right))
            | (FlowValue::Bound(left), FlowValue::MaybeUnbound(right))
            | (FlowValue::MaybeUnbound(left), FlowValue::MaybeUnbound(right)) => {
                FlowValue::MaybeUnbound(Self::union_types(left, right))
            }

            (FlowValue::MaybeUnbound(ty), FlowValue::Unbound)
            | (FlowValue::Unbound, FlowValue::MaybeUnbound(ty)) => {
                FlowValue::MaybeUnbound(ty.clone())
            }
        }
    }
}