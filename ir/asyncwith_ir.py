from dataclasses import dataclass

from common.span import SourceSpan
from generated import _pb2
from ir.stmt_ir import StmtIR
from ir.withitem_ir import WithItemIR


@dataclass
class AsyncWithIR(StmtIR):
    items: list[WithItemIR]
    body: list[StmtIR]
    type_comment: str | None
    span: SourceSpan

    def to_proto(self):
        return _pb2.StmtIR(
            async_with=_pb2.AsyncWithIR(
                items=[item.to_proto() for item in self.items],
                body=[stmt.to_proto() for stmt in self.body],
                type_comment=self.type_comment,
                span=self.span.to_proto(),
            )
        )
