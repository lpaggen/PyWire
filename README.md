# Python 3.14 AST Serialization

This project serializes Python 3.14 abstract syntax trees without normalizing or
renaming Python AST nodes.

The transport pipeline is:

```text
CPython `ast` node
    -> same-named Python dataclass
    -> same-named Protocol Buffer message
    -> generated Prost wire struct
    -> independent same-named Rust AST struct
```

Concrete classes and their fields follow the
[official Python 3.14 abstract grammar](https://docs.python.org/3.14/library/ast.html#abstract-grammar)
exactly. Examples include:

- `Attribute(value, attr, ctx)`
- `Name(id, ctx)`
- `Assign(targets, value, type_comment)`
- `arguments(posonlyargs, args, vararg, kwonlyargs, kw_defaults, kwarg, defaults)`
- `MatchAs(pattern, name)`
- `Constant(value, kind)`

Expression and statement source attributes are preserved directly as
`lineno`, `col_offset`, `end_lineno`, and `end_col_offset`.

The publishable `pywire-python-ast` Rust crate is implemented in `rust`. Its
`ast` module contains one populated source module per official concrete node.
Every struct contains the official fields, and `PBDecoder` returns the owned
`ast::mod_::Mod` model rather than a protobuf alias.

`PBDecoder` recursively validates every decoded AST family, required child,
operator/context discriminator, parallel-list invariant, and `+` cardinality.
Errors include both the protobuf filename and the exact failing AST field path.

## Generate protobuf output

```bash
uv run python main.py example
```

## Validate

```bash
uv run python -m unittest discover
cargo test --manifest-path rust/Cargo.toml
cargo run --manifest-path rust/Cargo.toml --bin pywire-python-ast-decode -- ir_out
cargo run --manifest-path rust/Cargo.toml --bin pywire-python-ast-decode -- ir_out --parallel
cargo run --manifest-path rust/Cargo.toml --bin pywire-python-ast-decode -- ir_out --threads 4
cargo package --manifest-path rust/Cargo.toml
```

The parity tests compare the dataclass and protobuf field names against the
live Python 3.14 `ast` module.
