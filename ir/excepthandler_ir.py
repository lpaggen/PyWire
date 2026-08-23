from dataclasses import dataclass

from common.span import SourceSpan
from generated import _pb2
from ir.expr_ir import ExprIR
from ir.stmt_ir import StmtIR


@dataclass
class ExceptHandlerIR:
    type: ExprIR | None
    name: str | None
    body: list[StmtIR]
    span: SourceSpan

    def to_proto(self):
        return _pb2.ExceptHandlerIR(
            type=self.type.to_proto() if self.type is not None else None,
            name=self.name,
            body=[stmt.to_proto() for stmt in self.body],
            span=self.span.to_proto(),
        )
