use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use prost::Message;

use crate::ast::ConversionError;
use crate::ast::mod_::Mod;
use crate::pb;

pub struct PBDecoder {
    pub path: PathBuf,
    threads: usize,
}

impl PBDecoder {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            threads: 1,
        }
    }

    pub fn with_threads(mut self, threads: usize) -> Self {
        self.threads = threads.max(1);
        self
    }

    pub fn threads(&self) -> usize {
        self.threads
    }

    pub fn decode_dir(&self) -> io::Result<Vec<Mod>> {
        let mut paths = fs::read_dir(&self.path)?
            .map(|entry| entry.map(|entry| entry.path()))
            .collect::<Result<Vec<_>, _>>()?;
        paths.retain(|path| {
            path.is_file() && path.extension().is_some_and(|extension| extension == "pb")
        });
        paths.sort();

        if self.threads == 1 || paths.len() < 2 {
            return paths.iter().map(Self::decode_file).collect();
        }

        let worker_count = self.threads.min(paths.len());
        let chunk_size = paths.len().div_ceil(worker_count);

        std::thread::scope(|scope| {
            let handles = paths
                .chunks(chunk_size)
                .map(|chunk| {
                    scope.spawn(move || {
                        chunk
                            .iter()
                            .map(Self::decode_file)
                            .collect::<io::Result<Vec<_>>>()
                    })
                })
                .collect::<Vec<_>>();

            let mut modules = Vec::with_capacity(paths.len());
            for handle in handles {
                let mut chunk = handle
                    .join()
                    .map_err(|_| io::Error::other("protobuf decoder worker panicked"))??;
                modules.append(&mut chunk);
            }
            Ok(modules)
        })
    }

    pub fn decode_file(path: impl AsRef<Path>) -> io::Result<Mod> {
        let path = path.as_ref();
        let bytes = fs::read(path)?;
        let message = pb::ModNode::decode(bytes.as_slice()).map_err(|error| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "failed to decode protobuf file '{}': {error}",
                    path.display()
                ),
            )
        })?;

        Self::convert_mod(&message).map_err(|error| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "failed to translate protobuf AST '{}': {error}",
                    path.display()
                ),
            )
        })
    }

    pub fn convert_mod(message: &pb::ModNode) -> Result<Mod, ConversionError> {
        message.try_into()
    }

    #[allow(dead_code)]
    pub fn convert_stmt(message: &pb::Stmt) -> Result<crate::ast::stmt::Stmt, ConversionError> {
        message.try_into()
    }

    #[allow(dead_code)]
    pub fn convert_expr(message: &pb::ExprNode) -> Result<crate::ast::expr::Expr, ConversionError> {
        message.try_into()
    }

    #[allow(dead_code)]
    pub fn convert_pattern(
        message: &pb::Pattern,
    ) -> Result<crate::ast::pattern::Pattern, ConversionError> {
        message.try_into()
    }

    #[allow(dead_code)]
    pub fn convert_type_param(
        message: &pb::TypeParam,
    ) -> Result<crate::ast::type_param::TypeParam, ConversionError> {
        message.try_into()
    }

    #[allow(dead_code)]
    pub fn convert_except_handler(
        message: &pb::ExceptHandlerNode,
    ) -> Result<crate::ast::excepthandler::ExceptHandlerNode, ConversionError> {
        message.try_into()
    }
}

impl TryFrom<&crate::pb::Attribute> for crate::ast::expr::Attribute {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Attribute) -> Result<Self, Self::Error> {
        Ok(Self {
            value: crate::ast::conversion::required(
                value.value.as_ref(),
                "Attribute.value",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            attr: value.attr.clone(),
            ctx: crate::ast::conversion::required(
                value.ctx.as_ref(),
                "Attribute.ctx",
                |value: &crate::pb::ExprContext| value.try_into().map(std::convert::identity),
            )?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::Await> for crate::ast::expr::Await {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Await) -> Result<Self, Self::Error> {
        Ok(Self {
            value: crate::ast::conversion::required(
                value.value.as_ref(),
                "Await.value",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::BinOp> for crate::ast::expr::BinOp {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::BinOp) -> Result<Self, Self::Error> {
        Ok(Self {
            left: crate::ast::conversion::required(
                value.left.as_ref(),
                "BinOp.left",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            op: crate::ast::conversion::required(
                value.op.as_ref(),
                "BinOp.op",
                |value: &crate::pb::BinaryOperator| value.try_into().map(std::convert::identity),
            )?,
            right: crate::ast::conversion::required(
                value.right.as_ref(),
                "BinOp.right",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::BoolOp> for crate::ast::expr::BoolOp {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::BoolOp) -> Result<Self, Self::Error> {
        Ok(Self {
            op: crate::ast::conversion::required(
                value.op.as_ref(),
                "BoolOp.op",
                |value: &crate::pb::BoolOperator| value.try_into().map(std::convert::identity),
            )?,
            values: value
                .values
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::Call> for crate::ast::expr::Call {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Call) -> Result<Self, Self::Error> {
        Ok(Self {
            func: crate::ast::conversion::required(
                value.func.as_ref(),
                "Call.func",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            args: value
                .args
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            keywords: value
                .keywords
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::Compare> for crate::ast::expr::Compare {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Compare) -> Result<Self, Self::Error> {
        Ok(Self {
            left: crate::ast::conversion::required(
                value.left.as_ref(),
                "Compare.left",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            ops: value
                .ops
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            comparators: value
                .comparators
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::Constant> for crate::ast::expr::Constant {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Constant) -> Result<Self, Self::Error> {
        Ok(Self {
            value: crate::ast::conversion::required(
                value.value.as_ref(),
                "Constant.value",
                |value: &crate::pb::ConstantValue| value.try_into().map(std::convert::identity),
            )?,
            kind: value.kind.clone(),
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::Dict> for crate::ast::expr::Dict {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Dict) -> Result<Self, Self::Error> {
        Ok(Self {
            keys: value
                .keys
                .iter()
                .enumerate()
                .map(|(index, item)| {
                    crate::ast::conversion::optional(
                        item.value.as_ref(),
                        &format!("Dict.keys[{index}]"),
                        |value: &crate::pb::ExprNode| value.try_into(),
                    )
                })
                .collect::<Result<Vec<_>, _>>()?,
            values: value
                .values
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::DictComp> for crate::ast::expr::DictComp {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::DictComp) -> Result<Self, Self::Error> {
        Ok(Self {
            key: crate::ast::conversion::required(
                value.key.as_ref(),
                "DictComp.key",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            value: crate::ast::conversion::required(
                value.value.as_ref(),
                "DictComp.value",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            generators: value
                .generators
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::FormattedValue> for crate::ast::expr::FormattedValue {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::FormattedValue) -> Result<Self, Self::Error> {
        Ok(Self {
            value: crate::ast::conversion::required(
                value.value.as_ref(),
                "FormattedValue.value",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            conversion: value.conversion.clone(),
            format_spec: crate::ast::conversion::optional(
                value.format_spec.as_ref(),
                "FormattedValue.format_spec",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::GeneratorExp> for crate::ast::expr::GeneratorExp {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::GeneratorExp) -> Result<Self, Self::Error> {
        Ok(Self {
            elt: crate::ast::conversion::required(
                value.elt.as_ref(),
                "GeneratorExp.elt",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            generators: value
                .generators
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::IfExp> for crate::ast::expr::IfExp {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::IfExp) -> Result<Self, Self::Error> {
        Ok(Self {
            test: crate::ast::conversion::required(
                value.test.as_ref(),
                "IfExp.test",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            body: crate::ast::conversion::required(
                value.body.as_ref(),
                "IfExp.body",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            orelse: crate::ast::conversion::required(
                value.orelse.as_ref(),
                "IfExp.orelse",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::Interpolation> for crate::ast::expr::Interpolation {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Interpolation) -> Result<Self, Self::Error> {
        Ok(Self {
            value: crate::ast::conversion::required(
                value.value.as_ref(),
                "Interpolation.value",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            str: value.str.clone(),
            conversion: value.conversion.clone(),
            format_spec: crate::ast::conversion::optional(
                value.format_spec.as_ref(),
                "Interpolation.format_spec",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::JoinedStr> for crate::ast::expr::JoinedStr {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::JoinedStr) -> Result<Self, Self::Error> {
        Ok(Self {
            values: value
                .values
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::Lambda> for crate::ast::expr::Lambda {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Lambda) -> Result<Self, Self::Error> {
        Ok(Self {
            args: crate::ast::conversion::required(
                value.args.as_ref(),
                "Lambda.args",
                |value: &crate::pb::Arguments| value.try_into().map(std::convert::identity),
            )?,
            body: crate::ast::conversion::required(
                value.body.as_ref(),
                "Lambda.body",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::List> for crate::ast::expr::List {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::List) -> Result<Self, Self::Error> {
        Ok(Self {
            elts: value
                .elts
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            ctx: crate::ast::conversion::required(
                value.ctx.as_ref(),
                "List.ctx",
                |value: &crate::pb::ExprContext| value.try_into().map(std::convert::identity),
            )?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::ListComp> for crate::ast::expr::ListComp {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::ListComp) -> Result<Self, Self::Error> {
        Ok(Self {
            elt: crate::ast::conversion::required(
                value.elt.as_ref(),
                "ListComp.elt",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            generators: value
                .generators
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::Name> for crate::ast::expr::Name {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Name) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id.clone(),
            ctx: crate::ast::conversion::required(
                value.ctx.as_ref(),
                "Name.ctx",
                |value: &crate::pb::ExprContext| value.try_into().map(std::convert::identity),
            )?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::NamedExpr> for crate::ast::expr::NamedExpr {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::NamedExpr) -> Result<Self, Self::Error> {
        Ok(Self {
            target: crate::ast::conversion::required(
                value.target.as_ref(),
                "NamedExpr.target",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            value: crate::ast::conversion::required(
                value.value.as_ref(),
                "NamedExpr.value",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::Set> for crate::ast::expr::Set {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Set) -> Result<Self, Self::Error> {
        Ok(Self {
            elts: value
                .elts
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::SetComp> for crate::ast::expr::SetComp {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::SetComp) -> Result<Self, Self::Error> {
        Ok(Self {
            elt: crate::ast::conversion::required(
                value.elt.as_ref(),
                "SetComp.elt",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            generators: value
                .generators
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::Slice> for crate::ast::expr::Slice {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Slice) -> Result<Self, Self::Error> {
        Ok(Self {
            lower: crate::ast::conversion::optional(
                value.lower.as_ref(),
                "Slice.lower",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            upper: crate::ast::conversion::optional(
                value.upper.as_ref(),
                "Slice.upper",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            step: crate::ast::conversion::optional(
                value.step.as_ref(),
                "Slice.step",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::Starred> for crate::ast::expr::Starred {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Starred) -> Result<Self, Self::Error> {
        Ok(Self {
            value: crate::ast::conversion::required(
                value.value.as_ref(),
                "Starred.value",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            ctx: crate::ast::conversion::required(
                value.ctx.as_ref(),
                "Starred.ctx",
                |value: &crate::pb::ExprContext| value.try_into().map(std::convert::identity),
            )?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::Subscript> for crate::ast::expr::Subscript {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Subscript) -> Result<Self, Self::Error> {
        Ok(Self {
            value: crate::ast::conversion::required(
                value.value.as_ref(),
                "Subscript.value",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            slice: crate::ast::conversion::required(
                value.slice.as_ref(),
                "Subscript.slice",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            ctx: crate::ast::conversion::required(
                value.ctx.as_ref(),
                "Subscript.ctx",
                |value: &crate::pb::ExprContext| value.try_into().map(std::convert::identity),
            )?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::TemplateStr> for crate::ast::expr::TemplateStr {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::TemplateStr) -> Result<Self, Self::Error> {
        Ok(Self {
            values: value
                .values
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::Tuple> for crate::ast::expr::Tuple {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Tuple) -> Result<Self, Self::Error> {
        Ok(Self {
            elts: value
                .elts
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            ctx: crate::ast::conversion::required(
                value.ctx.as_ref(),
                "Tuple.ctx",
                |value: &crate::pb::ExprContext| value.try_into().map(std::convert::identity),
            )?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::UnaryOp> for crate::ast::expr::UnaryOp {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::UnaryOp) -> Result<Self, Self::Error> {
        Ok(Self {
            op: crate::ast::conversion::required(
                value.op.as_ref(),
                "UnaryOp.op",
                |value: &crate::pb::UnaryOperator| value.try_into().map(std::convert::identity),
            )?,
            operand: crate::ast::conversion::required(
                value.operand.as_ref(),
                "UnaryOp.operand",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::Yield> for crate::ast::expr::Yield {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Yield) -> Result<Self, Self::Error> {
        Ok(Self {
            value: crate::ast::conversion::optional(
                value.value.as_ref(),
                "Yield.value",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::YieldFrom> for crate::ast::expr::YieldFrom {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::YieldFrom) -> Result<Self, Self::Error> {
        Ok(Self {
            value: crate::ast::conversion::required(
                value.value.as_ref(),
                "YieldFrom.value",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::AnnAssign> for crate::ast::stmt::AnnAssign {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::AnnAssign) -> Result<Self, Self::Error> {
        Ok(Self {
            target: crate::ast::conversion::required(
                value.target.as_ref(),
                "AnnAssign.target",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            annotation: crate::ast::conversion::required(
                value.annotation.as_ref(),
                "AnnAssign.annotation",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            value: crate::ast::conversion::optional(
                value.value.as_ref(),
                "AnnAssign.value",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            simple: value.simple.clone(),
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::Assert> for crate::ast::stmt::Assert {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Assert) -> Result<Self, Self::Error> {
        Ok(Self {
            test: crate::ast::conversion::required(
                value.test.as_ref(),
                "Assert.test",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            msg: crate::ast::conversion::optional(
                value.msg.as_ref(),
                "Assert.msg",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::Assign> for crate::ast::stmt::Assign {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Assign) -> Result<Self, Self::Error> {
        Ok(Self {
            targets: value
                .targets
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            value: crate::ast::conversion::required(
                value.value.as_ref(),
                "Assign.value",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            type_comment: value.type_comment.clone(),
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::AsyncFor> for crate::ast::stmt::AsyncFor {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::AsyncFor) -> Result<Self, Self::Error> {
        Ok(Self {
            target: crate::ast::conversion::required(
                value.target.as_ref(),
                "AsyncFor.target",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            iter: crate::ast::conversion::required(
                value.iter.as_ref(),
                "AsyncFor.iter",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            body: value
                .body
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            orelse: value
                .orelse
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            type_comment: value.type_comment.clone(),
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::AsyncFunctionDef> for crate::ast::stmt::AsyncFunctionDef {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::AsyncFunctionDef) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name.clone(),
            args: crate::ast::conversion::required(
                value.args.as_ref(),
                "AsyncFunctionDef.args",
                |value: &crate::pb::Arguments| value.try_into().map(std::convert::identity),
            )?,
            body: value
                .body
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            decorator_list: value
                .decorator_list
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            returns: crate::ast::conversion::optional(
                value.returns.as_ref(),
                "AsyncFunctionDef.returns",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            type_comment: value.type_comment.clone(),
            type_params: value
                .type_params
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::AsyncWith> for crate::ast::stmt::AsyncWith {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::AsyncWith) -> Result<Self, Self::Error> {
        Ok(Self {
            items: value
                .items
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            body: value
                .body
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            type_comment: value.type_comment.clone(),
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::AugAssign> for crate::ast::stmt::AugAssign {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::AugAssign) -> Result<Self, Self::Error> {
        Ok(Self {
            target: crate::ast::conversion::required(
                value.target.as_ref(),
                "AugAssign.target",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            op: crate::ast::conversion::required(
                value.op.as_ref(),
                "AugAssign.op",
                |value: &crate::pb::BinaryOperator| value.try_into().map(std::convert::identity),
            )?,
            value: crate::ast::conversion::required(
                value.value.as_ref(),
                "AugAssign.value",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::Break> for crate::ast::stmt::Break {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Break) -> Result<Self, Self::Error> {
        Ok(Self {
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::ClassDef> for crate::ast::stmt::ClassDef {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::ClassDef) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name.clone(),
            bases: value
                .bases
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            keywords: value
                .keywords
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            body: value
                .body
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            decorator_list: value
                .decorator_list
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            type_params: value
                .type_params
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::Continue> for crate::ast::stmt::Continue {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Continue) -> Result<Self, Self::Error> {
        Ok(Self {
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::Delete> for crate::ast::stmt::Delete {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Delete) -> Result<Self, Self::Error> {
        Ok(Self {
            targets: value
                .targets
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::Expr> for crate::ast::stmt::Expr {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Expr) -> Result<Self, Self::Error> {
        Ok(Self {
            value: crate::ast::conversion::required(
                value.value.as_ref(),
                "Expr.value",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::For> for crate::ast::stmt::For {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::For) -> Result<Self, Self::Error> {
        Ok(Self {
            target: crate::ast::conversion::required(
                value.target.as_ref(),
                "For.target",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            iter: crate::ast::conversion::required(
                value.iter.as_ref(),
                "For.iter",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            body: value
                .body
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            orelse: value
                .orelse
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            type_comment: value.type_comment.clone(),
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::FunctionDef> for crate::ast::stmt::FunctionDef {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::FunctionDef) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name.clone(),
            args: crate::ast::conversion::required(
                value.args.as_ref(),
                "FunctionDef.args",
                |value: &crate::pb::Arguments| value.try_into().map(std::convert::identity),
            )?,
            body: value
                .body
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            decorator_list: value
                .decorator_list
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            returns: crate::ast::conversion::optional(
                value.returns.as_ref(),
                "FunctionDef.returns",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            type_comment: value.type_comment.clone(),
            type_params: value
                .type_params
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::Global> for crate::ast::stmt::Global {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Global) -> Result<Self, Self::Error> {
        Ok(Self {
            names: value.names.clone(),
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::If> for crate::ast::stmt::If {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::If) -> Result<Self, Self::Error> {
        Ok(Self {
            test: crate::ast::conversion::required(
                value.test.as_ref(),
                "If.test",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            body: value
                .body
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            orelse: value
                .orelse
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::Import> for crate::ast::stmt::Import {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Import) -> Result<Self, Self::Error> {
        Ok(Self {
            names: value
                .names
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::ImportFrom> for crate::ast::stmt::ImportFrom {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::ImportFrom) -> Result<Self, Self::Error> {
        Ok(Self {
            module: value.module.clone(),
            names: value
                .names
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            level: value.level.clone(),
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::Match> for crate::ast::stmt::Match {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Match) -> Result<Self, Self::Error> {
        Ok(Self {
            subject: crate::ast::conversion::required(
                value.subject.as_ref(),
                "Match.subject",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            cases: value
                .cases
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::Nonlocal> for crate::ast::stmt::Nonlocal {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Nonlocal) -> Result<Self, Self::Error> {
        Ok(Self {
            names: value.names.clone(),
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::Pass> for crate::ast::stmt::Pass {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Pass) -> Result<Self, Self::Error> {
        Ok(Self {
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::Raise> for crate::ast::stmt::Raise {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Raise) -> Result<Self, Self::Error> {
        Ok(Self {
            exc: crate::ast::conversion::optional(
                value.exc.as_ref(),
                "Raise.exc",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            cause: crate::ast::conversion::optional(
                value.cause.as_ref(),
                "Raise.cause",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::Return> for crate::ast::stmt::Return {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Return) -> Result<Self, Self::Error> {
        Ok(Self {
            value: crate::ast::conversion::optional(
                value.value.as_ref(),
                "Return.value",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::Try> for crate::ast::stmt::Try {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Try) -> Result<Self, Self::Error> {
        Ok(Self {
            body: value
                .body
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            handlers: value
                .handlers
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            orelse: value
                .orelse
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            finalbody: value
                .finalbody
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::TryStar> for crate::ast::stmt::TryStar {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::TryStar) -> Result<Self, Self::Error> {
        Ok(Self {
            body: value
                .body
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            handlers: value
                .handlers
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            orelse: value
                .orelse
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            finalbody: value
                .finalbody
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::TypeAlias> for crate::ast::stmt::TypeAlias {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::TypeAlias) -> Result<Self, Self::Error> {
        Ok(Self {
            name: crate::ast::conversion::required(
                value.name.as_ref(),
                "TypeAlias.name",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            type_params: value
                .type_params
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            value: crate::ast::conversion::required(
                value.value.as_ref(),
                "TypeAlias.value",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::While> for crate::ast::stmt::While {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::While) -> Result<Self, Self::Error> {
        Ok(Self {
            test: crate::ast::conversion::required(
                value.test.as_ref(),
                "While.test",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            body: value
                .body
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            orelse: value
                .orelse
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::With> for crate::ast::stmt::With {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::With) -> Result<Self, Self::Error> {
        Ok(Self {
            items: value
                .items
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            body: value
                .body
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            type_comment: value.type_comment.clone(),
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::MatchAs> for crate::ast::pattern::MatchAs {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::MatchAs) -> Result<Self, Self::Error> {
        Ok(Self {
            pattern: crate::ast::conversion::optional(
                value.pattern.as_ref(),
                "MatchAs.pattern",
                |value: &crate::pb::Pattern| value.try_into().map(Box::new),
            )?,
            name: value.name.clone(),
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::MatchClass> for crate::ast::pattern::MatchClass {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::MatchClass) -> Result<Self, Self::Error> {
        Ok(Self {
            cls: crate::ast::conversion::required(
                value.cls.as_ref(),
                "MatchClass.cls",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            patterns: value
                .patterns
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            kwd_attrs: value.kwd_attrs.clone(),
            kwd_patterns: value
                .kwd_patterns
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::MatchMapping> for crate::ast::pattern::MatchMapping {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::MatchMapping) -> Result<Self, Self::Error> {
        Ok(Self {
            keys: value
                .keys
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            patterns: value
                .patterns
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            rest: value.rest.clone(),
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::MatchOr> for crate::ast::pattern::MatchOr {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::MatchOr) -> Result<Self, Self::Error> {
        Ok(Self {
            patterns: value
                .patterns
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::MatchSequence> for crate::ast::pattern::MatchSequence {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::MatchSequence) -> Result<Self, Self::Error> {
        Ok(Self {
            patterns: value
                .patterns
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::MatchSingleton> for crate::ast::pattern::MatchSingleton {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::MatchSingleton) -> Result<Self, Self::Error> {
        Ok(Self {
            value: crate::ast::conversion::required(
                value.value.as_ref(),
                "MatchSingleton.value",
                |value: &crate::pb::ConstantValue| value.try_into().map(std::convert::identity),
            )?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::MatchStar> for crate::ast::pattern::MatchStar {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::MatchStar) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name.clone(),
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::MatchValue> for crate::ast::pattern::MatchValue {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::MatchValue) -> Result<Self, Self::Error> {
        Ok(Self {
            value: crate::ast::conversion::required(
                value.value.as_ref(),
                "MatchValue.value",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::ParamSpec> for crate::ast::type_param::ParamSpec {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::ParamSpec) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name.clone(),
            default_value: crate::ast::conversion::optional(
                value.default_value.as_ref(),
                "ParamSpec.default_value",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::TypeVar> for crate::ast::type_param::TypeVar {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::TypeVar) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name.clone(),
            bound: crate::ast::conversion::optional(
                value.bound.as_ref(),
                "TypeVar.bound",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            default_value: crate::ast::conversion::optional(
                value.default_value.as_ref(),
                "TypeVar.default_value",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::TypeVarTuple> for crate::ast::type_param::TypeVarTuple {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::TypeVarTuple) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name.clone(),
            default_value: crate::ast::conversion::optional(
                value.default_value.as_ref(),
                "TypeVarTuple.default_value",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::ExceptHandler> for crate::ast::excepthandler::ExceptHandler {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::ExceptHandler) -> Result<Self, Self::Error> {
        Ok(Self {
            r#type: crate::ast::conversion::optional(
                value.r#type.as_ref(),
                "ExceptHandler.type",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            name: value.name.clone(),
            body: value
                .body
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::Expression> for crate::ast::mod_::Expression {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Expression) -> Result<Self, Self::Error> {
        Ok(Self {
            body: crate::ast::conversion::required(
                value.body.as_ref(),
                "Expression.body",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
        })
    }
}

impl TryFrom<&crate::pb::FunctionType> for crate::ast::mod_::FunctionType {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::FunctionType) -> Result<Self, Self::Error> {
        Ok(Self {
            argtypes: value
                .argtypes
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            returns: crate::ast::conversion::required(
                value.returns.as_ref(),
                "FunctionType.returns",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
        })
    }
}

impl TryFrom<&crate::pb::Interactive> for crate::ast::mod_::Interactive {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Interactive) -> Result<Self, Self::Error> {
        Ok(Self {
            body: value
                .body
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl TryFrom<&crate::pb::Module> for crate::ast::mod_::Module {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Module) -> Result<Self, Self::Error> {
        Ok(Self {
            body: value
                .body
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            type_ignores: value
                .type_ignores
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl TryFrom<&crate::pb::Suite> for crate::ast::mod_::Suite {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::Suite) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::TypeIgnore> for crate::ast::mod_::TypeIgnore {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::TypeIgnore) -> Result<Self, Self::Error> {
        Ok(Self {
            lineno: value.lineno.clone(),
            tag: value.tag.clone(),
        })
    }
}

impl TryFrom<&crate::pb::Alias> for crate::ast::helper::Alias {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Alias) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name.clone(),
            asname: value.asname.clone(),
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::Arg> for crate::ast::helper::Arg {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Arg) -> Result<Self, Self::Error> {
        Ok(Self {
            arg: value.arg.clone(),
            annotation: crate::ast::conversion::optional(
                value.annotation.as_ref(),
                "Arg.annotation",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            type_comment: value.type_comment.clone(),
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::Arguments> for crate::ast::helper::Arguments {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Arguments) -> Result<Self, Self::Error> {
        Ok(Self {
            posonlyargs: value
                .posonlyargs
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            args: value
                .args
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            vararg: crate::ast::conversion::optional(
                value.vararg.as_ref(),
                "Arguments.vararg",
                |value: &crate::pb::Arg| value.try_into().map(std::convert::identity),
            )?,
            kwonlyargs: value
                .kwonlyargs
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            kw_defaults: value
                .kw_defaults
                .iter()
                .enumerate()
                .map(|(index, item)| {
                    crate::ast::conversion::optional(
                        item.value.as_ref(),
                        &format!("Arguments.kw_defaults[{index}]"),
                        |value: &crate::pb::ExprNode| value.try_into(),
                    )
                })
                .collect::<Result<Vec<_>, _>>()?,
            kwarg: crate::ast::conversion::optional(
                value.kwarg.as_ref(),
                "Arguments.kwarg",
                |value: &crate::pb::Arg| value.try_into().map(std::convert::identity),
            )?,
            defaults: value
                .defaults
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl TryFrom<&crate::pb::Comprehension> for crate::ast::helper::Comprehension {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Comprehension) -> Result<Self, Self::Error> {
        Ok(Self {
            target: crate::ast::conversion::required(
                value.target.as_ref(),
                "Comprehension.target",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            iter: crate::ast::conversion::required(
                value.iter.as_ref(),
                "Comprehension.iter",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            ifs: value
                .ifs
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            is_async: value.is_async.clone(),
        })
    }
}

impl TryFrom<&crate::pb::Keyword> for crate::ast::helper::Keyword {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Keyword) -> Result<Self, Self::Error> {
        Ok(Self {
            arg: value.arg.clone(),
            value: crate::ast::conversion::required(
                value.value.as_ref(),
                "Keyword.value",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            lineno: value.lineno,
            col_offset: value.col_offset,
            end_lineno: value.end_lineno,
            end_col_offset: value.end_col_offset,
        })
    }
}

impl TryFrom<&crate::pb::MatchCase> for crate::ast::helper::MatchCase {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::MatchCase) -> Result<Self, Self::Error> {
        Ok(Self {
            pattern: crate::ast::conversion::required(
                value.pattern.as_ref(),
                "MatchCase.pattern",
                |value: &crate::pb::Pattern| value.try_into().map(Box::new),
            )?,
            guard: crate::ast::conversion::optional(
                value.guard.as_ref(),
                "MatchCase.guard",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            body: value
                .body
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl TryFrom<&crate::pb::WithItem> for crate::ast::helper::WithItem {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::WithItem) -> Result<Self, Self::Error> {
        Ok(Self {
            context_expr: crate::ast::conversion::required(
                value.context_expr.as_ref(),
                "WithItem.context_expr",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
            optional_vars: crate::ast::conversion::optional(
                value.optional_vars.as_ref(),
                "WithItem.optional_vars",
                |value: &crate::pb::ExprNode| value.try_into().map(Box::new),
            )?,
        })
    }
}

impl TryFrom<&crate::pb::Add> for crate::ast::operator::Add {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::Add) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::And> for crate::ast::operator::And {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::And) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::BitAnd> for crate::ast::operator::BitAnd {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::BitAnd) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::BitOr> for crate::ast::operator::BitOr {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::BitOr) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::BitXor> for crate::ast::operator::BitXor {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::BitXor) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::Del> for crate::ast::operator::Del {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::Del) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::Div> for crate::ast::operator::Div {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::Div) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::Eq> for crate::ast::operator::Eq {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::Eq) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::FloorDiv> for crate::ast::operator::FloorDiv {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::FloorDiv) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::Gt> for crate::ast::operator::Gt {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::Gt) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::GtE> for crate::ast::operator::GtE {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::GtE) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::In> for crate::ast::operator::In {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::In) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::Invert> for crate::ast::operator::Invert {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::Invert) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::Is> for crate::ast::operator::Is {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::Is) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::IsNot> for crate::ast::operator::IsNot {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::IsNot) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::LShift> for crate::ast::operator::LShift {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::LShift) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::Load> for crate::ast::operator::Load {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::Load) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::Lt> for crate::ast::operator::Lt {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::Lt) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::LtE> for crate::ast::operator::LtE {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::LtE) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::MatMult> for crate::ast::operator::MatMult {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::MatMult) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::Mod> for crate::ast::operator::Mod {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::Mod) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::Mult> for crate::ast::operator::Mult {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::Mult) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::Not> for crate::ast::operator::Not {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::Not) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::NotEq> for crate::ast::operator::NotEq {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::NotEq) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::NotIn> for crate::ast::operator::NotIn {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::NotIn) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::Or> for crate::ast::operator::Or {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::Or) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::Pow> for crate::ast::operator::Pow {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::Pow) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::RShift> for crate::ast::operator::RShift {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::RShift) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::Store> for crate::ast::operator::Store {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::Store) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::Sub> for crate::ast::operator::Sub {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::Sub) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::UAdd> for crate::ast::operator::UAdd {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::UAdd) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::USub> for crate::ast::operator::USub {
    type Error = crate::ast::ConversionError;

    fn try_from(_value: &crate::pb::USub) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl TryFrom<&crate::pb::ExprNode> for crate::ast::expr::Expr {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::ExprNode) -> Result<Self, Self::Error> {
        use crate::pb::expr_node::Kind;

        match value.kind.as_ref() {
            Some(Kind::BoolOp(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::BoolOp| {
                    node.try_into().map(Box::new).map(Self::BoolOp)
                })
            }
            Some(Kind::NamedExpr(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::NamedExpr| {
                    node.try_into().map(Box::new).map(Self::NamedExpr)
                })
            }
            Some(Kind::BinOp(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::BinOp| {
                    node.try_into().map(Box::new).map(Self::BinOp)
                })
            }
            Some(Kind::UnaryOp(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::UnaryOp| {
                    node.try_into().map(Box::new).map(Self::UnaryOp)
                })
            }
            Some(Kind::Lambda(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Lambda| {
                    node.try_into().map(Box::new).map(Self::Lambda)
                })
            }
            Some(Kind::IfExp(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::IfExp| {
                    node.try_into().map(Box::new).map(Self::IfExp)
                })
            }
            Some(Kind::Dict(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Dict| {
                    node.try_into().map(Box::new).map(Self::Dict)
                })
            }
            Some(Kind::Set(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Set| {
                    node.try_into().map(Box::new).map(Self::Set)
                })
            }
            Some(Kind::ListComp(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::ListComp| {
                    node.try_into().map(Box::new).map(Self::ListComp)
                })
            }
            Some(Kind::SetComp(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::SetComp| {
                    node.try_into().map(Box::new).map(Self::SetComp)
                })
            }
            Some(Kind::DictComp(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::DictComp| {
                    node.try_into().map(Box::new).map(Self::DictComp)
                })
            }
            Some(Kind::GeneratorExp(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::GeneratorExp| {
                    node.try_into().map(Box::new).map(Self::GeneratorExp)
                })
            }
            Some(Kind::AwaitExpr(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Await| {
                    node.try_into().map(Box::new).map(Self::Await)
                })
            }
            Some(Kind::YieldExpr(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Yield| {
                    node.try_into().map(Box::new).map(Self::Yield)
                })
            }
            Some(Kind::YieldFrom(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::YieldFrom| {
                    node.try_into().map(Box::new).map(Self::YieldFrom)
                })
            }
            Some(Kind::Compare(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Compare| {
                    node.try_into().map(Box::new).map(Self::Compare)
                })
            }
            Some(Kind::Call(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Call| {
                    node.try_into().map(Box::new).map(Self::Call)
                })
            }
            Some(Kind::FormattedValue(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::FormattedValue| {
                    node.try_into().map(Box::new).map(Self::FormattedValue)
                })
            }
            Some(Kind::Interpolation(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Interpolation| {
                    node.try_into().map(Box::new).map(Self::Interpolation)
                })
            }
            Some(Kind::JoinedStr(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::JoinedStr| {
                    node.try_into().map(Box::new).map(Self::JoinedStr)
                })
            }
            Some(Kind::TemplateStr(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::TemplateStr| {
                    node.try_into().map(Box::new).map(Self::TemplateStr)
                })
            }
            Some(Kind::Constant(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Constant| {
                    node.try_into().map(Box::new).map(Self::Constant)
                })
            }
            Some(Kind::Attribute(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Attribute| {
                    node.try_into().map(Box::new).map(Self::Attribute)
                })
            }
            Some(Kind::Subscript(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Subscript| {
                    node.try_into().map(Box::new).map(Self::Subscript)
                })
            }
            Some(Kind::Starred(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Starred| {
                    node.try_into().map(Box::new).map(Self::Starred)
                })
            }
            Some(Kind::Name(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Name| {
                    node.try_into().map(Box::new).map(Self::Name)
                })
            }
            Some(Kind::List(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::List| {
                    node.try_into().map(Box::new).map(Self::List)
                })
            }
            Some(Kind::Tuple(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Tuple| {
                    node.try_into().map(Box::new).map(Self::Tuple)
                })
            }
            Some(Kind::Slice(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Slice| {
                    node.try_into().map(Box::new).map(Self::Slice)
                })
            }
            None => Err(crate::ast::ConversionError::new(
                "Expr",
                "missing concrete Expr",
            )),
        }
    }
}

impl TryFrom<&crate::pb::Stmt> for crate::ast::stmt::Stmt {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Stmt) -> Result<Self, Self::Error> {
        use crate::pb::stmt::Kind;

        match value.kind.as_ref() {
            Some(Kind::FunctionDef(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::FunctionDef| {
                    node.try_into().map(Box::new).map(Self::FunctionDef)
                })
            }
            Some(Kind::AsyncFunctionDef(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::AsyncFunctionDef| {
                    node.try_into().map(Box::new).map(Self::AsyncFunctionDef)
                })
            }
            Some(Kind::ClassDef(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::ClassDef| {
                    node.try_into().map(Box::new).map(Self::ClassDef)
                })
            }
            Some(Kind::ReturnStmt(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Return| {
                    node.try_into().map(Box::new).map(Self::Return)
                })
            }
            Some(Kind::DeleteStmt(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Delete| {
                    node.try_into().map(Box::new).map(Self::Delete)
                })
            }
            Some(Kind::Assign(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Assign| {
                    node.try_into().map(Box::new).map(Self::Assign)
                })
            }
            Some(Kind::TypeAlias(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::TypeAlias| {
                    node.try_into().map(Box::new).map(Self::TypeAlias)
                })
            }
            Some(Kind::AugAssign(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::AugAssign| {
                    node.try_into().map(Box::new).map(Self::AugAssign)
                })
            }
            Some(Kind::AnnAssign(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::AnnAssign| {
                    node.try_into().map(Box::new).map(Self::AnnAssign)
                })
            }
            Some(Kind::ForStmt(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::For| {
                    node.try_into().map(Box::new).map(Self::For)
                })
            }
            Some(Kind::AsyncFor(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::AsyncFor| {
                    node.try_into().map(Box::new).map(Self::AsyncFor)
                })
            }
            Some(Kind::WhileStmt(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::While| {
                    node.try_into().map(Box::new).map(Self::While)
                })
            }
            Some(Kind::IfStmt(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::If| {
                    node.try_into().map(Box::new).map(Self::If)
                })
            }
            Some(Kind::WithStmt(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::With| {
                    node.try_into().map(Box::new).map(Self::With)
                })
            }
            Some(Kind::AsyncWith(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::AsyncWith| {
                    node.try_into().map(Box::new).map(Self::AsyncWith)
                })
            }
            Some(Kind::MatchStmt(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Match| {
                    node.try_into().map(Box::new).map(Self::Match)
                })
            }
            Some(Kind::RaiseStmt(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Raise| {
                    node.try_into().map(Box::new).map(Self::Raise)
                })
            }
            Some(Kind::TryStmt(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Try| {
                    node.try_into().map(Box::new).map(Self::Try)
                })
            }
            Some(Kind::TryStar(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::TryStar| {
                    node.try_into().map(Box::new).map(Self::TryStar)
                })
            }
            Some(Kind::AssertStmt(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Assert| {
                    node.try_into().map(Box::new).map(Self::Assert)
                })
            }
            Some(Kind::ImportStmt(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Import| {
                    node.try_into().map(Box::new).map(Self::Import)
                })
            }
            Some(Kind::ImportFrom(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::ImportFrom| {
                    node.try_into().map(Box::new).map(Self::ImportFrom)
                })
            }
            Some(Kind::GlobalStmt(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Global| {
                    node.try_into().map(Box::new).map(Self::Global)
                })
            }
            Some(Kind::NonlocalStmt(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Nonlocal| {
                    node.try_into().map(Box::new).map(Self::Nonlocal)
                })
            }
            Some(Kind::ExprStmt(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Expr| {
                    node.try_into().map(Box::new).map(Self::Expr)
                })
            }
            Some(Kind::PassStmt(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Pass| {
                    node.try_into().map(Box::new).map(Self::Pass)
                })
            }
            Some(Kind::BreakStmt(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Break| {
                    node.try_into().map(Box::new).map(Self::Break)
                })
            }
            Some(Kind::ContinueStmt(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Continue| {
                    node.try_into().map(Box::new).map(Self::Continue)
                })
            }
            None => Err(crate::ast::ConversionError::new(
                "Stmt",
                "missing concrete Stmt",
            )),
        }
    }
}

impl TryFrom<&crate::pb::Pattern> for crate::ast::pattern::Pattern {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::Pattern) -> Result<Self, Self::Error> {
        use crate::pb::pattern::Kind;

        match value.kind.as_ref() {
            Some(Kind::MatchValue(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::MatchValue| {
                    node.try_into().map(Box::new).map(Self::MatchValue)
                })
            }
            Some(Kind::MatchSingleton(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::MatchSingleton| {
                    node.try_into().map(Box::new).map(Self::MatchSingleton)
                })
            }
            Some(Kind::MatchSequence(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::MatchSequence| {
                    node.try_into().map(Box::new).map(Self::MatchSequence)
                })
            }
            Some(Kind::MatchMapping(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::MatchMapping| {
                    node.try_into().map(Box::new).map(Self::MatchMapping)
                })
            }
            Some(Kind::MatchClass(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::MatchClass| {
                    node.try_into().map(Box::new).map(Self::MatchClass)
                })
            }
            Some(Kind::MatchStar(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::MatchStar| {
                    node.try_into().map(Box::new).map(Self::MatchStar)
                })
            }
            Some(Kind::MatchAs(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::MatchAs| {
                    node.try_into().map(Box::new).map(Self::MatchAs)
                })
            }
            Some(Kind::MatchOr(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::MatchOr| {
                    node.try_into().map(Box::new).map(Self::MatchOr)
                })
            }
            None => Err(crate::ast::ConversionError::new(
                "Pattern",
                "missing concrete Pattern",
            )),
        }
    }
}

impl TryFrom<&crate::pb::ModNode> for crate::ast::mod_::Mod {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::ModNode) -> Result<Self, Self::Error> {
        use crate::pb::mod_node::Kind;

        match value.kind.as_ref() {
            Some(Kind::Module(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Module| {
                    node.try_into()
                        .map(std::convert::identity)
                        .map(Self::Module)
                })
            }
            Some(Kind::Interactive(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Interactive| {
                    node.try_into()
                        .map(std::convert::identity)
                        .map(Self::Interactive)
                })
            }
            Some(Kind::Expression(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Expression| {
                    node.try_into()
                        .map(std::convert::identity)
                        .map(Self::Expression)
                })
            }
            Some(Kind::FunctionType(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::FunctionType| {
                    node.try_into()
                        .map(std::convert::identity)
                        .map(Self::FunctionType)
                })
            }
            Some(Kind::Suite(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::Suite| {
                    node.try_into().map(std::convert::identity).map(Self::Suite)
                })
            }
            None => Err(crate::ast::ConversionError::new(
                "Mod",
                "missing concrete Mod",
            )),
        }
    }
}

impl TryFrom<&crate::pb::TypeParam> for crate::ast::type_param::TypeParam {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::TypeParam) -> Result<Self, Self::Error> {
        use crate::pb::type_param::Kind;

        match value.kind.as_ref() {
            Some(Kind::TypeVar(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::TypeVar| {
                    node.try_into()
                        .map(std::convert::identity)
                        .map(Self::TypeVar)
                })
            }
            Some(Kind::ParamSpec(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::ParamSpec| {
                    node.try_into()
                        .map(std::convert::identity)
                        .map(Self::ParamSpec)
                })
            }
            Some(Kind::TypeVarTuple(node)) => {
                crate::ast::conversion::borrowed(node, |node: &crate::pb::TypeVarTuple| {
                    node.try_into()
                        .map(std::convert::identity)
                        .map(Self::TypeVarTuple)
                })
            }
            None => Err(crate::ast::ConversionError::new(
                "TypeParam",
                "missing concrete TypeParam",
            )),
        }
    }
}

impl TryFrom<&crate::pb::ExceptHandlerNode> for crate::ast::excepthandler::ExceptHandlerNode {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::ExceptHandlerNode) -> Result<Self, Self::Error> {
        match value.kind.as_ref() {
            Some(crate::pb::except_handler_node::Kind::ExceptHandler(node)) => {
                Ok(Self::ExceptHandler(Box::new(node.try_into()?)))
            }
            None => Err(crate::ast::ConversionError::new(
                "ExceptHandlerNode",
                "missing concrete exception handler",
            )),
        }
    }
}

impl TryFrom<&crate::pb::BoolOperator> for crate::ast::operator::BoolOp {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::BoolOperator) -> Result<Self, Self::Error> {
        use crate::pb::bool_operator::Kind;

        match value.kind.as_ref() {
            Some(Kind::AndOp(node)) => Ok(Self::And(node.try_into()?)),
            Some(Kind::OrOp(node)) => Ok(Self::Or(node.try_into()?)),
            None => Err(crate::ast::ConversionError::new(
                "BoolOp",
                "missing concrete operator",
            )),
        }
    }
}

impl TryFrom<&crate::pb::BinaryOperator> for crate::ast::operator::Operator {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::BinaryOperator) -> Result<Self, Self::Error> {
        use crate::pb::binary_operator::Kind;

        match value.kind.as_ref() {
            Some(Kind::Add(node)) => Ok(Self::Add(node.try_into()?)),
            Some(Kind::Sub(node)) => Ok(Self::Sub(node.try_into()?)),
            Some(Kind::Mult(node)) => Ok(Self::Mult(node.try_into()?)),
            Some(Kind::MatMult(node)) => Ok(Self::MatMult(node.try_into()?)),
            Some(Kind::Div(node)) => Ok(Self::Div(node.try_into()?)),
            Some(Kind::Mod(node)) => Ok(Self::Mod(node.try_into()?)),
            Some(Kind::Pow(node)) => Ok(Self::Pow(node.try_into()?)),
            Some(Kind::LShift(node)) => Ok(Self::LShift(node.try_into()?)),
            Some(Kind::RShift(node)) => Ok(Self::RShift(node.try_into()?)),
            Some(Kind::BitOr(node)) => Ok(Self::BitOr(node.try_into()?)),
            Some(Kind::BitXor(node)) => Ok(Self::BitXor(node.try_into()?)),
            Some(Kind::BitAnd(node)) => Ok(Self::BitAnd(node.try_into()?)),
            Some(Kind::FloorDiv(node)) => Ok(Self::FloorDiv(node.try_into()?)),
            None => Err(crate::ast::ConversionError::new(
                "Operator",
                "missing concrete operator",
            )),
        }
    }
}

impl TryFrom<&crate::pb::UnaryOperator> for crate::ast::operator::UnaryOperator {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::UnaryOperator) -> Result<Self, Self::Error> {
        use crate::pb::unary_operator::Kind;

        match value.kind.as_ref() {
            Some(Kind::Invert(node)) => Ok(Self::Invert(node.try_into()?)),
            Some(Kind::NotOp(node)) => Ok(Self::Not(node.try_into()?)),
            Some(Kind::UAdd(node)) => Ok(Self::UAdd(node.try_into()?)),
            Some(Kind::USub(node)) => Ok(Self::USub(node.try_into()?)),
            None => Err(crate::ast::ConversionError::new(
                "UnaryOperator",
                "missing concrete operator",
            )),
        }
    }
}

impl TryFrom<&crate::pb::ComparisonOperator> for crate::ast::operator::CmpOp {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::ComparisonOperator) -> Result<Self, Self::Error> {
        use crate::pb::comparison_operator::Kind;

        match value.kind.as_ref() {
            Some(Kind::Eq(node)) => Ok(Self::Eq(node.try_into()?)),
            Some(Kind::NotEq(node)) => Ok(Self::NotEq(node.try_into()?)),
            Some(Kind::Lt(node)) => Ok(Self::Lt(node.try_into()?)),
            Some(Kind::LtE(node)) => Ok(Self::LtE(node.try_into()?)),
            Some(Kind::Gt(node)) => Ok(Self::Gt(node.try_into()?)),
            Some(Kind::GtE(node)) => Ok(Self::GtE(node.try_into()?)),
            Some(Kind::IsOp(node)) => Ok(Self::Is(node.try_into()?)),
            Some(Kind::IsNot(node)) => Ok(Self::IsNot(node.try_into()?)),
            Some(Kind::InOp(node)) => Ok(Self::In(node.try_into()?)),
            Some(Kind::NotIn(node)) => Ok(Self::NotIn(node.try_into()?)),
            None => Err(crate::ast::ConversionError::new(
                "CmpOp",
                "missing concrete operator",
            )),
        }
    }
}

impl TryFrom<&crate::pb::ExprContext> for crate::ast::operator::ExprContext {
    type Error = crate::ast::ConversionError;

    fn try_from(value: &crate::pb::ExprContext) -> Result<Self, Self::Error> {
        use crate::pb::expr_context::Kind;

        match value.kind.as_ref() {
            Some(Kind::Load(node)) => Ok(Self::Load(node.try_into()?)),
            Some(Kind::Store(node)) => Ok(Self::Store(node.try_into()?)),
            Some(Kind::Del(node)) => Ok(Self::Del(node.try_into()?)),
            None => Err(crate::ast::ConversionError::new(
                "ExprContext",
                "missing concrete operator",
            )),
        }
    }
}

impl TryFrom<&crate::pb::ConstantValue> for crate::ast::ConstantValue {
    type Error = ConversionError;

    fn try_from(value: &crate::pb::ConstantValue) -> Result<Self, Self::Error> {
        use crate::pb::constant_value::Kind;

        match value.kind.as_ref() {
            Some(Kind::Integer(value)) => Ok(Self::Integer(value.clone())),
            Some(Kind::Float(value)) => Ok(Self::Float(*value)),
            Some(Kind::String(value)) => Ok(Self::String(value.clone())),
            Some(Kind::Bytes(value)) => Ok(Self::Bytes(value.clone())),
            Some(Kind::Boolean(value)) => Ok(Self::Boolean(*value)),
            Some(Kind::Complex(value)) => Ok(Self::Complex {
                real: value.real,
                imag: value.imag,
            }),
            Some(Kind::None(_)) => Ok(Self::None),
            Some(Kind::Ellipsis(_)) => Ok(Self::Ellipsis),
            None => Err(ConversionError::new(
                "ConstantValue",
                "missing concrete constant value",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn module_message() -> pb::ModNode {
        pb::ModNode {
            kind: Some(pb::mod_node::Kind::Module(pb::Module {
                body: vec![pb::Stmt {
                    kind: Some(pb::stmt::Kind::PassStmt(pb::Pass {
                        lineno: Some(1),
                        col_offset: Some(0),
                        end_lineno: Some(1),
                        end_col_offset: Some(4),
                    })),
                }],
                type_ignores: Vec::new(),
            })),
        }
    }

    fn temporary_directory() -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("pywire-python-ast-{}-{nonce}", std::process::id()))
    }

    fn write_module(path: &Path) {
        fs::write(path, module_message().encode_to_vec()).unwrap();
    }

    #[test]
    fn recursively_translates_a_message_into_owned_rust_ast() {
        let module = PBDecoder::convert_mod(&module_message()).unwrap();

        let Mod::Module(module) = module else {
            panic!("decoded AST is not a Rust Module");
        };
        assert_eq!(module.body.len(), 1);
    }

    #[test]
    fn parallel_decoding_matches_sequential_decoding() {
        let path = temporary_directory();
        fs::create_dir_all(&path).unwrap();
        write_module(&path.join("first.pb"));
        write_module(&path.join("second.pb"));

        let sequential = PBDecoder::new(&path).decode_dir().unwrap();
        let parallel = PBDecoder::new(&path).with_threads(4).decode_dir().unwrap();
        fs::remove_dir_all(path).unwrap();

        assert_eq!(parallel, sequential);
    }

    #[test]
    fn rejects_a_missing_module_variant() {
        let error = PBDecoder::convert_mod(&pb::ModNode { kind: None }).unwrap_err();
        assert_eq!(error.to_string(), "Mod: missing concrete Mod");
    }
}
