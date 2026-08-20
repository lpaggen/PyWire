use crate::{diagnostic::diagnostic::Diagnostic, ir::stmt_ir::StmtIR};




pub struct TypeResolverNew {
    pub diagnostics: Vec<Diagnostic>,
}

impl TypeResolverNew {
    fn new(body: Vec<StmtIR>, diagnostics: Vec<Diagnostic>) -> Self {
        Self {
            diagnostics,
        }
    }

    fn resolve_types(&mut self) {
        
    }
}