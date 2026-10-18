fn main() {
    println!("cargo:rerun-if-changed=proto/python_ast.proto");
    prost_build::compile_protos(&["proto/python_ast.proto"], &["proto"]).unwrap();
}
