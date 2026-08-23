use crate::ir::expr_ir::ExprIR;
use crate::ir::span_ir::SourceSpan;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Conversion {
    None = -1,
    Str,
    Repr,
    Ascii,
}

impl TryFrom<i32> for Conversion {
    type Error = String;

    fn try_from(int: i32) -> Result<Self, Self::Error> {
        match int {
            -1 => Ok(Conversion::None),
            97 => Ok(Conversion::Ascii),
            114 => Ok(Conversion::Str),
            115 => Ok(Conversion::Repr),
            _ => Err(format!("invalid Conversion value: {}", int)),
        }
    }
}

// #[derive(Debug, Clone)]
// pub enum JoinedStrValueIR {
//     FormattedValue(FormattedValueIR),
//     Constant(ConstantIR),
// }

#[derive(Debug, Clone)]
pub struct JoinedStrIR {
    pub values: Vec<ExprIR>,  // this can only be Constant or FormattedValue, both are ExprIR
    pub span: Option<SourceSpan>,
}

#[derive(Debug, Clone)]
pub struct FormattedValueIR {
    pub value: Box<ExprIR>,
    pub conversion: Conversion,
    pub format_spec: Option<JoinedStrIR>,
    pub span: Option<SourceSpan>,
}
