use crate::{
    diagnostic::diagnostic::{Diagnostic, DiagnosticKind},
    ir::{
        expr_ir::ExprIR,
        nodes::{
            AnnotationIR, BindingIR, CallExprIR, ClassIR, DeclIR, NoneIR, attributeexpr_ir,
            binding_ir::{self, BindingKind},
            boolop_ir, compare_ir, subscript_ir, unaryop_ir,
        },
        operator::Operator,
        span_ir::SourceSpan,
    },
    linker::{
        global_scope_table::GlobalSymbolTable, program_table::ProgramTable,
        resolution_table::ResolutionTable, symbol_ref::SymbolRef,
    },
    types::types::{DimType, TensorTypeState, Type},
};

use crate::diagnostic::diagnostic::Severity;
use crate::type_resolver::symbol_type_table::SymbolTypeTable;

pub struct TypeResolver<'a> {
    pub diagnostics: Vec<Diagnostic>,
    symbol_types: &'a mut SymbolTypeTable, // we udpate it as we go
    program_table: &'a ProgramTable,
}

impl<'a> TypeResolver<'a> {
    fn new(
        symbol_types: &'a mut SymbolTypeTable,
        program_table: &'a ProgramTable,
        diagnostics: Vec<Diagnostic>,
    ) -> Self {
        Self {
            symbol_types,
            program_table,
            diagnostics,
        }
    }

    // walk self.scopes (TODO add this as borrow in struct)
    fn resolve_name(
        &self,
        mut scope_id: i64,
        program_id: i64,
        name: &str,
        use_span: &SourceSpan,
    ) -> Option<SymbolRef> {
        // let program = self.program_table.by_id.get(&program_id)?;  // will always exist
        // loop {
        //     let scope = program  // TODO make "scopes" hashmap for quicker lookup
        //         .scopes.iter()
        //         .find(|s| s.id == scope_id)?;

        //     let symbol = program
        //         .symbols
        //         .iter()
        //         .filter(|symbol| {
        //             let span = &symbol.span.unwrap();
        //             symbol.scope_id == scope_id
        //             && symbol.name == name
        //             && symbol.span.as_ref().line < use_span.line
        //         })
        //         .max_by_key(|symbol| symbol.span.unwrap().line);

        //     scope_id = scope.parent_id?;  // look for symbol in parent
        // }

        // TODO this is near impossible levels of complexity actually, defer to later build

        todo!()
    }

    fn promote_numeric(&self, left: &Type, right: &Type) -> Option<Type> {
        match (left, right) {
            (Type::Complex, Type::Bool | Type::Int | Type::Float | Type::Complex)
            | (Type::Bool | Type::Int | Type::Float, Type::Complex) => Some(Type::Complex),

            (Type::Float, Type::Bool | Type::Int | Type::Float)
            | (Type::Bool | Type::Int, Type::Float) => Some(Type::Float),

            (Type::Bool | Type::Int, Type::Bool | Type::Int) => Some(Type::Int),

            _ => None,
        }
    }

    fn resolve_div(&self, left: &Type, right: &Type) -> Option<Type> {
        match (left, right) {
            (Type::Complex, Type::Bool | Type::Int | Type::Float | Type::Complex)
            | (Type::Bool | Type::Int | Type::Float, Type::Complex) => Some(Type::Complex),

            (Type::Bool | Type::Int | Type::Float, Type::Bool | Type::Int | Type::Float) => {
                Some(Type::Float)
            }

            _ => None,
        }
    }

    fn resolve_floor_div(&self, left: &Type, right: &Type) -> Option<Type> {
        match (left, right) {
            (Type::Bool | Type::Int, Type::Bool | Type::Int) => Some(Type::Int),
            (Type::Bool | Type::Int | Type::Float, Type::Bool | Type::Int | Type::Float) => {
                Some(Type::Float)
            }
            _ => None,
        }
    }

    fn resolve_mod(&self, left: &Type, right: &Type) -> Option<Type> {
        match (left, right) {
            (Type::Bool | Type::Int, Type::Bool | Type::Int) => Some(Type::Int),
            (Type::Bool | Type::Int | Type::Float, Type::Bool | Type::Int | Type::Float) => {
                Some(Type::Float)
            }
            // Python's `%` operator also performs string/bytes formatting.
            (Type::String, _) => Some(Type::String),
            (Type::Bytes, _) => Some(Type::Bytes),
            _ => None,
        }
    }

    fn invalid_binary_operation(
        &mut self,
        op: Operator,
        left: &Type,
        right: &Type,
        span: Option<SourceSpan>,
    ) -> Type {
        // Avoid cascading errors when an operand could not be inferred earlier.
        if *left != Type::Unknown && *right != Type::Unknown {
            self.diagnostics.push(Diagnostic {
                severity: Severity::ERROR,
                span,
                kind: DiagnosticKind::TypeError,
                message: format!(
                    "Unsupported operand types for `{:?}`: `{:?}` and `{:?}`.",
                    op, left, right,
                ),
            });
        }
        Type::Unknown
    }

    fn resolve_tensor_binary(&self, op: Operator, left: &Type, right: &Type) -> Option<Type> {
        let tensor = match (left, right) {
            (Type::Tensor(tensor), Type::Tensor(_)) => tensor,
            (Type::Tensor(tensor), scalar) | (scalar, Type::Tensor(tensor))
                if self.promote_numeric(scalar, scalar).is_some() =>
            {
                tensor
            }
            _ => return None,
        };
        match op {
            Operator::Add
            | Operator::Sub
            | Operator::Mult
            | Operator::Div
            | Operator::FloorDiv
            | Operator::Mod
            | Operator::Pow => Some(Type::Tensor(tensor.clone())),
            _ => None,
        }
    }

    fn resolve_matmul(&mut self, left: &Type, right: &Type, span: Option<SourceSpan>) -> Type {
        let (Type::Tensor(left_state), Type::Tensor(right_state)) = (left, right) else {
            return self.invalid_binary_operation(Operator::MatMult, left, right, span);
        };

        if let (TensorTypeState::Resolved(left_tensor), TensorTypeState::Resolved(right_tensor)) =
            (left_state, right_state)
        {
            if left_tensor.shape.is_empty() || right_tensor.shape.is_empty() {
                self.diagnostics.push(Diagnostic {
                    severity: Severity::ERROR,
                    span,
                    kind: DiagnosticKind::ShapeError,
                    message: "Matrix multiplication requires tensors with at least one dimension."
                        .into(),
                });
                return Type::Unknown;
            }
            let left_inner = left_tensor.shape.last().expect("non-empty shape");
            let right_inner = if right_tensor.shape.len() == 1 {
                &right_tensor.shape[0]
            } else {
                &right_tensor.shape[right_tensor.shape.len() - 2]
            };
            if let (DimType::Known(left_dim), DimType::Known(right_dim)) = (left_inner, right_inner)
            {
                if left_dim != right_dim {
                    self.diagnostics.push(Diagnostic {
                        severity: Severity::ERROR,
                        span,
                        kind: DiagnosticKind::ShapeError,
                        message: format!(
                            "Matrix multiplication dimension mismatch: `{left_dim}` and `{right_dim}`."
                        ),
                    });
                    return Type::Unknown;
                }
            }
        }

        // The exact shape also requires batch broadcasting, which is handled later.
        Type::Tensor(TensorTypeState::Unresolved)
    }

    /// Resolve a Python binary operation and report invalid known operand pairs.
    fn resolve_binary_types(
        &mut self,
        op: Operator,
        left: Type,
        right: Type,
        span: Option<SourceSpan>,
    ) -> Type {
        if op == Operator::MatMult {
            return self.resolve_matmul(&left, &right, span);
        }
        if let Some(result) = self.resolve_tensor_binary(op, &left, &right) {
            return result;
        }

        let result = match op {
            Operator::Add => {
                self.promote_numeric(&left, &right)
                    .or_else(|| match (&left, &right) {
                        (Type::String, Type::String) => Some(Type::String),
                        (Type::Bytes, Type::Bytes) => Some(Type::Bytes),
                        (Type::List(a), Type::List(b)) => {
                            Some(Type::List(a.iter().chain(b).cloned().collect()))
                        }
                        (Type::Tuple(a), Type::Tuple(b)) => {
                            Some(Type::Tuple(a.iter().chain(b).cloned().collect()))
                        }
                        _ => None,
                    })
            }
            Operator::Sub => self.promote_numeric(&left, &right),
            Operator::Mult => {
                self.promote_numeric(&left, &right)
                    .or_else(|| match (&left, &right) {
                        (Type::String, Type::Bool | Type::Int)
                        | (Type::Bool | Type::Int, Type::String) => Some(Type::String),
                        (Type::Bytes, Type::Bool | Type::Int)
                        | (Type::Bool | Type::Int, Type::Bytes) => Some(Type::Bytes),
                        (Type::List(items), Type::Bool | Type::Int)
                        | (Type::Bool | Type::Int, Type::List(items)) => {
                            Some(Type::List(items.clone()))
                        }
                        (Type::Tuple(items), Type::Bool | Type::Int)
                        | (Type::Bool | Type::Int, Type::Tuple(items)) => {
                            Some(Type::Tuple(items.clone()))
                        }
                        _ => None,
                    })
            }
            Operator::Div => self.resolve_div(&left, &right),
            Operator::FloorDiv => self.resolve_floor_div(&left, &right),
            Operator::Mod => self.resolve_mod(&left, &right),
            Operator::Pow => self.promote_numeric(&left, &right),
            Operator::LShift | Operator::RShift => match (&left, &right) {
                (Type::Bool | Type::Int, Type::Bool | Type::Int) => Some(Type::Int),
                _ => None,
            },
            Operator::BitOr | Operator::BitXor | Operator::BitAnd => match (&left, &right) {
                (Type::Bool, Type::Bool) => Some(Type::Bool),
                (Type::Bool | Type::Int, Type::Bool | Type::Int) => Some(Type::Int),
                _ => None,
            },
            // These return one of their operands; without unions, only equal types
            // can be represented precisely.
            Operator::And | Operator::Or => Some(if left == right {
                left.clone()
            } else {
                Type::Unknown
            }),
            Operator::Eq
            | Operator::NotEq
            | Operator::Is
            | Operator::IsNot
            | Operator::In
            | Operator::NotIn => Some(Type::Bool),
            Operator::Lt | Operator::LtE | Operator::Gt | Operator::GtE => {
                if self.promote_numeric(&left, &right).is_some()
                    || matches!(
                        (&left, &right),
                        (Type::String, Type::String) | (Type::Bytes, Type::Bytes)
                    )
                {
                    Some(Type::Bool)
                } else {
                    None
                }
            }
            Operator::Not | Operator::UAdd | Operator::USub | Operator::Unknown(_) => None,
            Operator::MatMult => unreachable!(),
        };

        result.unwrap_or_else(|| self.invalid_binary_operation(op, &left, &right, span))
    }

    fn resolve_call(&self, call: &CallExprIR, scope_id: i64) -> Type {
        todo!()
    }

    // TODO give it ScopeID too, this is extremely important
    fn resolve_expr(&mut self, expr: &ExprIR, scope_id: i64, program_id: i64) -> Type {
        match expr {
            ExprIR::IntegerExpr(_) => Type::Int,
            ExprIR::FloatExpr(_) => Type::Float,
            ExprIR::BoolExpr(_) => Type::Bool,
            ExprIR::StringExpr(_) => Type::String,
            ExprIR::NoneExpr(_) => Type::None,

            ExprIR::ListExpr(list) => {
                let element_types = list
                    .elements
                    .iter()
                    .map(|element| self.resolve_expr(element, scope_id, program_id))
                    .collect();

                Type::List(element_types)
            }

            ExprIR::TupleExpr(tuple) => {
                let element_types = tuple
                    .elements
                    .iter()
                    .map(|element: &ExprIR| self.resolve_expr(element, scope_id, program_id))
                    .collect();

                Type::Tuple(element_types)
            }

            ExprIR::SliceExpr(slice_ir) => {
                todo!()
            }

            ExprIR::SubscriptExpr(subscript_ir) => {
                todo!()
            }

            ExprIR::AttributeExpr(attributeexpr_ir) => {
                todo!()
            }

            ExprIR::BoolOpExpr(boolop_ir) => {
                todo!()
            }

            ExprIR::UnaryOpExpr(unaryop_ir) => {
                todo!()
            }

            ExprIR::CompareExpr(compare_ir) => {
                todo!()
            }

            // variables, example -> x: int = y (i want to check if y is in a parent scope)
            ExprIR::IdentifierExpr(name) => {
                let use_span = match &name.span {
                    Some(span) => span,
                    _other_none => {
                        let message = format!(
                            "Variable {} is missing a span and cannot be resolved",
                            name.name
                        );
                        panic!("{}", message);
                    }
                };

                let Some(symbol) = self.resolve_name(scope_id, program_id, &name.name, &use_span)
                else {
                    return Type::Unknown;
                };

                self.symbol_types
                    .by_ref
                    .get(&symbol)
                    .cloned()
                    .unwrap_or(Type::Unknown)
            }

            ExprIR::CallExpr(call) => self.resolve_call(call, scope_id),

            ExprIR::BinOpExpr(binary) => {
                let lhs = self.resolve_expr(&binary.left, scope_id, program_id);
                let rhs = self.resolve_expr(&binary.right, scope_id, program_id);

                self.resolve_binary_types(binary.op, lhs, rhs, binary.span.clone())
            }

            _ => Type::Unknown,
        }
    }

    fn resolve_assign(&mut self, binding_ir: &BindingIR, program_id: i64) -> Type {
        let symbol_ref = SymbolRef {
            program_id,
            symbol_id: binding_ir.id,
        };

        let value = match &binding_ir.value {
            Some(value) => self.resolve_expr(value, binding_ir.scope_id, program_id),
            _other_none => Type::Unknown, // take existing, this only happens for -> x: int // no value provided
        };

        if value == Type::Unknown {
            // TODO refine, this is mediocre
            self.diagnostics.push(Diagnostic {
                severity: Severity::WARNING,
                span: binding_ir.span.clone(),
                kind: DiagnosticKind::UnknownAssignValue,
                message: format!(
                    "Unable to resolve right hand side of assignment.\n\
                    Inferred initializer type: `{:?}`.\n\
                    Reason: could not infer type of variable {:?}.
                    Suggested fixes:\n\
                    - provide an annotation for variable `{:?}`;\n\
                    - ensure the right hand side has a defined type;",
                    value, binding_ir.value, value,
                ),
            });
        }

        value
    }

    pub fn resolve_annassign(&mut self, binding_ir: &BindingIR, program_id: i64) -> Type {
        let symbol_ref = SymbolRef {
            program_id,
            symbol_id: binding_ir.id,
        };

        // get the existing type, resolved via annotation only, can be Unknown otherwise
        let annotation_type = self
            .symbol_types
            .by_ref
            .get(&symbol_ref)
            .cloned()
            .unwrap_or(Type::Unknown);

        let value = match &binding_ir.value {
            Some(value) => self.resolve_expr(value, binding_ir.scope_id, program_id),
            _other_none => annotation_type.clone(), // take existing, this only happens for -> x: int // no value provided
        };

        if value != annotation_type {
            self.diagnostics.push(Diagnostic {
                severity: Severity::WARNING,
                span: binding_ir.span.clone(),
                kind: DiagnosticKind::MismatchedAnnotationType,
                message: format!(
                    "Type mismatch in annotated assignment.\n\
                    Declared type: `{:?}`.\n\
                    Inferred initializer type: `{:?}`.\n\
                    Reason: a value of type `{:?}` cannot be assigned to a binding annotated as `{:?}`.\n\
                    Suggested fixes:\n\
                    - change the annotation to `{:?}`;\n\
                    - replace the initializer with a value of type `{:?}`;\n\
                    - explicitly convert the initializer to `{:?}`.",
                    annotation_type,
                    value,
                    value,
                    annotation_type,
                    value,
                    annotation_type,
                    annotation_type,
                ),
            });
        }

        value
    }

    /// resolves types on RHS, example -> x: int = 5, match "int" and "5". or -> x = 5 -> go from Type::Unknown to Type::Int
    pub fn resolve_types(&mut self, resolutions: &ResolutionTable, programs: &ProgramTable) {
        // TODO need to resolve the actual RHS of each decl
        // meaning we need access to scopes too, since RHS may be in specific local scopes etc, can only resolve if outer scope <= own scope
        for (&program_id, program) in &programs.by_id {
            for decl in &program.decls {
                let final_type = match decl {
                    DeclIR::Binding(binding_ir) => match &binding_ir.kind {
                        BindingKind::Assign => self.resolve_assign(binding_ir, binding_ir.scope_id),

                        BindingKind::AnnAssign => {
                            self.resolve_annassign(binding_ir, binding_ir.scope_id)
                        }

                        BindingKind::Unknown => {
                            self.diagnostics.push(Diagnostic {
                                severity: Severity::WARNING,
                                span: decl.span().clone(),
                                kind: DiagnosticKind::UnknownBindingKind,
                                message: format!(
                                    "variable {} got an unknown ... TODO",
                                    decl.symbol_id()
                                ),
                            });

                            Type::Unknown
                        }
                    },

                    DeclIR::Class(class_ir) => {
                        todo!()
                    }

                    DeclIR::Function(function_ir) => {
                        todo!()
                    }
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linker::program_table::ProgramTable;

    fn span() -> SourceSpan {
        SourceSpan {
            file: "test.py".into(),
            line: 1,
            col: 0,
            end_line: 1,
            end_col: 10,
        }
    }

    #[test]
    fn applies_python_sequence_rules() {
        let mut symbol_types = SymbolTypeTable::new();
        let programs = ProgramTable::new();
        let mut resolver = TypeResolver::new(&mut symbol_types, &programs, Vec::new());

        assert_eq!(
            resolver.resolve_binary_types(Operator::Mult, Type::String, Type::Int, Some(span())),
            Type::String
        );
        assert_eq!(
            resolver.resolve_binary_types(
                Operator::Add,
                Type::List(vec![Type::Int]),
                Type::List(vec![Type::String]),
                Some(span())
            ),
            Type::List(vec![Type::Int, Type::String])
        );
        assert!(resolver.diagnostics.is_empty());
    }

    #[test]
    fn invalid_known_operands_produce_an_error() {
        let mut symbol_types = SymbolTypeTable::new();
        let programs = ProgramTable::new();
        let mut resolver = TypeResolver::new(&mut symbol_types, &programs, Vec::new());

        let result =
            resolver.resolve_binary_types(Operator::Mult, Type::String, Type::String, Some(span()));

        assert_eq!(result, Type::Unknown);
        assert_eq!(resolver.diagnostics.len(), 1);
        assert!(matches!(resolver.diagnostics[0].severity, Severity::ERROR));
        assert!(matches!(
            resolver.diagnostics[0].kind,
            DiagnosticKind::TypeError
        ));
        assert!(resolver.diagnostics[0].span.is_some());
    }

    #[test]
    fn tensor_matmul_string_produces_an_error() {
        let mut symbol_types = SymbolTypeTable::new();
        let programs = ProgramTable::new();
        let mut resolver = TypeResolver::new(&mut symbol_types, &programs, Vec::new());

        let result = resolver.resolve_binary_types(
            Operator::MatMult,
            Type::Tensor(TensorTypeState::Unresolved),
            Type::String,
            Some(span()),
        );

        assert_eq!(result, Type::Unknown);
        assert_eq!(resolver.diagnostics.len(), 1);
        assert!(matches!(resolver.diagnostics[0].severity, Severity::ERROR));
        assert!(resolver.diagnostics[0].message.contains("MatMult"));
    }

    #[test]
    fn unknown_operands_do_not_cascade_diagnostics() {
        let mut symbol_types = SymbolTypeTable::new();
        let programs = ProgramTable::new();
        let mut resolver = TypeResolver::new(&mut symbol_types, &programs, Vec::new());

        assert_eq!(
            resolver.resolve_binary_types(Operator::Add, Type::Unknown, Type::String, Some(span())),
            Type::Unknown
        );
        assert!(resolver.diagnostics.is_empty());
    }
}
