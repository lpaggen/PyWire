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
    global_symbol_table: &'a GlobalSymbolTable,
}

impl<'a> TypeResolver<'a> {
    fn new(
        symbol_types: &'a mut SymbolTypeTable,
        program_table: &'a ProgramTable,
        diagnostics: Vec<Diagnostic>,
        global_symbol_table: &'a GlobalSymbolTable,
    ) -> Self {
        Self {
            symbol_types,
            program_table,
            diagnostics,
            global_symbol_table,
        }
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

    fn infer_div_type(&self, left: &Type, right: &Type) -> Option<Type> {
        match (left, right) {
            (Type::Complex, Type::Bool | Type::Int | Type::Float | Type::Complex)
            | (Type::Bool | Type::Int | Type::Float, Type::Complex) => Some(Type::Complex),

            (Type::Bool | Type::Int | Type::Float, Type::Bool | Type::Int | Type::Float) => {
                Some(Type::Float)
            }

            _ => None,
        }
    }

    fn infer_floor_div_type(&self, left: &Type, right: &Type) -> Option<Type> {
        match (left, right) {
            (Type::Bool | Type::Int, Type::Bool | Type::Int) => Some(Type::Int),
            (Type::Bool | Type::Int | Type::Float, Type::Bool | Type::Int | Type::Float) => {
                Some(Type::Float)
            }
            _ => None,
        }
    }

    fn infer_mod_type(&self, left: &Type, right: &Type) -> Option<Type> {
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

    fn infer_tensor_binary_type(&self, op: Operator, left: &Type, right: &Type) -> Option<Type> {
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

    fn infer_matmul_type(&mut self, left: &Type, right: &Type, span: Option<SourceSpan>) -> Type {
        let (Type::Tensor(left_state), Type::Tensor(right_state)) = (left, right) else {
            return self.invalid_binary_operation(Operator::MatMult, left, right, span);
        };



        todo!()

        // if let (TensorTypeState::Resolved(left_tensor), TensorTypeState::Resolved(right_tensor)) =
        //     (left_state, right_state)
        // {
        //     if left_tensor.shape.is_empty() || right_tensor.shape.is_empty() {
        //         self.diagnostics.push(Diagnostic {
        //             severity: Severity::ERROR,
        //             span,
        //             kind: DiagnosticKind::ShapeError,
        //             message: "Matrix multiplication requires tensors with at least one dimension."
        //                 .into(),
        //         });
        //         return Type::Unknown;
        //     }
        //     let left_inner = left_tensor.shape.last().expect("non-empty shape");
        //     let right_inner = if right_tensor.shape.len() == 1 {
        //         &right_tensor.shape[0]
        //     } else {
        //         &right_tensor.shape[right_tensor.shape.len() - 2]
        //     };
        //     if let (DimType::Known(left_dim), DimType::Known(right_dim)) = (left_inner, right_inner)
        //     {
        //         if left_dim != right_dim {
        //             self.diagnostics.push(Diagnostic {
        //                 severity: Severity::ERROR,
        //                 span,
        //                 kind: DiagnosticKind::ShapeError,
        //                 message: format!(
        //                     "Matrix multiplication dimension mismatch: `{left_dim}` and `{right_dim}`."
        //                 ),
        //             });
        //             return Type::Unknown;
        //         }
        //     }
        // }

        // // The exact shape also requires batch broadcasting, which is handled later.
        // Type::Tensor(TensorTypeState::Unresolved)
    }

    /// Resolve a Python binary operation and report invalid known operand pairs.
    fn infer_binary_type(
        &mut self,
        op: Operator,
        left: Type,
        right: Type,
        span: Option<SourceSpan>,
    ) -> Type {
        if op == Operator::MatMult {
            return self.infer_matmul_type(&left, &right, span);
        }
        if let Some(result) = self.infer_tensor_binary_type(op, &left, &right) {
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
            Operator::Div => self.infer_div_type(&left, &right),
            Operator::FloorDiv => self.infer_floor_div_type(&left, &right),
            Operator::Mod => self.infer_mod_type(&left, &right),
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

    fn infer_call_type(&self, call: &CallExprIR, scope_id: i64) -> Type {
        todo!()
    }

    // TODO give it ScopeID too, this is extremely important
    fn infer_expr_type(&mut self, expr: &ExprIR, scope_id: i64, program_id: i64) -> Type {
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
                    .map(|element| self.infer_expr_type(element, scope_id, program_id))
                    .collect();

                Type::List(element_types)
            }

            ExprIR::TupleExpr(tuple) => {
                let element_types = tuple
                    .elements
                    .iter()
                    .map(|element: &ExprIR| self.infer_expr_type(element, scope_id, program_id))
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
                let Some(symbol) =
                    self.resolve_name(name.use_scope_id, program_id, &name.name)
                else {
                    return Type::Unknown;
                };

                self.symbol_types
                    .by_ref
                    .get(&symbol)
                    .cloned()
                    .unwrap_or(Type::Unknown)
            }

            ExprIR::CallExpr(call) => self.infer_call_type(call, scope_id),

            ExprIR::BinOpExpr(binary) => {
                let lhs = self.infer_expr_type(&binary.left, scope_id, program_id);
                let rhs = self.infer_expr_type(&binary.right, scope_id, program_id);

                self.infer_binary_type(binary.op, lhs, rhs, binary.span.clone())
            }

            _ => Type::Unknown,
        }
    }

    // walk self.scopes (TODO add this as borrow in struct)
    fn resolve_name(
        &self,
        mut scope_id: i64,
        program_id: i64,
        name: &str,
    ) -> Option<SymbolRef> {
        let program = self.program_table.by_id.get(&program_id)?;
        loop {
            if let Some(symbol) = program
                .symbols
                .iter()
                .find(|symbol| symbol.scope_id == scope_id && symbol.name == name)
            {
                return Some(SymbolRef {
                    program_id,
                    symbol_id: symbol.id,
                });
            }

            let scope = program
                .scopes.iter()
                .find(|s| s.id == scope_id)?;

            scope_id = scope.parent_id?;
        }
    }

    fn infer_assign_type(&mut self, binding_ir: &BindingIR, program_id: i64) -> Type {
        let symbol_ref = SymbolRef {
            program_id,
            symbol_id: binding_ir.target_id,
        };

        let value = match &binding_ir.value {
            Some(value) => self.infer_expr_type(value, binding_ir.scope_id, program_id),
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

    pub fn infer_annotated_assign_type(
        &mut self,
        binding_ir: &BindingIR,
        program_id: i64,
    ) -> Type {
        let symbol_ref = SymbolRef {
            program_id,
            symbol_id: binding_ir.target_id,
        };

        // get the existing type, resolved via annotation only, can be Unknown otherwise
        let annotation_type = self
            .symbol_types
            .by_ref
            .get(&symbol_ref)
            .cloned()
            .unwrap_or(Type::Unknown);

        let value = match &binding_ir.value {
            Some(value) => self.infer_expr_type(value, binding_ir.scope_id, program_id),
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
    pub fn infer_program_types(
        &mut self,
        resolutions: &ResolutionTable,
        programs: &ProgramTable,
    ) {
        // TODO need to resolve the actual RHS of each decl
        // meaning we need access to scopes too, since RHS may be in specific local scopes etc, can only resolve if outer scope <= own scope
        for (&program_id, program) in &programs.by_id {
            for decl in &program.decls {
                let final_type = match decl {
                    DeclIR::Binding(binding_ir) => match &binding_ir.kind {
                        BindingKind::Assign => self.infer_assign_type(binding_ir, program_id),

                        BindingKind::AnnAssign => {
                            self.infer_annotated_assign_type(binding_ir, program_id)
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
