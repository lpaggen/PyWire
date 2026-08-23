from dataclasses import dataclass

from common.span import SourceSpan
from generated import _pb2
from ir.expr_ir import ExprIR
from ir.stmt_ir import StmtIR


@dataclass
class AsyncForIR(StmtIR):
    target: ExprIR
    iter: ExprIR
    body: list[StmtIR]
    orelse: list[StmtIR]
    type_comment: str | None
    span: SourceSpan

    def to_proto(self):
        return _pb2.StmtIR(
            async_for=_pb2.AsyncForIR(
                target=self.target.to_proto(),
                iter=self.iter.to_proto(),
                body=[stmt.to_proto() for stmt in self.body],
                orelse=[stmt.to_proto() for stmt in self.orelse],
                type_comment=self.type_comment,
                span=self.span.to_proto(),
            )
        )
