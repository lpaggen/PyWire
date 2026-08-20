// use crate::{diagnostic::diagnostic::{Diagnostic, DiagnosticKind}, ir::{expr_ir::ExprIR, nodes::{BindingIR, CallExprIR, binding_ir::BindingKind, identifier_ir}, operator::Operator, stmt_ir::StmtIR}, linker::{program_table::ProgramTable, symbol_ref::SymbolRef}, pb::expr_ir::Kind::Identifier, type_resolver::{flow_env::{self, FlowEnv, FlowValue}, symbol_type_table::SymbolTypeTable}, types::types::Type};
// use crate::diagnostic::diagnostic::Severity;

// pub struct TypeResolver<'a> {
//     pub diagnostics: Vec<Diagnostic>,
//     programs: &'a ProgramTable,
//     symbol_types: &'a SymbolTypeTable,
// }

// impl<'a> TypeResolver<'a> {
//     pub fn new(programs: &'a ProgramTable, symbol_types: &'a SymbolTypeTable) -> Self {
//         Self { 
//             diagnostics: Vec::new(),
//             programs: programs,
//             symbol_types: symbol_types,
//         }
//     }

//     fn is_assignable(&self, actual: &Type, expected: &Type) -> bool {
//         actual == expected
//             || matches!((actual, expected), (Type::Int, Type::Float))
//             || matches!(actual, Type::Unknown)
//             || matches!(expected, Type::Unknown)
//     }

//     fn infer_binary_type(&self, op: Operator, left: &ExprIR, right: &ExprIR) -> Type {
//         todo!()
//     }

//     fn infer_call_type(&self, call: &CallExprIR) -> Type {
//         todo!()
//     }

//     fn infer_expr_type(&self, expr: &ExprIR) -> Type {
//         match expr {
//             ExprIR::IntegerExpr(_) => Type::Int,
//             ExprIR::FloatExpr(_) => Type::Float,
//             ExprIR::BoolExpr(_) => Type::Bool,
//             ExprIR::StringExpr(_) => Type::String,
//             ExprIR::NoneExpr(_) => Type::None,

//             ExprIR::ListExpr(list) => {
//                 let element_types = list
//                     .elements
//                     .iter()
//                     .map(|element| self.infer_expr_type(element))
//                     .collect();

//                 Type::List(element_types)
//             }

//             ExprIR::TupleExpr(tuple) => {
//                 let element_types = tuple
//                     .elements
//                     .iter()
//                     .map(|element| self.infer_expr_type(element))
//                     .collect();

//                 Type::Tuple(element_types)
//             }

//             ExprIR::CallExpr(call) => {
//                 self.infer_call_type(call)
//             },

//             ExprIR::BinOpExpr(binop_expr) => {
//                 self.infer_binary_type(binop_expr.op, &binop_expr.left, &binop_expr.right)
//             },

//             // name resolution | x: int = a <- we need to find what Type "a" is, is it declared? accessible? unbound?
//             ExprIR::IdentifierExpr(identifier) => {
//                 // let symbol_ref = SymbolRef {
//                 //     program_id: ...,
//                 //     symbol_id: identifier.name.
//                 // }

//                 // need to find by &identifer.name somehow...
//                 // 1) find ref from name
//                 // 2) query flow_env with ref

//                 // self.flow_env.get(k)

//                 Type::Unknown
//             },

//             ExprIR::SliceExpr(slice) => {
//                 todo!()
//             }

//             ExprIR::SubscriptExpr(subscript) => {
//                 todo!()
//             }

//             ExprIR::AttributeExpr(attribute) => {
//                 todo!()
//             }

//             ExprIR::BoolOpExpr(boolean) => {
//                 todo!()
//             }

//             ExprIR::UnaryOpExpr(unary) => {
//                 todo!()
//             }

//             ExprIR::CompareExpr(cmp) => {
//                 todo!()
//             }

//             _ => Type::Unknown,
//         }
//     }

//     fn handle_assign(&mut self, id: i64, binding_ir: &BindingIR, flow_env: &mut FlowEnv) {
//         let target_ref = SymbolRef {
//             program_id: id,
//             symbol_id: binding_ir.target_id
//         };

//         let value_type = match &binding_ir.value {
//             Some(value) => self.infer_expr_type(value),
//             _other_none => {
//                 self.diagnostics.push(Diagnostic { 
//                     severity: Severity::ERROR, 
//                     span: Some(binding_ir.span.clone().unwrap()), // weird, why am i asking for option here
//                     kind: DiagnosticKind::MissingBindingValue,
//                     message: format!(
//                         "assignment to `{}` is missing a value",
//                         binding_ir.id
//                     ),
//                 });
//                 Type::Unknown
//             }
//         };

//         flow_env.by_ref.insert(target_ref, FlowValue::Bound(value_type));
//     }

//     fn handle_annassign(&mut self, id: i64, binding_ir: &BindingIR, flow_env: &mut FlowEnv) {
//         let target_ref = SymbolRef {
//             program_id: id,
//             symbol_id: binding_ir.target_id
//         };

//         let expected_type = self.symbol_types.get(&target_ref);

//         let actual_type = match &binding_ir.value {
//             Some(value) => self.infer_expr_type(value),
//             _other_none => Type::Unknown,
//         };

//         if self.is_assignable(&actual_type, &expected_type) {
//             flow_env.by_ref.insert(target_ref, FlowValue::Bound(expected_type));
//         }
//     }

//     pub fn resolve_types(
//         &mut self
//     ) {
//         for (&id, program) in &self.programs.by_id {
//             let mut env = FlowEnv::new();
//             self.infer_statements(&program.body, &mut env, id);
//         }
//     }

//     pub fn infer_statements(
//         &mut self, 
//         stmts: &Vec<StmtIR>,
//         env: &mut FlowEnv,
//         program_id: i64,
//     ) {
//         for stmt in stmts {
//             self.infer_stmt(stmt, env, program_id);
//         }
//     }

//     fn infer_stmt(
//         &mut self,
//         stmt: &StmtIR,
//         env: &mut FlowEnv,
//         program_id: i64,
//     ) {
//         match stmt {
//             StmtIR::Binding(binding_ir) => {
//                 println!("entered Binding");
//                 println!("binding kind: {:?}", binding_ir.kind);

//                 match &binding_ir.kind {
//                     BindingKind::AnnAssign => {
//                         println!("ann assign");

//                         self.handle_annassign(program_id, &binding_ir, env)
//                     }

//                     BindingKind::Assign => {
//                         println!("plain assignment");

//                         self.handle_assign(program_id, &binding_ir)

//                     }

//                     BindingKind::Unknown => {
//                         println!("unknown binding");
//                     }
//                 }
//             }

//             StmtIR::ExprStmt(expr_stmt_ir) => {}

//             StmtIR::AugAssign(aug_assign_ir) => {},

//             StmtIR::If(if_stmt) => {
//                 let mut then_env = env.clone();
//                 self.infer_statements(&if_stmt.body, &mut then_env, program_id);

//                 let mut else_env = env.clone();
//                 self.infer_statements(&if_stmt.orelse, &mut else_env, program_id);

//                 // overwrite env
//                 *env = FlowEnv::merge(&then_env, &else_env);
//             },

//             StmtIR::WhileLoop(while_loop_ir) => {},

//             StmtIR::ForLoop(for_loop_ir) => {},

//             StmtIR::Function(function_ir) => {},

//             StmtIR::Class(class_ir) => {},

//             StmtIR::Import(import_ir) => {},

//             StmtIR::Return(return_ir) => {},
//         }
//     }            
// }
