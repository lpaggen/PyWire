use crate::diagnostic::diagnostic::Diagnostic;
use crate::ir::nodes::program_ir::ProgramIR;
use crate::linker::global_scope_table::GlobalSymbolTable;
use crate::linker::import_graph::ImportGraph;
use crate::linker::program_table::ProgramTable;
use crate::linker::resolution_table::ResolutionTable;
use crate::type_resolver::symbol_type_table::SymbolTypeTable;
use crate::type_resolver::type_resolver::TypeResolver;
use crate::pb_decoder::pb_decoder::PBDecoder;


mod linker;
mod pb_decoder;
mod diagnostic;
mod types;
mod ir;
mod type_resolver;

pub mod pb {
    include!(concat!(env!("OUT_DIR"), "/pdc.ir.rs"));
}

fn main() -> Result<(), Vec<Diagnostic>> {

    let decoder: PBDecoder = PBDecoder::new("../ir_out/");

    let programs: Vec<ProgramIR> = match decoder.decode_dir() {
        Ok(programs) => programs,
        Err(err) => panic!("{}", err), // should not panic since it depends on Python frontend
    };

    let mut table: ProgramTable = ProgramTable::new();
    table.build_tables(programs);

    let mut symbols = GlobalSymbolTable::new();
    symbols.build(&table);

    let mut graph: ImportGraph = ImportGraph::new();
    graph.build(&table);

    let mut resolved: ResolutionTable = ResolutionTable::new();
    resolved.resolve_imports(&table, &symbols);

    let mut types: SymbolTypeTable = SymbolTypeTable::new();
    types.build(&table, &symbols, &resolved)?;

    for (symbol_ref, symbol_type) in &types.by_ref {
        println!("{:?}, {:?}", symbol_ref, symbol_type)
    }

    // let resolver: TypeResolver<'_> = TypeResolver::new(&resolved, &types);
    // resolver.infer_program_types();

    // println!("{:?}", graph.tarjan_scc());

    Ok(())

}
