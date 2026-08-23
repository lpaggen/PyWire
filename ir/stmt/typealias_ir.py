from dataclasses import dataclass

from common.span import SourceSpan
from generated import _pb2
from ir.expr_ir import ExprIR
from ir.function_ir import ParamIR
from ir.stmt_ir import StmtIR


@dataclass
class TypeAliasIR(StmtIR):
    name: ExprIR
    type_params: list[ParamIR]
    value: ExprIR
    span: SourceSpan

    def to_proto(self):
        return _pb2.StmtIR(
            type_alias=_pb2.TypeAliasIR(
                name=self.name.to_proto(),
                type_params=[param.to_proto() for param in self.type_params],
                value=self.value.to_proto(),
                span=self.span.to_proto(),
            )
        )
