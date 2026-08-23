use crate::ir::nodes::program_ir::ProgramIR;
use crate::pb_decoder::pb_decoder::PBDecoder;

mod ir;
mod pb_decoder;

pub mod pb {
    include!(concat!(env!("OUT_DIR"), "/pdc.ir.rs"));
}

fn main() -> Result<(), Vec<Diagnostic>> {
    let decoder: PBDecoder = PBDecoder::new("../ir_out/");

    let programs: Vec<ProgramIR> = match decoder.decode_dir() {
        Ok(programs) => programs,
        Err(err) => panic!("{}", err), // should not panic since it depends on Python frontend
    };

    Ok(())
}
