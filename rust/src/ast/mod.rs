pub(crate) mod conversion;

pub mod excepthandler;
pub mod expr;
pub mod helper;
pub mod mod_;
pub mod operator;
pub mod pattern;
pub mod stmt;
pub mod type_param;

pub use conversion::ConversionError;

#[derive(Debug, Clone, PartialEq)]
pub enum ConstantValue {
    Integer(String),
    Float(f64),
    String(String),
    Bytes(Vec<u8>),
    Boolean(bool),
    Complex { real: f64, imag: f64 },
    None,
    Ellipsis,
}
