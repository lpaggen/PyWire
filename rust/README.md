# PyWire Python AST

`pywire-python-ast` provides owned Rust data structures matching the official
Python 3.14 abstract syntax tree and a recursive decoder for PyWire Protocol
Buffer messages.

The public AST under `pywire_python_ast::ast` is independent of Prost. The
generated protobuf messages are exposed separately under
`pywire_python_ast::pb` for callers that need in-memory conversion.

```rust
use pywire_python_ast::{ast::mod_::Mod, PBDecoder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let modules: Vec<Mod> = PBDecoder::new("ir_out")
        .with_threads(4)
        .decode_dir()?;

    println!("decoded {} modules", modules.len());
    Ok(())
}
```

Decode one file with `PBDecoder::decode_file`, or convert an existing generated
message with `PBDecoder::convert_mod` and the other `convert_*` functions.

The bundled command-line utility accepts a directory containing `.pb` files:

```bash
cargo run --bin pywire-python-ast-decode -- ir_out --parallel
```

The transport schema is bundled into the crate, so crates.io packages build
without access to the parent PyWire repository.
